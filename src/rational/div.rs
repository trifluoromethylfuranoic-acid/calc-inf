use core::ops::{Div, DivAssign};

use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::rational::Rational;

impl Div<&Rational> for &Rational {
	type Output = Rational;

	fn div(self, rhs: &Rational) -> Self::Output {
		assert!(!rhs.is_zero(), "division by zero");
		let mut num = &self.n * &rhs.d;
		if rhs.is_negative() {
			num.neg_in_place();
		}
		Rational::new(num, &self.d * rhs.n.inner())
	}
}

impl Div<&BigUInt> for &Rational {
	type Output = Rational;

	fn div(self, rhs: &BigUInt) -> Self::Output {
		assert!(!rhs.is_zero(), "division by zero");
		Rational::new(self.n.clone(), &self.d * rhs)
	}
}

impl Div<&Rational> for &BigUInt {
	type Output = Rational;

	fn div(self, rhs: &Rational) -> Self::Output {
		assert!(!rhs.is_zero(), "division by zero");
		let mut num = BigInt::from(self * &rhs.d);
		if rhs.is_negative() {
			num.neg_in_place();
		}
		Rational::new(num, rhs.n.clone().unsigned_abs())
	}
}

macro_rules! impl_div_u {
    ($($t:ty),*) => {$(
        impl Div<$t> for &Rational {
            type Output = Rational;

            fn div(self, rhs: $t) -> Self::Output {
                assert_ne!(rhs, 0, "division by zero");
                Rational::new(
                    self.n.clone(),
                    &self.d * rhs
                )
            }
        }

        impl Div<&Rational> for $t {
            type Output = Rational;

            fn div(self, rhs: &Rational) -> Self::Output {
                assert!(!rhs.is_zero(), "division by zero");
                let mut num = BigInt::from(self * &rhs.d);
                if rhs.is_negative() {
                    num.neg_in_place();
                }
                Rational::new(
                    num,
                    rhs.n.clone().unsigned_abs()
                )
            }
        }
    )*}
}

impl_div_u! { u8, u16, u32, u64, u128, usize }

impl Div<&BigInt> for &Rational {
	type Output = Rational;

	fn div(self, rhs: &BigInt) -> Self::Output {
		assert!(!rhs.is_zero(), "division by zero");
		let mut num = self.n.clone();
		if rhs.is_negative() {
			num.neg_in_place();
		}
		Rational::new(num, &self.d * rhs.inner())
	}
}

impl Div<&Rational> for &BigInt {
	type Output = Rational;

	fn div(self, rhs: &Rational) -> Self::Output {
		assert!(!rhs.is_zero(), "division by zero");
		let mut num = self * &rhs.d;
		if rhs.is_negative() {
			num.neg_in_place();
		}
		Rational::new(num, rhs.n.clone().unsigned_abs())
	}
}

macro_rules! impl_div_i {
    ($($t:ty),*) => {$(
        impl Div<$t> for &Rational {
            type Output = Rational;

            fn div(self, rhs: $t) -> Self::Output {
                assert_ne!(rhs, 0, "division by zero");
                let mut num = self.n.clone();
                if rhs.is_negative() {
                    num.neg_in_place();
                }
                Rational::new(
                    num,
                    &self.d * rhs.unsigned_abs()
                )
            }
        }

        impl Div<&Rational> for $t {
            type Output = Rational;

            fn div(self, rhs: &Rational) -> Self::Output {
                assert!(!rhs.is_zero(), "division by zero");
                let mut num = self * &BigInt::from(rhs.d.clone());
                if rhs.is_negative() {
                    num.neg_in_place();
                }
                Rational::new(
                    num,
                    rhs.n.clone().unsigned_abs()
                )
            }
        }
    )*}
}

impl_div_i! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_div_assign {
    ($($t:ty),*) => {$(
        impl DivAssign<$t> for Rational {
            fn div_assign(&mut self, rhs: $t) {
                *self = &*self / rhs;
            }
        }
    )*}
}

impl_div_assign! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt, &Rational }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rational_div() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let b = Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		let result = &a / &b;
		assert_eq!(*result.numerator(), BigInt::from(3));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));

		// Test negative rationals
		let c = Rational::new(BigInt::from(-1), BigUInt::from(2u64)); // -1/2
		let d = Rational::new(BigInt::from(-1), BigUInt::from(3u64)); // -1/3
		let result = &c / &d;
		assert_eq!(*result.numerator(), BigInt::from(3));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));

		// Test mixed signs
		let result = &c / &b;
		assert_eq!(*result.numerator(), BigInt::from(-3));
		assert_eq!(*result.denominator(), BigUInt::from(2u64));

		// Test division by negative integer
		let result = &a / -2i32;
		assert_eq!(*result.numerator(), BigInt::from(-1));
		assert_eq!(*result.denominator(), BigUInt::from(4u64));
	}

	#[test]
	fn test_rational_div_integer() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = &a / 2i32;
		assert_eq!(*result.numerator(), BigInt::from(1));
		assert_eq!(*result.denominator(), BigUInt::from(4u64));
	}

	#[test]
	fn test_integer_div_rational() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		let result = 2i32 / &a;
		assert_eq!(*result.numerator(), BigInt::from(4));
		assert_eq!(*result.denominator(), BigUInt::from(1u64));
	}

	#[test]
	fn test_rational_div_assign() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a /= &Rational::new(BigInt::from(1), BigUInt::from(3u64)); // 1/3
		assert_eq!(*a.numerator(), BigInt::from(3));
		assert_eq!(*a.denominator(), BigUInt::from(2u64));
	}

	#[test]
	fn test_rational_div_assign_integer() {
		let mut a = Rational::new(BigInt::from(1), BigUInt::from(2u64)); // 1/2
		a /= 2i32;
		assert_eq!(*a.numerator(), BigInt::from(1));
		assert_eq!(*a.denominator(), BigUInt::from(4u64));
	}

	#[test]
	#[should_panic(expected = "division by zero")]
	fn test_rational_div_by_zero() {
		let a = Rational::new(BigInt::from(1), BigUInt::from(2u64));
		let b = Rational::ZERO;
		let _ = &a / &b;
	}
}
