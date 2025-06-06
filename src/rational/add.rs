use core::ops::{Add, AddAssign};

use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::rational::Rational;

impl Add<&Rational> for &Rational {
	type Output = Rational;

	fn add(self, rhs: &Rational) -> Self::Output {
		Rational::new(&self.n * &rhs.d + &(&rhs.n * &self.d), &self.d * &rhs.d)
	}
}

macro_rules! impl_add {
	($($t:ty),*) => {$(
		impl Add<$t> for &Rational {
			type Output = Rational;

			fn add(self, rhs: $t) -> Self::Output {
				Rational::new(
					&self.n + rhs * &self.d,
					self.d.clone()
				)
			}
		}

		impl Add<&Rational> for $t {
			type Output = Rational;

			fn add(self, rhs: &Rational) -> Self::Output {
				Rational::new(
					&rhs.n + self * &rhs.d,
					rhs.d.clone()
				)
			}
		}
	)*}
}

impl_add! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt }

macro_rules! impl_add_assign {
	($($t:ty),*) => {$(
		impl AddAssign<$t> for Rational {
			fn add_assign(&mut self, rhs: $t) {
				*self = &*self + rhs;
			}
		}
	)*}
}

impl_add_assign! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt, &Rational }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rational_add() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let b = Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		let result = &a + &b;
		assert_eq!(*result.numerator(), BigInt::from(5));
		assert_eq!(*result.denominator(), BigUInt::from(6u64));
	}

	#[test]
	fn test_rational_add_integer() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = &a + 2i32;
		assert_eq!(*result.numerator(), BigInt::from(5));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_integer_add_rational() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = 2i32 + &a;
		assert_eq!(*result.numerator(), BigInt::from(5));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_rational_add_assign() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a += &Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		assert_eq!(*a.numerator(), BigInt::from(5));
		assert_eq!(*a.denominator(), BigUInt::from(6u64));
	}

	#[test]
	fn test_rational_add_assign_integer() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a += 2i32;
		assert_eq!(*a.numerator(), BigInt::from(5));
		assert_eq!(*a.denominator(), BigUInt::from(2u64));
	}
}
