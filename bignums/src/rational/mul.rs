use core::ops::{Mul, MulAssign};

use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::rational::Rational;

impl Mul<&Rational> for &Rational {
	type Output = Rational;

	fn mul(self, rhs: &Rational) -> Self::Output {
		Rational::new(&self.n * &rhs.n, &self.d * &rhs.d)
	}
}

macro_rules! impl_mul {
    ($($t:ty),*) => {$(
        impl Mul<$t> for &Rational {
            type Output = Rational;

            fn mul(self, rhs: $t) -> Self::Output {
                Rational::new(
                    &self.n * rhs,
                    self.d.clone()
                )
            }
        }

        impl Mul<&Rational> for $t {
            type Output = Rational;

            fn mul(self, rhs: &Rational) -> Self::Output {
                Rational::new(
                    &rhs.n * self,
                    rhs.d.clone()
                )
            }
        }
    )*}
}

impl_mul! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt }

macro_rules! impl_mul_assign {
    ($($t:ty),*) => {$(
        impl MulAssign<$t> for Rational {
            fn mul_assign(&mut self, rhs: $t) {
                *self = &*self * rhs;
            }
        }
    )*}
}

impl_mul_assign! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt, &Rational }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rational_mul() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let b = Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		let result = &a * &b;
		assert_eq!(*result.numerator(), BigInt::from(1));
		assert_eq!(*result.denominator(), BigUInt::from(6u64));
	}

	#[test]
	fn test_rational_mul_integer() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = &a * 2i32;
		assert_eq!(*result.numerator(), BigInt::from(2));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_integer_mul_rational() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = 2i32 * &a;
		assert_eq!(*result.numerator(), BigInt::from(2));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_rational_mul_assign() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a *= &Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		assert_eq!(*a.numerator(), BigInt::from(1));
		assert_eq!(*a.denominator(), BigUInt::from(6u64));
	}

	#[test]
	fn test_rational_mul_assign_integer() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a *= 2i32;
		assert_eq!(*a.numerator(), BigInt::from(2));
		assert_eq!(*a.denominator(), BigUInt::from(2u64));
	}
}
