use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;

impl Neg for BigFloat {
	type Output = BigFloat;

	fn neg(mut self) -> Self::Output {
		self.neg_in_place();
		self
	}
}

impl BigFloat {
	pub fn neg_in_place(&mut self) {
		self.m.neg_in_place();
	}
}

impl Sub<&BigFloat> for &BigFloat {
	type Output = BigFloat;

	fn sub(self, rhs: &BigFloat) -> Self::Output {
		if self.is_zero() {
			return -rhs.clone();
		}
		if rhs.is_zero() {
			return self.clone();
		}
		if self.e >= rhs.e {
			let mut lhs = (*self).clone();
			lhs.m.magnitude <<= lhs.e - rhs.e;
			lhs.e = rhs.e;
			lhs.m -= &rhs.m;
			lhs.normalize();
			lhs
		} else {
			let mut rhs = rhs.clone();
			rhs.m.magnitude <<= rhs.e - self.e;
			rhs.e = self.e;
			rhs.m -= &self.m;
			rhs.normalize();
			rhs
		}
	}
}

impl SubAssign<&BigFloat> for BigFloat {
	fn sub_assign(&mut self, rhs: &BigFloat) {
		*self = &*self - rhs;
	}
}

macro_rules! impl_sub_iu {
	($($t:ty),*) => {$(
		impl Sub<$t> for &BigFloat {
			type Output = BigFloat;

			fn sub(self, rhs: $t) -> Self::Output {
				let rhs = BigFloat::from(rhs);
				self - &rhs
			}
		}

		impl Sub<&BigFloat> for $t {
			type Output = BigFloat;

			fn sub(self, rhs: &BigFloat) -> Self::Output {
				let lhs = BigFloat::from(self);
				&lhs - rhs
			}
		}

		impl SubAssign<$t> for BigFloat {
			fn sub_assign(&mut self, rhs: $t) {
				*self = &*self - rhs;
			}
		}
	)*}
}

impl_sub_iu! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt }

macro_rules! impl_sub_f {
	($($t:ty),*) => {$(
		impl Sub<$t> for &BigFloat {
			type Output = BigFloat;

			fn sub(self, rhs: $t) -> Self::Output {
				let rhs = BigFloat::try_from(rhs).unwrap();
				self - &rhs
			}
		}

		impl Sub<&BigFloat> for $t {
			type Output = BigFloat;

			fn sub(self, rhs: &BigFloat) -> Self::Output {
				let lhs = BigFloat::try_from(self).unwrap();
				&lhs - rhs
			}
		}

		impl SubAssign<$t> for BigFloat {
			fn sub_assign(&mut self, rhs: $t) {
				*self = &*self - rhs;
			}
		}
	)*}
}

impl_sub_f! { f32, f64 }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bigfloat_sub() {
		let a = BigFloat::from(5);
		let b = BigFloat::from(3);
		assert_eq!(&a - &b, BigFloat::from(2));
	}

	#[test]
	fn test_sub_with_zero() {
		let a = BigFloat::from(5);
		let zero = BigFloat::from(0);
		assert_eq!(&a - &zero, a);
		assert_eq!(&zero - &a, BigFloat::from(-5));
	}

	#[test]
	fn test_sub_different_exponents() {
		let mut a = BigFloat::from(5);
		let mut b = BigFloat::from(3);
		a <<= 2; // a = 5 * 2^2
		b <<= 1; // b = 3 * 2^1
		assert_eq!(&a - &b, BigFloat::from(14));
	}

	#[test]
	fn test_sub_integers() {
		let a = BigFloat::from(5);
		assert_eq!(&a - 3u32, BigFloat::from(2));
		assert_eq!(3i64 - &a, BigFloat::from(-2));
	}

	#[test]
	fn test_sub_floats() {
		let a = BigFloat::from(5);
		assert_eq!(&a - 3.0f32, BigFloat::from(2));
		assert_eq!(3.0f64 - &a, BigFloat::from(-2));
	}
}
