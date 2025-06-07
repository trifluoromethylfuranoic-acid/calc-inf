use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::TryFromFloatError;
use crate::{SetVal, TrySetVal};

macro_rules! impl_from {
	($($t:ty),*) => {$(
		impl From<$t> for BigFloat {
			fn from(value: $t) -> Self {
				let mut res = Self::ZERO;
				res.set_val(value);
				res
			}
		}
	)*}
}

impl_from! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &BigUInt, &BigInt }

macro_rules! impl_try_from {
	($($t:ty),*) => {$(
		impl TryFrom<$t> for BigFloat {
			type Error = TryFromFloatError;
			fn try_from(value: $t) -> Result<Self, Self::Error> {
				let mut res = Self::ZERO;
				res.try_set_val(value)?;
				Ok(res)
			}
		}
	)*}
}

impl_try_from! { f32, f64 }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_float_normal() {
		let a = BigFloat::from(1);
		assert_eq!(a, BigFloat::ONE);

		let a = BigFloat::try_from(1.0).unwrap();
		assert_eq!(a, BigFloat::ONE);

		let a = BigFloat::try_from(2.0).unwrap();
		assert_eq!(
			a,
			BigFloat::from_mantissa_exponent(BigInt::ONE, BigInt::ONE)
		);

		let a = BigFloat::try_from(0.5f64).unwrap();
		assert_eq!(
			a,
			BigFloat::from_mantissa_exponent(BigInt::ONE, BigInt::NEG_ONE)
		);

		let a = BigFloat::try_from(-0.5f64).unwrap();
		assert_eq!(
			a,
			BigFloat::from_mantissa_exponent(BigInt::NEG_ONE, BigInt::NEG_ONE)
		);
	}

	#[test]
	fn test_from_float_subnormal_f32() {
		let smallest_positive = f32::from_bits(1);
		let a = BigFloat::try_from(smallest_positive).unwrap();
		assert_eq!(
			a,
			BigFloat::from_mantissa_exponent(BigInt::ONE, BigInt::from(-149))
		);
	}

	#[test]
	fn test_from_float_subnormal_f64() {
		let smallest_positive = f64::from_bits(1);
		let a = BigFloat::try_from(smallest_positive).unwrap();
		assert_eq!(
			a,
			BigFloat::from_mantissa_exponent(BigInt::ONE, BigInt::from(-1074))
		);
	}
}
