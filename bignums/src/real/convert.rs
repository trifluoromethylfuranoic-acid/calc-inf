use alloc::boxed::Box;

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::TryFromFloatError;
use crate::rational::Rational;
use crate::real::Real;

impl From<BigFloat> for Real {
	fn from(value: BigFloat) -> Self {
		Real::new(move |_prec| value.clone())
	}
}

impl From<Rational> for Real {
	fn from(value: Rational) -> Self {
		Real::new(move |prec| value.to_float(prec))
	}
}

macro_rules! impl_from {
	($($t:ty),*) => {$(
		impl From<$t> for Real {
			fn from(value: $t) -> Self {
				let x = BigFloat::from(value);
				Real::from(x)
			}
		}
	)*}
}

impl_from! { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
BigUInt, BigInt, &BigUInt, &BigInt }

macro_rules! impl_try_from {
($($t:ty),*) => {$(
		impl TryFrom<$t> for Real {
			type Error = TryFromFloatError;
			fn try_from(value: $t) -> Result<Self, Self::Error> {
				let x = BigFloat::try_from(value)?;
				Ok(Real::from(x))
			}
		}
	)*}
}

impl_try_from! { f32, f64 }
