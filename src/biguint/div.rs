use crate::biguint::BigUInt;
use crate::biguint::mul::MulTo;
use crate::util::u64s_to_u128;
use crate::SetVal;

impl BigUInt {
	/// Calculates the quotient and remainder
	/// Preconditions:
	///     * d != 0
	///     * n >= d
	/// Puts quotient in q and remainder in r
	/// n and d are mutable for implementation reasons, they are restored to original values before return.
	/// Uses long division, internally uses Knuth's Algorithm D
	pub fn div_rem_to_unchecked(
		n: &mut BigUInt,
		d: &mut BigUInt,
		q: &mut BigUInt,
		r: &mut BigUInt,
	) {
		debug_assert!(*d != 0, "division by zero");
		debug_assert!(n >= d, "can't divide smaller number by bigger number");

		// Keep track by how much we shifted to backshift r, n and d later
		let normalization_bitshift = d.data[d.len() - 1].leading_zeros();
		// Normalize
		*n <<= normalization_bitshift;
		*d <<= normalization_bitshift;

		Self::div_rem_to_normalized(n, d, q, r);

		// Shift r, n and d back
		*r >>= normalization_bitshift;
		*n >>= normalization_bitshift;
		*d >>= normalization_bitshift;
	}

	fn div_rem_to_normalized(n: &BigUInt, d: &BigUInt, q: &mut BigUInt, r: &mut BigUInt) {
		// Intermediate numerator
		let mut n_inter = n.clone();
		// Make it the same len as d
		n_inter >>= (n.len() - d.len()) as u32 * u64::BITS;
		// We need it to be one digit longer than d, so we put a leading zero
		n_inter.data.push(0u64);
		// Clear q
		q.set_zero();
		// We go backwards from the most significant remaining digits of n
		let iter = n.data[0..(n.len() - d.len())].iter().rev();
		for &x in iter {
			// Preconditions:
			//     * Length:
			//         -- initially, ensured above
			//         -- during iteration, ensured below
			//     * d != 0: is ensured by caller
			//     * Overflow:
			//         -- initially we take d.len() digits and pad with high 0,
			//                so effectively dividing same digit numbers, no overflow
			//         -- rest is ensured by properties of division
			//     * Normalization: done above
			let q_i = div_n_plus_1_digits_normalized(&n_inter, d, r);
			// Assemble q in reverse order, reverse it back later
			q.data.push(q_i);
			// Remainder becomes new numerator
			n_inter.set_val(&*r);
			// Bring down another digit
			n_inter.data.insert(0, x);

			// Pad d with zeros until correct len
			while n_inter.len() <= d.len() {
				n_inter.data.push(0u64);
			}
		}
		let q_i = div_n_plus_1_digits_normalized(&n_inter, d, r);
		q.data.push(q_i);

		// Unfuck q
		q.data.reverse();
		q.truncate_leading_zeros();
	}

	pub fn div_rem_to(n: &mut BigUInt, d: &mut BigUInt, q: &mut BigUInt, r: &mut BigUInt) {
		assert_ne!(*d, 0, "division by zero");
		if n < d {
			q.set_zero();
			r.set_val(&*n);
		} else {
			Self::div_rem_to_unchecked(n, d, q, r);
		}
	}

	pub fn div_rem(&mut self, d: &mut BigUInt) -> (BigUInt, BigUInt) {
		let mut q = Self::ZERO;
		let mut r = Self::ZERO;
		Self::div_rem_to(self, d, &mut q, &mut r);
		(q, r)
	}
}

/// Divides (n+1)-digit numerator by n-digit denominator.
/// Preconditions:
///     * Length: n.len() == d.len() + 1
///     * d != 0
///     * Overflow: quotient must fit in one digit
///     * Normalization: denominator's leading digit has to be >= 2^64 / 2
/// Returns the quotient, puts remainder in r
/// Uses Knuth's Algorithm D, as described in https://ridiculousfish.com/blog/posts/labor-of-division-episode-iv.html
fn div_n_plus_1_digits_normalized(n: &BigUInt, d: &BigUInt, r: &mut BigUInt) -> u64 {
	debug_assert_eq!(
		n.len(),
		d.len() + 1,
		"numerator is not 1 digit longer than the denominator"
	);
	debug_assert_ne!(*d, 0, "division by zero");
	debug_assert!(d[d.len() - 1] >= (1 << 63), "denominator is not normalized");

	let n_hi = [n[n.len() - 2], n[n.len() - 1]];
	let d_hi = d[d.len() - 1];
	let q_est = div_2_digits(n_hi, d_hi);
	let mut q = q_est;
	// q_est is always an overestimate, and at most by 2 because inputs are normalized
	for _ in 0..=2 {
		// Use r's buffer to temporarily hold q * d
		r.mul_to(d, &BigUInt::from(q));
		if *r <= *n {
			break;
		} else {
			q -= 1;
		}
	}
	// Put the remainder into r
	r.checked_sub_from_assign(n);
	q
}

/// Divides 2-digit numerator by 1-digit denominator.
/// Result must fit in one digit
/// Returns the quotient
fn div_2_digits(n: [u64; 2], d: u64) -> u64 {
	let n_wide = u64s_to_u128(n);
	let d_wide = d as u128;
	let q_wide = n_wide / d_wide;
	debug_assert!(
		q_wide <= (u64::MAX as u128),
		"quotient does not fit in 1 digit"
	);
	q_wide as u64
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::{from_foreign_biguint, to_foreign_biguint};

	#[test]
	fn test_div_n_plus_1_digits_normalized() {
		let d = BigUInt::from(0xff00000000000000u64);
		let n = BigUInt::from_vec_le(vec![1, 0x000000000000000fu64]);
		div_n_plus_1_digits_normalized_helper(n, d);

		let d = BigUInt::from(0xff00000004565000u64);
		let n = BigUInt::from_vec_le(vec![0xff, 0x0000000000000546fu64]);
		div_n_plus_1_digits_normalized_helper(n, d);

		let d = BigUInt::from(0xf000000456540000u64);
		let n = BigUInt::from_vec_le(vec![0xf12f, 0x0000000000005345fu64]);
		div_n_plus_1_digits_normalized_helper(n, d);

		let d = BigUInt::from_vec_le(vec![5426457365867876856, 0xff00000000000000u64]);
		let n = BigUInt::from_vec_le(vec![43543624643643765, 0xf12f, 0x000000000000ea50fu64]);
		div_n_plus_1_digits_normalized_helper(n, d);
	}

	fn div_n_plus_1_digits_normalized_helper(n: BigUInt, d: BigUInt) {
		let mut r_n = BigUInt::ZERO;
		let q_n = div_n_plus_1_digits_normalized(&n, &d, &mut r_n);
		assert_eq!(q_n * &d + &r_n, n);
		let n_f = to_foreign_biguint(n);
		let d_f = to_foreign_biguint(d);
		let q_f = from_foreign_biguint(&n_f / &d_f);
		let r_f = from_foreign_biguint(&n_f % &d_f);
		assert_eq!(q_n, q_f);
		assert_eq!(r_n, r_f);
	}

	#[test]
	fn test_div_rem() {
		div_rem_helper(
			BigUInt::from_vec_le(vec![
				6848468468486468486,
				6851351684844315148,
				87951463548843415,
				6848464568135153,
			]),
			BigUInt::from_vec_le(vec![
				486468153601531,
				484684416531315,
				468431513584864,
				84686484684864,
			]),
		);

		div_rem_helper(
			BigUInt::from_vec_le(vec![
				u64::MAX - 10,
				u64::MAX,
				u64::MAX - 1,
				u64::MAX - 4564564,
			]),
			BigUInt::from_vec_le(vec![u64::MAX - 1, u64::MAX - 156456, u64::MAX, u64::MAX]),
		);

		div_rem_helper(
			BigUInt::from_vec_le(vec![
				u64::MAX - 10,
				u64::MAX,
				u64::MAX - 1,
				u64::MAX - 4564564,
			]),
			BigUInt::from(25345u64),
		);

		div_rem_helper(BigUInt::from(u128::MAX), BigUInt::from(u128::MAX));
	}

	fn div_rem_helper(mut n: BigUInt, mut d: BigUInt) {
		let n_c = n.clone();
		let d_c = d.clone();
		let (q_n, r_n) = n.div_rem(&mut d);
		assert_eq!(n, n_c);
		assert_eq!(d, d_c);
		let n_f = to_foreign_biguint(n);
		let d_f = to_foreign_biguint(d);
		let q_f = from_foreign_biguint(&n_f / &d_f);
		let r_f = from_foreign_biguint(&n_f % &d_f);
		assert_eq!(q_n, q_f);
		assert_eq!(r_n, r_f);
	}
}
