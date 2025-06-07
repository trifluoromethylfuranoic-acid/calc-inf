use core::iter;
use core::num::FpCategory;

use smallvec::SmallVec;

macro_rules! x_to_ys_le {
	($($name:ident: $big:ty => $small:ty),*) => {$(
		#[allow(dead_code)]
		pub(crate) fn $name(val: $big) -> [$small; 2] {
			let arr: [$small; 2] = unsafe { core::mem::transmute(val) };
			#[cfg(target_endian = "big")]
				return [arr[1], arr[0]];
			#[cfg(target_endian = "little")]
				return arr;
		}
	)*}
}

x_to_ys_le! { u128_to_u64s: u128 => u64, i128_to_i64s: i128 => i64, i128_to_u64s: i128 => u64 }

macro_rules! xs_to_y_le {
	($($name:ident: $small:ty => $big:ty),*) => {$(
		#[allow(dead_code)]
		pub(crate) fn $name(val: [$small; 2]) -> $big {
			#[cfg(target_endian = "big")]
				{ (val[0], val[1]) = (val[1], val[0]); }
			unsafe { core::mem::transmute(val) }
		})*
	}
}

xs_to_y_le! { u64s_to_u128: u64 => u128, i64s_to_i128: i64 => i128, u64s_to_i128: i64 => u128 }

pub(crate) trait VecExt<T>
where
	T: Default + Clone,
{
	fn get_or_default(&self, idx: usize) -> T;
	fn set_or_insert(&mut self, idx: usize, val: T);
	fn extend_by(&mut self, by: usize, val: T);
	fn extend_zero(&mut self, by: usize) {
		self.extend_by(by, T::default());
	}
	fn set_len_fill(&mut self, new_len: usize, val: T);
	fn set_len_fill_zero(&mut self, new_len: usize) {
		self.set_len_fill(new_len, T::default());
	}
}

impl<T, const N: usize> VecExt<T> for SmallVec<[T; N]>
where
	T: Default + Clone,
{
	fn get_or_default(&self, idx: usize) -> T {
		self.get(idx).cloned().unwrap_or_default()
	}

	fn set_or_insert(&mut self, idx: usize, val: T) {
		if idx < self.len() {
			self[idx] = val;
		} else {
			self.extend_zero(idx - self.len());
			self.push(val);
		}
	}

	fn extend_by(&mut self, by: usize, val: T) {
		self.grow(self.len() + by);
		self.extend(iter::repeat_n(val, by));
	}

	fn set_len_fill(&mut self, new_len: usize, val: T) {
		if new_len < self.len() {
			self.truncate(new_len);
			self.fill(val);
		} else {
			self.fill(val.clone());
			self.extend_by(new_len - self.len(), val);
		}
	}
}

pub(crate) fn carrying_mul(lhs: u64, rhs: u64) -> (u64, u64) {
	const MASK: u64 = 0x00000000ffffffff;
	let (a, b, c, d);

	a = (lhs & MASK) * (rhs & MASK);
	b = (lhs >> 32) * (rhs & MASK);
	c = (lhs & MASK) * (rhs >> 32);
	d = (lhs >> 32) * (rhs >> 32);

	let (lo, carry1) = u64::overflowing_add(a, b << 32);
	let (lo, carry2) = u64::overflowing_add(lo, c << 32);
	let hi = d + (b >> 32) + (c >> 32) + (carry1 as u64) + (carry2 as u64);
	(lo, hi)
}

pub(crate) fn f32_parts(val: f32) -> Result<(bool, u32, i32), TryFromFloatError> {
	// f32: 1-bit sign, 8-bits exponent (biased by 127), 23-bits mantissa (leading 1 not stored)
	const MANTISSA_MASK: u32 = (1u32 << 23) - 1u32;
	const MANTISSA_MISSING_ONE_MASK: u32 = 1u32 << 23;
	const EXPONENT_MASK: u32 = (1u32 << 8) - 1u32;
	const EXPONENT_BIAS: i32 = -127i32;

	match val.classify() {
		FpCategory::Nan => Err(TryFromFloatError::NaN),
		FpCategory::Infinite => Err(TryFromFloatError::Infinite),
		FpCategory::Zero => Ok((false, 0, 0)),
		FpCategory::Subnormal => {
			let bits = val.to_bits();
			let is_negative = val.is_sign_negative();
			let mut exp = EXPONENT_BIAS + 1;
			let mut mant = bits & MANTISSA_MASK;
			// we shift so the implicit fixed point is after the leading 1 and correct
			// the exponent respectively
			let shift = mant.leading_zeros() - 8;
			mant <<= shift;
			exp -= shift as i32;
			Ok((is_negative, mant, exp))
		}
		FpCategory::Normal => {
			let bits = val.to_bits();
			let is_negative = val.is_sign_negative();
			let exp = ((bits >> 23) & EXPONENT_MASK) as i32 + EXPONENT_BIAS;
			let mant = bits & MANTISSA_MASK | MANTISSA_MISSING_ONE_MASK;
			Ok((is_negative, mant, exp))
		}
	}
}

pub(crate) fn f64_parts(val: f64) -> Result<(bool, u64, i64), TryFromFloatError> {
	// f64: 1-bit sign, 11-bits exponent (biased by 1023), 52-bits mantissa (leading 1 not stored)
	const MANTISSA_MASK: u64 = (1u64 << 52) - 1u64;
	const MANTISSA_MISSING_ONE_MASK: u64 = 1u64 << 52;
	const EXPONENT_MASK: u64 = (1u64 << 11) - 1u64;
	const EXPONENT_BIAS: i64 = -1023i64;

	match val.classify() {
		FpCategory::Nan => Err(TryFromFloatError::NaN),
		FpCategory::Infinite => Err(TryFromFloatError::Infinite),
		FpCategory::Zero => Ok((false, 0, 0)),
		FpCategory::Subnormal => {
			let bits = val.to_bits();
			let is_negative = val.is_sign_negative();
			let mut exp = EXPONENT_BIAS + 1;
			let mut mant = bits & MANTISSA_MASK;
			// we shift so the implicit fixed point is after the leading 1 and correct
			// the exponent respectively
			let shift = mant.leading_zeros() - 11;
			mant <<= shift;
			exp -= shift as i64;
			Ok((is_negative, mant, exp))
		}
		FpCategory::Normal => {
			let bits = val.to_bits();
			let is_negative = val.is_sign_negative();
			let exp = ((bits >> 52) & EXPONENT_MASK) as i64 + EXPONENT_BIAS;
			let mant = bits & MANTISSA_MASK | MANTISSA_MISSING_ONE_MASK;
			Ok((is_negative, mant, exp))
		}
	}
}

#[cfg(test)]
use crate::biguint::BigUInt;
use crate::error::TryFromFloatError;

#[cfg(test)]
pub(crate) fn to_foreign_biguint(a: BigUInt) -> num_bigint::BigUint {
	let data = a.into_bytes_le();
	num_bigint::BigUint::from_bytes_le(&data)
}

#[cfg(test)]
pub(crate) fn from_foreign_biguint(a: num_bigint::BigUint) -> BigUInt {
	let data = a.to_bytes_le();
	BigUInt::from_bytes_le(data)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_carrying_mul() {
		assert_eq!(
			carrying_mul(u64::MAX, u64::MAX),
			(1u64, 0xffff_ffff_ffff_fffe_u64)
		);
		carrying_mul_helper(45646544848615, 8468481531548);
		carrying_mul_helper(783735, 453422222222222225);
		carrying_mul_helper(1111111111111111, 999999999999999999);
		carrying_mul_helper(u64::MAX, u64::MAX);
	}

	fn carrying_mul_helper(a: u64, b: u64) {
		let res_u128 = (a as u128) * (b as u128);
		let (x, y) = carrying_mul(a, b);
		let res = u64s_to_u128([x, y]);
		assert_eq!(res, res_u128);
	}

	#[test]
	fn test_f32_parts_normal() {
		let (sign, mant, exp) = f32_parts(3.14159).unwrap();
		assert!(!sign);
		assert_eq!(exp, 1);
		assert_eq!(mant, 0b1100_1001_0000_1111_1101_0000);
	}

	#[test]
	fn test_f64_parts_normal() {
		let (sign, mant, exp) = f64_parts(3.14159).unwrap();
		assert!(!sign);
		assert_eq!(exp, 1);
		assert_eq!(
			mant,
			0b1_1001_0010_0001_1111_1001_1111_0000_0001_1011_1000_0110_0110_1110
		);
	}
}
