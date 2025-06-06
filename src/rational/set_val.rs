use core::num::FpCategory;

use crate::bigint::BigInt;
use crate::error::TryFromFloatError;
use crate::rational::Rational;
use crate::{SetVal, TrySetVal};

impl SetVal<&Rational> for Rational {
	fn set_val(&mut self, src: &Rational) {
		self.n.set_val(&src.n);
		self.d.set_val(&src.d)
	}
}

impl<T> SetVal<T> for Rational
where
	BigInt: SetVal<T>,
{
	fn set_val(&mut self, src: T) {
		self.n.set_val(src);
		self.d.set_one();
	}
}

impl TrySetVal<f32> for Rational {
	type Error = TryFromFloatError;

	fn try_set_val(&mut self, src: f32) -> Result<(), Self::Error> {
		match src.classify() {
			FpCategory::Nan => Err(TryFromFloatError::NaN),
			FpCategory::Infinite => Err(TryFromFloatError::Infinite),
			FpCategory::Zero => {
				self.set_zero();
				Ok(())
			}
			FpCategory::Subnormal => {
				todo!()
			}
			FpCategory::Normal => {
				todo!()
			}
		}
	}
}

impl Clone for Rational {
	fn clone(&self) -> Self {
		Self {
			n: self.n.clone(),
			d: self.d.clone(),
		}
	}

	fn clone_from(&mut self, source: &Self) {
		self.set_val(source);
	}
}
