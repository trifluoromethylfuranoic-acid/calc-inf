use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::TryFromFloatError;
use crate::{SetVal, TrySetVal, util};

impl SetVal<&BigFloat> for BigFloat {
	fn set_val(&mut self, src: &BigFloat) {
		self.m.set_val(&src.m);
		self.e.set_val(&src.e);
	}
}

impl SetVal<&BigUInt> for BigFloat {
	fn set_val(&mut self, src: &BigUInt) {
		if src.is_zero() {
			self.set_zero();
			return;
		}
		self.m.set_val(src);
		self.m >>= src.trailing_zeros();
		self.e.set_val(src.ilog2());
	}
}

impl SetVal<&BigInt> for BigFloat {
	fn set_val(&mut self, src: &BigInt) {
		if src.is_zero() {
			self.set_zero();
			return;
		}
		self.m.set_val(src);
		self.m >>= src.magnitude.trailing_zeros();
		self.e.set_val(src.magnitude.ilog2());
	}
}

macro_rules! impl_set_val {
	($($t:ty),*) => {$(
		impl SetVal<$t> for BigFloat {
			fn set_val(&mut self, src: $t) {
				self.set_val(&BigInt::from(src))
			}
		}
	)*};
}

impl_set_val! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl TrySetVal<f32> for BigFloat {
	type Error = TryFromFloatError;

	fn try_set_val(&mut self, src: f32) -> Result<(), Self::Error> {
		let (is_negative, mut m, e) = util::f32_parts(src)?;
		m >>= m.trailing_zeros();
		self.m.set_val(m);
		self.m.set_sign(is_negative);
		self.e.set_val(e);
		Ok(())
	}
}

impl TrySetVal<f64> for BigFloat {
	type Error = TryFromFloatError;

	fn try_set_val(&mut self, src: f64) -> Result<(), Self::Error> {
		let (is_negative, mut m, e) = util::f64_parts(src)?;
		m >>= m.trailing_zeros();
		self.m.set_val(m);
		self.m.set_sign(is_negative);
		self.e.set_val(e);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_set_val_biguint() {
		let mut a = BigFloat::ZERO;
		let b = BigUInt::from(123u32);
		a.set_val(&b);
		assert_eq!(a.mantissa(), &BigInt::from(123));
		assert_eq!(a.exponent(), &BigInt::from(6));
	}

	#[test]
	fn test_set_val_bigint() {
		let mut a = BigFloat::ZERO;
		let b = BigInt::from(-123);
		a.set_val(&b);
		assert_eq!(a.mantissa(), &BigInt::from(-123));
		assert_eq!(a.exponent(), &BigInt::from(6));
	}

	#[test]
	fn test_set_val_primitive() {
		let mut a = BigFloat::ZERO;
		a.set_val(123i32);
		assert_eq!(a.mantissa(), &BigInt::from(123));
		assert_eq!(a.exponent(), &BigInt::from(6));
	}
}
