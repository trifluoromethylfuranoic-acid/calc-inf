use crate::SetVal;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;

macro_rules! impl_set_val_u {
	($($t:ty),*) => {$(
		impl SetVal<$t> for BigInt {
			fn set_val(&mut self, src: $t) {
				self.is_negative = false;
				self.magnitude.set_val(src);
			}
		}
	)*}
}

impl_set_val_u! { u8, u16, u32, u64, u128, usize, &BigUInt }

macro_rules! impl_set_val_i {
	($($t:ty),*) => {$(
		impl SetVal<$t> for BigInt {
			fn set_val(&mut self, src: $t) {
				self.is_negative = src.is_negative();
				self.magnitude.set_val(src.unsigned_abs());
			}
		}
	)*}
}

impl_set_val_i! { i8, i16, i32, i64, i128, isize }

impl SetVal<&BigInt> for BigInt {
	fn set_val(&mut self, src: &BigInt) {
		self.is_negative = src.is_negative();
		self.magnitude.set_val(src.inner());
	}
}

impl Clone for BigInt {
	fn clone(&self) -> Self {
		Self {
			is_negative: self.is_negative,
			magnitude: self.magnitude.clone(),
		}
	}

	fn clone_from(&mut self, src: &Self) {
		self.set_val(src)
	}
}
