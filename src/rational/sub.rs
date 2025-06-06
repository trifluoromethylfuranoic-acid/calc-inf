use core::ops::{Neg, Sub, SubAssign};
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::rational::Rational;

impl Rational {
	pub fn neg_in_place(&mut self) {
		self.n.neg_in_place();
	}
}

impl Neg for Rational {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		self.neg_in_place();
		self
	}
}

impl Sub<&Rational> for &Rational {
	type Output = Rational;

	fn sub(self, rhs: &Rational) -> Self::Output {
		Rational::new(
			&self.n * &rhs.d - &(&rhs.n * &self.d),
			&self.d * &rhs.d
		)
	}
}

macro_rules! impl_sub {
	($($t:ty),*) => {$(
		impl Sub<$t> for &Rational {
			type Output = Rational;

			fn sub(self, rhs: $t) -> Self::Output {
				Rational::new(
					&self.n - rhs * &self.d,
					self.d.clone()
				)
			}
		}

		impl Sub<&Rational> for $t {
			type Output = Rational;

			fn sub(self, rhs: &Rational) -> Self::Output {
				Rational::new(
					self * &rhs.d - &rhs.n,
					rhs.d.clone()
				)
			}
		}
	)*}
}

impl_sub! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt }

macro_rules! impl_sub_assign {
	($($t:ty),*) => {$(
		impl SubAssign<$t> for Rational {
			fn sub_assign(&mut self, rhs: $t) {
				*self = &*self - rhs;
			}
		}
	)*}
}

impl_sub_assign! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt, &Rational }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rational_sub() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let b = Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		let result = &a - &b;
		assert_eq!(*result.numerator(), BigInt::from(1));
		assert_eq!(*result.denominator(), BigUInt::from(6u64));
	}

	#[test]
	fn test_rational_sub_integer() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = &a - 2i32;
		assert_eq!(*result.numerator(), BigInt::from(-3));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_integer_sub_rational() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = 2i32 - &a;
		assert_eq!(*result.numerator(), BigInt::from(3));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_rational_sub_assign() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a -= &Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		assert_eq!(*a.numerator(), BigInt::from(1));
		assert_eq!(*a.denominator(), BigUInt::from(6u64));
	}

	#[test]
	fn test_rational_sub_assign_integer() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a -= 2i32;
		assert_eq!(*a.numerator(), BigInt::from(-3));
		assert_eq!(*a.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_rational_neg() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = -a;
		assert_eq!(*result.numerator(), BigInt::from(-1));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));
	}
}


