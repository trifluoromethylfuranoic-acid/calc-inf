use core::convert::{TryFrom, TryInto};

use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::{TryFromIntError, TryIntoIntError};
use crate::util::u64s_to_u128;
use crate::{SetVal, TrySetVal};

impl TryFrom<BigInt> for BigUInt {
	type Error = TryFromIntError;
	fn try_from(value: BigInt) -> Result<Self, Self::Error> {
		if value.is_negative() {
			Err(TryFromIntError)
		} else {
			Ok(value.into_inner())
		}
	}
}

macro_rules! impl_from_u {
	($($t:ty),*) => {
		$(impl From<$t> for BigUInt {
			fn from(val: $t) -> Self {
				let mut res = Self::ZERO;
				res.set_val(val);
				res
			}
		})*
	}
}

impl_from_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_try_from_i {
	($($t:ty),*) => {
		$(impl TryFrom<$t> for BigUInt {
			type Error = TryFromIntError;
			fn try_from(val: $t) -> Result<Self, Self::Error> {
				let mut res = Self::ZERO;
				res.try_set_val(val)?;
				Ok(res)
			}
		})*
	}
}

impl_try_from_i! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_try_into {
	($($t:ty),*) => {
		$(impl TryFrom<&BigUInt> for $t {
			type Error = TryIntoIntError;
			fn try_from(val: &BigUInt) -> Result<Self, Self::Error> {
				match val.len() {
					0usize => Ok(0u64 as $t),
					1usize => Ok(val[0].try_into().map_err(|_| TryIntoIntError)?),
					_ => Err(TryIntoIntError)
				}
			}
		})*
	}
}

impl_try_into! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

impl TryFrom<&BigUInt> for u128 {
	type Error = TryIntoIntError;
	fn try_from(val: &BigUInt) -> Result<Self, Self::Error> {
		match val.len() {
			0usize => Ok(0u64 as u128),
			1usize => Ok(val[0] as u128),
			2usize => Ok(u64s_to_u128([val[0], val[1]])),
			_ => Err(TryIntoIntError),
		}
	}
}

impl TryFrom<&BigUInt> for i128 {
	type Error = TryIntoIntError;
	fn try_from(val: &BigUInt) -> Result<Self, Self::Error> {
		match val.len() {
			0usize => Ok(0u64 as i128),
			1usize => Ok(val[0] as i128),
			2usize => Ok(u64s_to_u128([val[0], val[1]])
				.try_into()
				.map_err(|_| TryIntoIntError)?),
			_ => Err(TryIntoIntError),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create() {
		let zero: BigUInt = Default::default();
		let cmp = BigUInt::from_vec_le(vec![]);
		assert_eq!(zero, cmp);

		let from_u32 = BigUInt::from(128u32);
		let cmp = BigUInt::from_vec_le(vec![128u64]);
		assert_eq!(from_u32, cmp);

		let from_u64 = BigUInt::from(u64::MAX);
		let cmp = BigUInt::from_vec_le(vec![u64::MAX]);
		assert_eq!(from_u64, cmp);

		let from_u128 = BigUInt::from(u128::MAX - 1u128);
		let cmp = BigUInt::from_vec_le(vec![u64::MAX - 1u64, u64::MAX]);
		assert_eq!(from_u128, cmp);

		let from_i32 = BigUInt::try_from(128i32).unwrap();
		let cmp = BigUInt::from_vec_le(vec![128u64]);
		assert_eq!(from_i32, cmp);

		let from_i64 = BigUInt::try_from(-1i64);
		assert!(from_i64.is_err());

		let from_i128 = BigUInt::try_from(i128::MAX).unwrap();
		let cmp = BigUInt::from_vec_le(vec![u64::MAX, i64::MAX as u64]);
		assert_eq!(from_i128, cmp);

		let from_empty_vec = BigUInt::from_vec_le(vec![]);
		let cmp = BigUInt::from_vec_le(vec![]);
		assert_eq!(from_empty_vec, cmp);

		let from_defective_vec = BigUInt::from_vec_le(vec![0u64, 0u64]);
		let cmp = BigUInt::from_vec_le(vec![]);
		assert_eq!(from_defective_vec, cmp);
	}
}
