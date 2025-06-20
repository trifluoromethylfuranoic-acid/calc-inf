use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::TryFromFloatError;
use crate::rational::Rational;
use crate::{SetVal, TrySetVal, util};

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

impl BigFloat {
	pub fn to_f32(&self) -> f32 {
		if self.is_zero() {
			0.0f32
		} else {
			let shift = self.m.magnitude.leading_zeros();
			let leading_digit = self.m.magnitude.data[self.m.magnitude.len() - 1];
			let second_digit = if self.m.magnitude.len() >= 2 {
				self.m.magnitude.data[self.m.magnitude.len() - 2]
			} else {
				0u64
			};
			let hi = leading_digit << shift | second_digit >> (u64::BITS as u64 - shift);
			let hi_u32 = (hi >> 32) as u32;

			let e = if self.e < i32::MIN as i64 {
				i32::MIN
			} else if self.e > i32::MAX as i64 {
				i32::MAX
			} else {
				i32::try_from(self.e).unwrap()
			}
			.saturating_add(self.m.magnitude.ilog2() as i32);

			util::f32_from_parts(self.is_negative(), hi_u32, e)
		}
	}

	pub fn to_f64(&self) -> f64 {
		if self.is_zero() {
			0.0f64
		} else {
			let shift = self.m.magnitude.leading_zeros();
			let leading_digit = self.m.magnitude.data[self.m.magnitude.len() - 1];
			let second_digit = if self.m.magnitude.len() >= 2 {
				self.m.magnitude.data[self.m.magnitude.len() - 2]
			} else {
				0u64
			};
			let hi = leading_digit << shift | second_digit >> (u64::BITS as u64 - shift);

			let e = self.e.saturating_add(self.m.magnitude.ilog2() as i64);

			util::f64_from_parts(self.is_negative(), hi, e)
		}
	}

	pub fn to_rational(&self) -> Rational {
		let mut n = self.m.clone();
		let mut d = BigUInt::ONE;
		if self.e.is_negative() {
			d <<= -self.e;
		} else {
			n <<= self.e;
		}
		Rational::new(n, d)
	}
}

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
		assert_eq!(a, BigFloat::from_mantissa_exponent(BigInt::ONE, 1));

		let a = BigFloat::try_from(0.5f64).unwrap();
		assert_eq!(a, BigFloat::from_mantissa_exponent(BigInt::ONE, -1));

		let a = BigFloat::try_from(-0.5f64).unwrap();
		assert_eq!(a, BigFloat::from_mantissa_exponent(BigInt::NEG_ONE, -1));

		let a = BigFloat::try_from(0.75).unwrap();
		assert_eq!(a, BigFloat::from_mantissa_exponent(BigInt::from(3u64), -2));
	}

	#[test]
	fn test_from_float_subnormal_f32() {
		let smallest_positive = f32::from_bits(1);
		let a = BigFloat::try_from(smallest_positive).unwrap();
		assert_eq!(a, BigFloat::from_mantissa_exponent(BigInt::ONE, -149));
	}

	#[test]
	fn test_from_float_subnormal_f64() {
		let smallest_positive = f64::from_bits(1);
		let a = BigFloat::try_from(smallest_positive).unwrap();
		assert_eq!(a, BigFloat::from_mantissa_exponent(BigInt::ONE, -1074));
	}

	#[test]
	fn test_to_float_zero() {
		let a = BigFloat::ZERO;
		assert_eq!(a.to_f32(), 0.0f32);
		assert_eq!(a.to_f64(), 0.0f64);
	}

	#[test]
	fn test_to_float_normal_positive() {
		let a = BigFloat::from_mantissa_exponent(BigInt::from(3u64), -2);
		assert_eq!(a.to_f32(), 0.75f32);
		assert_eq!(a.to_f64(), 0.75f64);

		let a = BigFloat::ONE;
		assert_eq!(a.to_f32(), 1.0f32);
		assert_eq!(a.to_f64(), 1.0f64);
	}

	#[test]
	fn test_to_float_normal_negative() {
		let a = BigFloat::from_mantissa_exponent(BigInt::from(-3i64), -2);
		assert_eq!(a.to_f32(), -0.75f32);
		assert_eq!(a.to_f64(), -0.75f64);

		let a = BigFloat::NEG_ONE;
		assert_eq!(a.to_f32(), -1.0f32);
		assert_eq!(a.to_f64(), -1.0f64);
	}

	#[test]
	fn test_to_float_extreme_values() {
		let a = BigFloat::from_mantissa_exponent(BigInt::ONE, 127);
		assert_eq!(a.to_f32(), f32::from_bits(0x7F000000));
		assert_eq!(a.to_f64(), f64::from_bits(0x47E0000000000000));

		let a = BigFloat::from_mantissa_exponent(BigInt::ONE, -127);
		assert_eq!(a.to_f32(), f32::from_bits(0x00400000));
		assert_eq!(a.to_f64(), f64::from_bits(0x3800000000000000));
	}

	#[test]
	fn test_to_rational() {
		let a = BigFloat::ZERO;
		assert_eq!(a.to_rational(), Rational::ZERO);

		let a = BigFloat::ONE;
		assert_eq!(a.to_rational(), Rational::ONE);

		let a = BigFloat::from_mantissa_exponent(BigInt::from(3u64), -2);
		assert_eq!(
			a.to_rational(),
			Rational::new(BigInt::from(3u64), BigUInt::from(4u64))
		);

		let a = BigFloat::from_mantissa_exponent(BigInt::from(-3i64), -2);
		assert_eq!(
			a.to_rational(),
			Rational::new(BigInt::from(-3i64), BigUInt::from(4u64))
		);
	}
}
