use core::ops::{Add, AddAssign};

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;

impl BigFloat {
	pub fn add_with_precision(&self, rhs: &BigFloat, prec: i64) -> BigFloat {
		let mut res = self + rhs;
		res.round_to_precision(prec);
		res
	}
}

impl Add<&BigFloat> for &BigFloat {
	type Output = BigFloat;

	fn add(self, rhs: &BigFloat) -> Self::Output {
		if self.is_zero() {
			return rhs.clone();
		}
		if rhs.is_zero() {
			return self.clone();
		}
		if self.e >= rhs.e {
			let mut lhs = (*self).clone();
			lhs.m.magnitude <<= lhs.e - rhs.e;
			lhs.e = rhs.e;
			lhs.m += &rhs.m;
			lhs.normalize();
			lhs
		} else {
			let mut rhs = rhs.clone();
			rhs.m.magnitude <<= rhs.e - self.e;
			rhs.e = self.e;
			rhs.m += &self.m;
			rhs.normalize();
			rhs
		}
	}
}

impl AddAssign<&BigFloat> for BigFloat {
	fn add_assign(&mut self, rhs: &BigFloat) {
		*self = &*self + rhs;
	}
}

macro_rules! impl_add_iu {
	($($t:ty),*) => {$(
		impl Add<$t> for &BigFloat {
			type Output = BigFloat;

			fn add(self, rhs: $t) -> Self::Output {
				let rhs = BigFloat::from(rhs);
				self + &rhs
			}
		}

		impl Add<&BigFloat> for $t {
			type Output = BigFloat;

			fn add(self, rhs: &BigFloat) -> Self::Output {
				let lhs = BigFloat::from(self);
				&lhs + rhs
			}
		}

		impl AddAssign<$t> for BigFloat {
			fn add_assign(&mut self, rhs: $t) {
				*self = &*self + rhs;
			}
		}
	)*}
}

impl_add_iu! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt }

macro_rules! impl_add_f {
	($($t:ty),*) => {$(
		impl Add<$t> for &BigFloat {
			type Output = BigFloat;

			fn add(self, rhs: $t) -> Self::Output {
				let rhs = BigFloat::try_from(rhs).unwrap();
				self + &rhs
			}
		}

		impl Add<&BigFloat> for $t {
			type Output = BigFloat;

			fn add(self, rhs: &BigFloat) -> Self::Output {
				let lhs = BigFloat::try_from(self).unwrap();
				&lhs + rhs
			}
		}

		impl AddAssign<$t> for BigFloat {
			fn add_assign(&mut self, rhs: $t) {
				*self = &*self + rhs;
			}
		}
	)*}
}

impl_add_f! { f32, f64 }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bigfloat_add() {
		let a = BigFloat::from(5);
		let b = BigFloat::from(3);
		assert_eq!(&a + &b, BigFloat::from(8));
	}

	#[test]
	fn test_add_with_zero() {
		let a = BigFloat::from(5);
		let zero = BigFloat::from(0);
		assert_eq!(&a + &zero, a);
		assert_eq!(&zero + &a, a);
	}

	#[test]
	fn test_add_different_exponents() {
		let mut a = BigFloat::from(5);
		let mut b = BigFloat::from(3);
		a <<= 2; // a = 5 * 2^2
		b <<= 1; // b = 3 * 2^1
		assert_eq!(&a + &b, BigFloat::from(26));
	}

	#[test]
	fn test_add_integers() {
		let a = BigFloat::from(5);
		assert_eq!(&a + 3u32, BigFloat::from(8));
		assert_eq!(3i64 + &a, BigFloat::from(8));
	}

	#[test]
	fn test_add_floats() {
		let a = BigFloat::from(5);
		assert_eq!(&a + 3.0f32, BigFloat::from(8));
		assert_eq!(3.0f64 + &a, BigFloat::from(8));
	}
}
