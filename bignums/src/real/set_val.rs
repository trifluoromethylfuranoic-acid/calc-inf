use alloc::boxed::Box;

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::TryFromFloatError;
use crate::rational::Rational;
use crate::real::Real;
use crate::{SetVal, TrySetVal};

impl SetVal<&Real> for Real {
	fn set_val(&mut self, src: &Real) {
		*self = src.clone();
	}
}

impl SetVal<BigUInt> for Real {
	fn set_val(&mut self, src: BigUInt) {
		let x = BigFloat::from(src);
		*self = Self::new(move |_prec| x.clone());
	}
}

impl SetVal<BigInt> for Real {
	fn set_val(&mut self, src: BigInt) {
		let x = BigFloat::from(src);
		*self = Self::new(move |_prec| x.clone());
	}
}

impl SetVal<BigFloat> for Real {
	fn set_val(&mut self, src: BigFloat) {
		*self = Self::new(move |_prec| src.clone());
	}
}

impl SetVal<Rational> for Real {
	fn set_val(&mut self, src: Rational) {
		*self = Self::new(move |prec| src.to_float(prec));
	}
}

macro_rules! impl_set_val_ui {
	($($t:ty),*) => {$(
		impl SetVal<$t> for Real {
			fn set_val(&mut self, src: $t) {
				let x = BigFloat::from(src);
				*self = Self::new(move |_prec| x.clone());
			}
		}
	)*};
}

impl_set_val_ui! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

macro_rules! impl_try_set_val_f {
	($($t:ty),*) => {$(
		impl TrySetVal<$t> for Real {
			type Error = TryFromFloatError;
			fn try_set_val(&mut self, src: $t) -> Result<(), Self::Error> {
				let x = BigFloat::try_from(src)?;
				*self = Self::new(move |_prec| x.clone());
				Ok(())
			}
		}
	)*};
}

impl_try_set_val_f! { f32, f64 }
