use core::convert::TryInto;

use crate::biguint::BigUInt;
use crate::error::TryFromIntError;
use crate::{SetVal, TrySetVal, util};

macro_rules! impl_set_val_u {
	($($t:ty),*) => {
		$(impl SetVal<$t> for BigUInt {
			fn set_val(&mut self, src: $t) {
				let val = src as u64;
				self.data.clear();
				if val != 0u64 {
					self.data.push(val);
				}
			}
		})*
	}
}

impl_set_val_u! { u8, u16, u32, u64 }

impl SetVal<u128> for BigUInt {
	fn set_val(&mut self, src: u128) {
		let le = util::u128_to_u64s(src);
		self.data.clear();
		self.data.extend(le);
		self.truncate_leading_zeros();
	}
}

impl SetVal<usize> for BigUInt {
	fn set_val(&mut self, src: usize) {
		#[cfg(target_pointer_width = "16")]
		let val = src as u16;
		#[cfg(target_pointer_width = "32")]
		let val = src as u32;
		#[cfg(target_pointer_width = "64")]
		let val = src as u64;

		self.data.clear();
		if val != 0u64 {
			self.data.push(val);
		}
	}
}

impl<'a> SetVal<&'a Self> for BigUInt {
	fn set_val(&mut self, src: &Self) {
		self.data.clone_from(&src.data)
	}
}

macro_rules! impl_try_set_val_i {
	($($i:ty => $u:ty),*) => {
		$(impl TrySetVal<$i> for BigUInt {
			type Error = TryFromIntError;

			fn try_set_val(&mut self, src: $i) -> Result<(), Self::Error> {
				let val: $u = src.try_into().map_err(|_| TryFromIntError)?;
				self.set_val(val);
				Ok(())
			}
		})*
	}
}

impl_try_set_val_i! { i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128, isize => usize }

impl Clone for BigUInt {
	fn clone(&self) -> Self {
		Self {
			data: self.data.clone(),
		}
	}

	fn clone_from(&mut self, src: &Self) {
		self.set_val(src)
	}
}
