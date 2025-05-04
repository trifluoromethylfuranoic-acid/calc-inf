use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::biguint::BigUInt;
use crate::util::{carrying_mul, VecExt};

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
}
