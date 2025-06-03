use core::ops::{Div, Rem};

use crate::biguint::mul::MulTo;
use crate::biguint::BigUInt;
use crate::error::TryFromIntError;
use crate::util::u64s_to_u128;
use crate::SetVal;

pub trait DivRem<RHS = Self> {
	type Output;
	fn div_rem(self, d: RHS) -> (Self::Output, Self::Output);
	fn div_rem_to(self, d: RHS, q: &mut Self::Output, r: &mut Self::Output);
}

impl DivRem for &mut BigUInt {
	type Output = BigUInt;

	fn div_rem(self, d: &mut BigUInt) -> (BigUInt, BigUInt) {
		let mut q = BigUInt::ZERO;
		let mut r = BigUInt::ZERO;
		self.div_rem_to(d, &mut q, &mut r);
		(q, r)
	}

	fn div_rem_to(self, d: &mut BigUInt, q: &mut BigUInt, r: &mut BigUInt) {
		assert_ne!(*d, 0u64, "division by zero");
		if self < d {
			q.set_zero();
			r.set_val(&*self);
		} else if *d == 1u64 {
			q.set_val(&*self);
			r.set_zero();
		} else {
			BigUInt::div_rem_to_unchecked(self, d, q, r);
		}
	}
}

macro_rules! impl_div_rem_u {
	($($t:ty),*) => {$(
		impl DivRem<$t> for &mut BigUInt {
			type Output = BigUInt;
			fn div_rem(self, d: $t) -> (BigUInt, BigUInt) {
				let mut q = BigUInt::ZERO;
				let mut r = BigUInt::ZERO;
				self.div_rem_to(d, &mut q, &mut r);
				(q, r)
			}

			fn div_rem_to(self, d: $t, q: &mut BigUInt, r: &mut BigUInt) {
				self.div_rem_to(&mut BigUInt::from(d), q, r);
			}
		}

		impl DivRem<&BigUInt> for $t {
			type Output = BigUInt;
			fn div_rem(self, d: &BigUInt) -> (BigUInt, BigUInt) {
				let mut q = BigUInt::ZERO;
				let mut r = BigUInt::ZERO;
				self.div_rem_to(d, &mut q, &mut r);
				(q, r)
			}

			fn div_rem_to(self, d: &BigUInt, q: &mut BigUInt, r: &mut BigUInt) {
				let n: u128 = self as u128;
				if let Ok(d) = TryInto::<u128>::try_into(d) {
					*q = BigUInt::from(n / d);
					*r = BigUInt::from(n / d);
				} else {
					q.set_zero();
					r.set_val(self);
				}
			}
		}
	)*};
}

impl_div_rem_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_div_rem_i {
	($($t:ty),*) => {$(
		impl DivRem<$t> for &mut BigUInt {
			type Output = BigUInt;
			fn div_rem(self, d: $t) -> (BigUInt, BigUInt) {
				let mut q = BigUInt::ZERO;
				let mut r = BigUInt::ZERO;
				self.div_rem_to(d, &mut q, &mut r);
				(q, r)
			}

			fn div_rem_to(self, d: $t, q: &mut BigUInt, r: &mut BigUInt) {
				self.div_rem_to(&mut BigUInt::try_from(d).unwrap(), q, r);
			}
		}

		impl DivRem<&BigUInt> for $t {
			type Output = BigUInt;
			fn div_rem(self, d: &BigUInt) -> (BigUInt, BigUInt) {
				let mut q = BigUInt::ZERO;
				let mut r = BigUInt::ZERO;
				self.div_rem_to(d, &mut q, &mut r);
				(q, r)
			}

			fn div_rem_to(self, d: &BigUInt, q: &mut BigUInt, r: &mut BigUInt) {
				let n: u128 = self.try_into().map_err(|_| TryFromIntError).unwrap();
				if let Ok(d) = TryInto::<u128>::try_into(d) {
					*q = BigUInt::from(n / d);
					*r = BigUInt::from(n / d);
				} else {
					q.set_zero();
					r.set_val(n);
				}
			}
		}
	)*};
}

impl_div_rem_i! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_div_and_rem {
	($(($t1:ty | $t2:ty)),*$(,)?) => {$(
		impl Div<$t2> for $t1 {
			type Output = BigUInt;
			fn div(self, rhs: $t2) -> BigUInt {
				self.div_rem(rhs).0
			}
		}

		impl Rem<$t2> for $t1 {
			type Output = BigUInt;
			fn rem(self, rhs: $t2) -> BigUInt {
				self.div_rem(rhs).1
			}
		}
	)*};
}

impl_div_and_rem! {
	(&mut BigUInt | &mut BigUInt),
	(&mut BigUInt | u8),
	(&mut BigUInt | u16),
	(&mut BigUInt | u32),
	(&mut BigUInt | u64),
	(&mut BigUInt | u128),
	(&mut BigUInt | usize),
	(&mut BigUInt | i8),
	(&mut BigUInt | i16),
	(&mut BigUInt | i32),
	(&mut BigUInt | i64),
	(&mut BigUInt | i128),
	(&mut BigUInt | isize),
	(u8           | &BigUInt),
	(u16          | &BigUInt),
	(u32          | &BigUInt),
	(u64          | &BigUInt),
	(u128         | &BigUInt),
	(usize        | &BigUInt),
	(i8           | &BigUInt),
	(i16          | &BigUInt),
	(i32          | &BigUInt),
	(i64          | &BigUInt),
	(i128         | &BigUInt),
	(isize        | &BigUInt),
}

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

	/// Calculates the quotient and remainder
	/// Preconditions:
	///     * d != 0
	///     * n >= d
	///     * Normalization: denominator's leading digit has to be >= 2^64 / 2
	/// Puts quotient in q and remainder in r
	/// n and d are mutable for implementation reasons, they are restored to original values before return.
	/// Uses long division, internally uses Knuth's Algorithm D
	fn div_rem_to_normalized(n: &BigUInt, d: &BigUInt, q: &mut BigUInt, r: &mut BigUInt) {
		// Intermediate numerator
		let mut n_inter = n.clone();
		// Make it the same len as d
		n_inter.shr_digits(n.len() - d.len());
		// Clear q
		q.set_zero();
		// Start from the most significant remaining digits of n
		let iter = n.data[0..(n.len() - d.len())].iter().rev();
		for &x in iter {
			// If n_inter is guaranteed smaller than d, write a zero into q and
			// continue with one more digit
			if n_inter.len() < d.len() {
				n_inter.data.insert(0, x);
				q.data.push(0u64);
				continue;
			}
			// If still too short, pad with a zero
			if n_inter.len() == d.len() {
				n_inter.data.push(0u64);
			}

			// Preconditions:
			//     * Length: ensured above
			//     * d != 0: is ensured by caller
			//     * Overflow:
			//         -- initially we take d.len() digits and pad with high 0,
			//                so effectively dividing same digit numbers, no overflow
			//         -- rest is ensured by properties of division
			//     * Normalization: guaranteed by caller
			let q_i = div_n_plus_1_digits_normalized(&n_inter, d, r);
			// Assemble q in reverse order, reverse it back later
			q.data.push(q_i);
			// Remainder becomes new numerator
			n_inter.set_val(&*r);
			// Bring down another digit
			n_inter.data.insert(0, x);
		}
		if n_inter.len() == d.len() {
			n_inter.data.push(0u64);
		}
		let q_i = div_n_plus_1_digits_normalized(&n_inter, d, r);
		q.data.push(q_i);

		// Unfuck q
		q.data.reverse();
		q.truncate_leading_zeros();
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
	fn correct_q(q_est: &mut u64, r: &mut BigUInt, n: &BigUInt, d: &BigUInt) {
		// Use r's buffer to hold q_est * d
		r.mul_to(d, &BigUInt::from(*q_est));

		// q_est is always an overestimate, and at most by 2 because inputs are normalized
		for _ in 0..2 {
			// Estimate is right
			if *r <= *n {
				break;
			}
			// Correct by one
			*q_est -= 1;
			*r -= d;
		}
		// Put the remainder into r
		r.checked_sub_from_assign(n);
	}

	debug_assert_eq!(
		n.len(),
		d.len() + 1,
		"numerator is not 1 digit longer than the denominator"
	);
	debug_assert_ne!(*d, 0, "division by zero");
	debug_assert!(d[d.len() - 1] >= (1 << 63), "denominator is not normalized");

	let n_hi = [n[n.len() - 2], n[n.len() - 1]];
	let d_hi = d[d.len() - 1];
	let mut q_est = div_2_digits(n_hi, d_hi);

	correct_q(&mut q_est, r, n, d);

	q_est
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
