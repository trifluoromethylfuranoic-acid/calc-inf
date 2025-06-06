use core::convert::{TryFrom, TryInto};

use crate::SetVal;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::TryIntoIntError;

impl From<BigUInt> for BigInt {
	fn from(val: BigUInt) -> Self {
		Self {
			magnitude: val,
			is_negative: false,
		}
	}
}

macro_rules! impl_from {
	($($t:ty),*) => {
		$(impl From<$t> for BigInt {
			fn from(val: $t) -> Self {
				let mut res = Self::ZERO;
				res.set_val(val);
				res
			}
		})*
	}
}

impl_from! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

macro_rules! impl_try_into_u {
	($($t:ty),*) => {$(
		impl TryFrom<&BigInt> for $t {
			type Error = TryIntoIntError;
			fn try_from(val: &BigInt) -> Result<Self, Self::Error> {
				if val.is_negative {
					Err(TryIntoIntError)
				} else {
					val.inner().try_into().map_err(|_| TryIntoIntError)
				}
			}
		}
		)*}
}

impl_try_into_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_try_into_i {
	($($u:ty => $i:ty),*) => {$(
		impl TryFrom<&BigInt> for $i {
			type Error = TryIntoIntError;
			fn try_from(val: &BigInt) -> Result<Self, Self::Error> {
				let mag: $u = val.inner().try_into()?;
				<$i>::from_sign_and_magnitude(val.is_negative, mag).ok_or(TryIntoIntError)
			}
		}
	)*}
}

impl_try_into_i! { u8 => i8, u16 => i16, u32 => i32, u64 => i64, u128 => i128, usize => isize }

trait FromSignAndMagnitude<T>: Sized {
	fn from_sign_and_magnitude(is_negative: bool, mag: T) -> Option<Self>;
}

macro_rules! from_sign_and_magnitude {
	($($u:ty => $i:ty),*) => {$(
		impl FromSignAndMagnitude<$u> for $i {
			fn from_sign_and_magnitude(is_negative: bool, mag: $u) -> Option<$i> {
				if is_negative {
					if mag <= (<$i>::MAX as $u) {
						Some(-(mag as $i))
					} else if mag == (<$i>::MAX as $u) + 1 {
						Some(<$i>::MIN)
					} else { None }
				} else {
					mag.try_into().ok()
				}
			}
		}
	)*}
}

from_sign_and_magnitude! { u8 => i8, u16 => i16, u32 => i32, u64 => i64, u128 => i128, usize => isize }
