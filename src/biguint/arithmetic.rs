use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::biguint::BigUInt;
use crate::SetVal;
use crate::util::{VecExt, carrying_mul, u64s_to_u128};

pub trait Arithmetic<T> {
	fn checked_sub(self, rhs: T) -> Option<BigUInt>;
}

impl Arithmetic<&BigUInt> for BigUInt {
	fn checked_sub(mut self, rhs: &BigUInt) -> Option<BigUInt> {
		let succ = self.checked_sub_assign(rhs);
		if succ { Some(self) } else { None }
	}
}

macro_rules! impl_arithmetic_u {
	($($t:ty),*) => {
		$(impl Arithmetic<$t> for BigUInt
		{
			fn checked_sub(self, rhs: $t) -> Option<BigUInt> {
				self.checked_sub(&BigUInt::from(rhs))
			}
		})*
	}
}

impl_arithmetic_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_arithmetic_i {
	($($t:ty),*) => {
		$(impl Arithmetic<$t> for BigUInt
		{
			fn checked_sub(self, rhs: $t) -> Option<BigUInt> {
				let abs = BigUInt::from(rhs.unsigned_abs());
				if rhs >= 0 {
					self.checked_sub(&abs)
				} else {
					Some(self + &abs)
				}
			}
		})*
	}
}

impl_arithmetic_i! { i8, i16, i32, i64, i128, isize }

impl BigUInt {
	/// Calculates lhs * rhs, saves result into self
	/// Unlike self = &lhs * &rhs, avoids allocations if possible
	pub fn mul_to(&mut self, lhs: &BigUInt, rhs: &BigUInt) {
		let new_len = lhs.len() + rhs.len();
		self.data.set_len_fill_zero(new_len);
		for (i, &a_i) in lhs.data.iter().enumerate() {
			let mut carry = 0u64;
			for (j, &b_j) in rhs.data.iter().enumerate() {
				let (lo, hi) = carrying_mul(a_i, b_j);
				let (sum1, carry1) = lo.overflowing_add(carry);
				let (sum2, carry2) = self[i + j].overflowing_add(sum1);
				self.data[i + j] = sum2;
				// Can't overflow because magic
				// u64 * u64 + u64 + u64 fits in 2 u64s
				carry = hi + carry1 as u64 + carry2 as u64;
			}
			self.data[i + rhs.len()] = carry;
		}
		self.truncate_leading_zeros();
	}

	/// Calculates self - lhs, saves result into self
	/// Returns false and leaves garbage in self on overflow.
	fn checked_sub_assign(&mut self, rhs: &Self) -> bool {
		let mut borrow = 0u64;
		let len = if self.len() >= rhs.len() {
			self.len()
		} else {
			return false;
		};
		for i in 0..len {
			let a = self.data[i];
			let b = rhs.data.get_or_default(i);
			let (diff1, borrow1) = a.overflowing_sub(b);
			let (diff2, borrow2) = diff1.overflowing_sub(borrow);
			self.data.set_or_insert(i, diff2);
			borrow = borrow1 as u64 + borrow2 as u64;
		}
		self.truncate_leading_zeros();
		borrow == 0
	}

	/// Calculates lhs - self, saves result into self
	/// Returns false and leaves garbage in self on overflow.
	fn checked_sub_from_assign(&mut self, lhs: &Self) -> bool {
		let mut borrow = 0u64;
		let len = if lhs.len() >= self.len() {
			lhs.len()
		} else {
			return false;
		};
		for i in 0..len {
			let a = lhs.data[i];
			let b = self.data.get_or_default(i);
			let (diff1, borrow1) = a.overflowing_sub(b);
			let (diff2, borrow2) = diff1.overflowing_sub(borrow);
			self.data.set_or_insert(i, diff2);
			borrow = borrow1 as u64 + borrow2 as u64;
		}
		self.truncate_leading_zeros();
		borrow == 0
	}

	pub fn add_to_pos(&mut self, mut i: usize, n: u64) {
		let (mut res, mut carry) = self[i].overflowing_add(n);
		self.data[i] = res;
		while carry {
			i += 1;
			(res, carry) = self[i].overflowing_add(carry as u64);
			self.data[i] = res;
		}
	}
}

impl AddAssign<&Self> for BigUInt {
	fn add_assign(&mut self, rhs: &Self) {
		let mut carry = 0u64;
		let len = usize::max(self.len(), rhs.len());
		for i in 0..len {
			let a = self.data.get_or_default(i);
			let b = rhs.data.get_or_default(i);
			let (sum1, carry1) = a.overflowing_add(b);
			let (sum2, carry2) = sum1.overflowing_add(carry);
			self.data.set_or_insert(i, sum2);
			carry = carry1 as u64 + carry2 as u64;
		}
		if carry != 0 {
			self.data.set_or_insert(len, carry);
		}
	}
}

macro_rules! impl_add_assign_u {
	($($t:ty),*) => {
		$(impl AddAssign<$t> for BigUInt
		{
			fn add_assign(&mut self, rhs: $t) {
				*self += &BigUInt::from(rhs);
			}
		})*
	}
}

impl_add_assign_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_add_assign_i {
	($($t:ty),*) => {
		$(impl AddAssign<$t> for BigUInt
		{
			fn add_assign(&mut self, rhs: $t) {
				let abs = BigUInt::from(rhs.unsigned_abs());
				if rhs >= 0 {
					*self += &abs;
				} else {
					*self -= &abs;
				}
			}
		})*
	}
}

impl_add_assign_i! { i8, i16, i32, i64, i128, isize }

impl Add<&BigUInt> for BigUInt {
	type Output = BigUInt;

	fn add(mut self, rhs: &BigUInt) -> Self::Output {
		self += rhs;
		self
	}
}

impl Add<BigUInt> for &BigUInt {
	type Output = BigUInt;

	fn add(self, mut rhs: BigUInt) -> Self::Output {
		rhs += self;
		rhs
	}
}

macro_rules! impl_add_u {
	($($t:ty),*) => {
		$(
			impl Add<$t> for BigUInt {
				type Output = BigUInt;
				fn add(self, rhs: $t) -> Self::Output {
					self + &BigUInt::from(rhs)
				}
			}

			impl Add<BigUInt> for $t {
				type Output = BigUInt;
				fn add(self, rhs: BigUInt) -> Self::Output {
					rhs + &BigUInt::from(self)
				}
			}
		)*
	}
}

impl_add_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_add_i {
	($($t:ty),*) => {
		$(
			impl Add<$t> for BigUInt {
				type Output = BigUInt;
				fn add(mut self, rhs: $t) -> Self::Output {
					self += rhs;
					self
				}
			}

			impl Add<BigUInt> for $t {
				type Output = BigUInt;
				fn add(self, mut rhs: BigUInt) -> Self::Output {
					rhs += self;
					rhs
				}
			}
		)*
	}
}

impl_add_i! { i8, i16, i32, i64, i128, isize }

impl SubAssign<&BigUInt> for BigUInt {
	fn sub_assign(&mut self, rhs: &BigUInt) {
		if !self.checked_sub_assign(rhs) {
			panic!("substruction would result in a negative BigUInt")
		}
	}
}

macro_rules! impl_sub_assign_u {
	($($t:ty),*) => {
		$(impl SubAssign<$t> for BigUInt
		{
			fn sub_assign(&mut self, rhs: $t) {
				*self -= &BigUInt::from(rhs);
			}
		})*
	}
}

impl_sub_assign_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_sub_assign_i {
	($($t:ty),*) => {
		$(impl SubAssign<$t> for BigUInt
		{
			fn sub_assign(&mut self, rhs: $t) {
				let abs = BigUInt::from(rhs.unsigned_abs());
				if rhs >= 0 {
					*self -= &abs;
				} else {
					*self += &abs;
				}
			}
		})*
	}
}

impl_sub_assign_i! { i8, i16, i32, i64, i128, isize }

impl Sub<&BigUInt> for BigUInt {
	type Output = BigUInt;

	fn sub(mut self, rhs: &BigUInt) -> Self::Output {
		self -= rhs;
		self
	}
}

impl Sub<BigUInt> for &BigUInt {
	type Output = BigUInt;

	fn sub(self, mut rhs: BigUInt) -> Self::Output {
		if !rhs.checked_sub_from_assign(self) {
			panic!("substruction would result in a negative BigUInt")
		}
		rhs
	}
}

macro_rules! impl_sub_u {
	($($t:ty),*) => {
		$(
			impl Sub<$t> for BigUInt {
				type Output = BigUInt;
				fn sub(self, rhs: $t) -> Self::Output {
					self - &BigUInt::from(rhs)
				}
			}

			impl Sub<BigUInt> for $t {
				type Output = BigUInt;
				fn sub(self, rhs: BigUInt) -> Self::Output {
					rhs - &BigUInt::from(self)
				}
			}
		)*
	}
}

impl_sub_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_sub_i {
	($($t:ty),*) => {
		$(
			impl Sub<$t> for BigUInt {
				type Output = BigUInt;
				fn sub(mut self, rhs: $t) -> Self::Output {
					self -= rhs;
					self
				}
			}

			impl Sub<BigUInt> for $t {
				type Output = BigUInt;
				fn sub(self, rhs: BigUInt) -> Self::Output {
					let abs = BigUInt::from(self.unsigned_abs());
					if self >= 0 {
						&abs - rhs
					} else {
						panic!("substruction would result in a negative BigUInt")
					}
				}
			}
		)*
	}
}

impl_sub_i! { i8, i16, i32, i64, i128, isize }

impl Mul<&BigUInt> for &BigUInt {
	type Output = BigUInt;

	fn mul(self, rhs: &BigUInt) -> Self::Output {
		let mut res = BigUInt::ZERO;
		res.mul_to(self, rhs);
		res
	}
}

macro_rules! impl_mul_u {
	($($t:ty),*) => {
		$(
			impl Mul<$t> for &BigUInt {
				type Output = BigUInt;
				fn mul(self, rhs: $t) -> Self::Output {
					self * &BigUInt::from(rhs)
				}
			}

			impl Mul<&BigUInt> for $t {
				type Output = BigUInt;
				fn mul(self, rhs: &BigUInt) -> Self::Output {
					rhs * &BigUInt::from(self)
				}
			}
		)*
	}
}

impl_mul_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_mul_i {
	($($t:ty),*) => {
		$(
			impl Mul<$t> for &BigUInt {
				type Output = BigUInt;
				fn mul(self, rhs: $t) -> Self::Output {
					self * &BigUInt::try_from(rhs).expect("multiplication of BigUInt by negative value")
				}
			}

			impl Mul<&BigUInt> for $t {
				type Output = BigUInt;
				fn mul(self, rhs: &BigUInt) -> Self::Output {
					rhs * &BigUInt::try_from(self).expect("multiplication of BigUInt by negative value")
				}
			}
		)*
	}
}

impl_mul_i! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_mul_assign {
	($($t:ty),*) => {
		$(
			impl MulAssign<$t> for BigUInt {
				fn mul_assign(&mut self, rhs: $t) {
					*self = &*self * rhs
				}
			}
		)*
	}
}

impl_mul_assign! { u8, u16, u32, u64, u128, usize, &BigUInt,
i8, i16, i32, i64, i128, isize}

impl BigUInt {
	/// Calculates the quotient and remainder
	/// Preconditions:
	///     * d != 0
	///     * n >= d
	/// Puts quotient in q and remainder in r
	/// n and d are mutable for implementation reasons, they are restored to original values before return.
	/// Uses long division, internally uses Knuth's Algorithm D
	pub fn div_rem_to_unchecked(n: &mut BigUInt, d: &mut BigUInt, q: &mut BigUInt, r: &mut BigUInt) {
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
fn div_n_plus_1_digits_normalized(
	n: &BigUInt,
	d: &BigUInt,
	r: &mut BigUInt
) -> u64 {
	debug_assert!(n.len() == d.len() + 1, "numerator is not 1 digit longer than the denominator");
	debug_assert!(d[d.len() - 1] >= (1 << 63), "denominator is not normalized");
	debug_assert!(*d != 0, "division by zero");

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
	debug_assert!(q_wide <= (u64::MAX as u128), "quotient does not fit in 1 digit");
	q_wide as u64
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::{from_foreign_biguint, to_foreign_biguint};
	#[test]
	fn test_add() {
		let mut u64_max_plus_one = BigUInt::from(u64::MAX);
		u64_max_plus_one += 1u64;
		let cmp1 = BigUInt::from(u64::MAX as u128 + 1u128);
		assert_eq!(u64_max_plus_one, cmp1);

		let mut num1 = u64_max_plus_one;
		num1 += &BigUInt::from(u64::MAX);
		let cmp2 = BigUInt::from(2u128 * u64::MAX as u128 + 1);
		assert_eq!(num1, cmp2);
	}

	#[test]
	fn test_sub() {
		let num1 = (BigUInt::from(u64::MAX) + 1) - 1;
		let cmp1 = BigUInt::from(u64::MAX);
		assert_eq!(num1, cmp1);

		let num2 = BigUInt::from(u128::MAX) + 1 - &BigUInt::from(u128::MAX);
		let cmp2 = BigUInt::from(1u64);
		assert_eq!(num2, cmp2);
	}

	#[test]
	#[should_panic]
	fn test_sub2() {
		let _ = BigUInt::from(1u64) - 2;
	}

	#[test]
	fn test_mul() {
		mul_helper(
			BigUInt::from_vec_le(vec![
				6848468468486468486,
				6851351684844315148,
				87951463548843415,
				6848468135153,
			]),
			BigUInt::from_vec_le(vec![
				486468153601531,
				484684416531315,
				468431513584864,
				84686484684864,
			]),
		);

		mul_helper(
			BigUInt::from_vec_le(vec![
				u64::MAX - 10,
				u64::MAX,
				u64::MAX - 1,
				u64::MAX - 4564564,
			]),
			BigUInt::from_vec_le(vec![u64::MAX - 1, u64::MAX - 156456, u64::MAX, u64::MAX]),
		);

		mul_helper(BigUInt::from(u128::MAX), BigUInt::from(u128::MAX));
	}

	fn mul_helper(a: BigUInt, b: BigUInt) {
		let res_native = &a * &b;
		let res_foreign = from_foreign_biguint(to_foreign_biguint(a) * to_foreign_biguint(b));
		assert_eq!(res_native, res_foreign)
	}

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
		let n = BigUInt::from_vec_le(vec![43543624643643765 , 0xf12f, 0x000000000000ea50fu64]);
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
