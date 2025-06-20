use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::TryFromFloatError;
use crate::{SetVal, TrySetVal, util};

impl SetVal<&BigFloat> for BigFloat {
	fn set_val(&mut self, src: &BigFloat) {
		self.m.set_val(&src.m);
		self.e = src.e;
	}
}

impl SetVal<&BigUInt> for BigFloat {
	fn set_val(&mut self, src: &BigUInt) {
		if src.is_zero() {
			self.set_zero();
			return;
		}
		self.m.set_val(src);
		self.e = 0;
		self.normalize();
	}
}

impl SetVal<&BigInt> for BigFloat {
	fn set_val(&mut self, src: &BigInt) {
		if src.is_zero() {
			self.set_zero();
			return;
		}
		self.m.set_val(src);
		self.e = 0;
		self.normalize();
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
		let (is_negative, m, e) = util::f32_to_parts(src)?;
		self.m.set_val(m);
		self.m.set_sign(is_negative);
		self.e = e as i64 - m.ilog2() as i64;
		self.normalize();
		Ok(())
	}
}

impl TrySetVal<f64> for BigFloat {
	type Error = TryFromFloatError;

	fn try_set_val(&mut self, src: f64) -> Result<(), Self::Error> {
		let (is_negative, m, e) = util::f64_to_parts(src)?;
		self.m.set_val(m);
		self.m.set_sign(is_negative);
		self.e = e - m.ilog2() as i64;
		self.normalize();
		Ok(())
	}
}

impl Clone for BigFloat {
	fn clone(&self) -> Self {
		Self {
			m: self.m.clone(),
			e: self.e,
		}
	}

	fn clone_from(&mut self, source: &Self) {
		self.set_val(source);
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
		assert_eq!(a.exponent(), 0);
	}

	#[test]
	fn test_set_val_bigint() {
		let mut a = BigFloat::ZERO;
		let b = BigInt::from(-123);
		a.set_val(&b);
		assert_eq!(a.mantissa(), &BigInt::from(-123));
		assert_eq!(a.exponent(), 0);
	}

	#[test]
	fn test_set_val_primitive() {
		let mut a = BigFloat::ZERO;
		a.set_val(123i32);
		assert_eq!(a.mantissa(), &BigInt::from(123));
		assert_eq!(a.exponent(), 0);
	}
}
