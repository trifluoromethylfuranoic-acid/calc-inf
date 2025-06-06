use core::ops::{Add, AddAssign};

use crate::biguint::BigUInt;
use crate::util::VecExt;

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

#[cfg(test)]
mod tests {
	use super::*;

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
}
