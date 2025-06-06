use core::ops::{Sub, SubAssign};

use crate::biguint::BigUInt;
use crate::util::VecExt;

pub trait CheckedSub<T> {
	fn checked_sub(self, rhs: T) -> Option<BigUInt>;
}

impl CheckedSub<&BigUInt> for BigUInt {
	fn checked_sub(mut self, rhs: &BigUInt) -> Option<BigUInt> {
		let succ = self.checked_sub_assign(rhs);
		if succ { Some(self) } else { None }
	}
}

macro_rules! impl_checked_sub_u {
	($($t:ty),*) => {
		$(impl CheckedSub<$t> for BigUInt
		{
			fn checked_sub(self, rhs: $t) -> Option<BigUInt> {
				self.checked_sub(&BigUInt::from(rhs))
			}
		})*
	}
}

impl_checked_sub_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_checked_sub_i {
	($($t:ty),*) => {
		$(impl CheckedSub<$t> for BigUInt
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

impl_checked_sub_i! { i8, i16, i32, i64, i128, isize }

impl BigUInt {
	/// Calculates self - lhs, saves result into self
	/// Returns false and leaves garbage in self on overflow.
	pub(crate) fn checked_sub_assign(&mut self, rhs: &Self) -> bool {
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
	pub(crate) fn checked_sub_from_assign(&mut self, lhs: &Self) -> bool {
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
}

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

#[cfg(test)]
mod tests {
	use super::*;

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
}
