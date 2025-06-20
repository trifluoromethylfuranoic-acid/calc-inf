use core::ops::{Mul, MulAssign};

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;

impl Mul<&BigFloat> for &BigFloat {
	type Output = BigFloat;

	fn mul(self, rhs: &BigFloat) -> Self::Output {
		BigFloat::from_mantissa_exponent(&self.m * &rhs.m, self.e + rhs.e)
	}
}

impl MulAssign<&BigFloat> for BigFloat {
	fn mul_assign(&mut self, rhs: &BigFloat) {
		*self = &*self * rhs;
	}
}

macro_rules! impl_mul_ui {
	($($t:ty),*) => {$(
		impl Mul<$t> for &BigFloat {
			type Output = BigFloat;

			fn mul(self, rhs: $t) -> Self::Output {
				BigFloat::from_mantissa_exponent(
					&self.m * rhs,
					self.e
				)
			}
		}

		impl Mul<&BigFloat> for $t {
			type Output = BigFloat;

			fn mul(self, rhs: &BigFloat) -> Self::Output {
				BigFloat::from_mantissa_exponent(
					self * &rhs.m,
					rhs.e
				)
			}
		}

		impl MulAssign<$t> for BigFloat {
			fn mul_assign(&mut self, rhs: $t) {
				self.m *= rhs;
				self.normalize();
			}
		}
	)*}
}

impl_mul_ui! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt }

macro_rules! impl_mul_f {
	($($t:ty),*) => {$(
		impl Mul<$t> for &BigFloat {
			type Output = BigFloat;

			fn mul(self, rhs: $t) -> Self::Output {
				self * &BigFloat::try_from(rhs).unwrap()
			}
		}

		impl Mul<&BigFloat> for $t {
			type Output = BigFloat;

			fn mul(self, rhs: &BigFloat) -> Self::Output {
				&BigFloat::try_from(self).unwrap() * rhs
			}
		}

		impl MulAssign<$t> for BigFloat {
			fn mul_assign(&mut self, rhs: $t) {
				*self *= &BigFloat::try_from(rhs).unwrap();
			}
		}
	)*}
}

impl_mul_f! { f32, f64 }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bigfloat_mul() {
		let a = BigFloat::from(5);
		let b = BigFloat::from(3);
		assert_eq!(&a * &b, BigFloat::from(15));

		let c = BigFloat::try_from(2.5f64).unwrap();
		let d = BigFloat::try_from(1.5f64).unwrap();
		assert_eq!(&c * &d, BigFloat::try_from(3.75f64).unwrap());
	}

	#[test]
	fn test_mul_with_zero() {
		let a = BigFloat::from(5);
		let zero = BigFloat::from(0);
		assert_eq!(&a * &zero, zero);
		assert_eq!(&zero * &a, zero);
	}

	#[test]
	fn test_mul_integers() {
		let a = BigFloat::from(5);
		assert_eq!(&a * 3u32, BigFloat::from(15));
		assert_eq!(3i64 * &a, BigFloat::from(15));
	}

	#[test]
	fn test_mul_floats() {
		let a = BigFloat::from(5);
		assert_eq!(&a * 3.0f32, BigFloat::from(15));
		assert_eq!(3.0f64 * &a, BigFloat::from(15));
	}
}
