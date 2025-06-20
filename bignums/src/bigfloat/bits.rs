use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::bigfloat::BigFloat;

macro_rules! impl_shr {
	($($t:ty),*) => {$(
		impl ShrAssign<$t> for BigFloat {
			fn shr_assign(&mut self, rhs: $t) {
				self.e = (self.e as i128)
					.strict_sub(rhs.try_into().unwrap())
					.try_into()
					.unwrap();
			}
		}

		impl Shr<$t> for BigFloat {
			type Output = BigFloat;
			fn shr(mut self, rhs: $t) -> BigFloat {
				self >>= rhs;
				self
			}
		}
	)*}
}

impl_shr! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

macro_rules! impl_shl {
	($($t:ty),*) => {$(
		impl ShlAssign<$t> for BigFloat {
			fn shl_assign(&mut self, rhs: $t) {
				self.e = (self.e as i128)
					.strict_add(rhs.try_into().unwrap())
					.try_into()
					.unwrap();
			}
		}

		impl Shl<$t> for BigFloat {
			type Output = BigFloat;
			fn shl(mut self, rhs: $t) -> BigFloat {
				self <<= rhs;
				self
			}
		}
	)*}
}

impl_shl! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
