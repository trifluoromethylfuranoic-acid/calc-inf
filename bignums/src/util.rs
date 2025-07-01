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

/// Converts f32 to (is_negative, mantissa, exponent)
/// Returns Err for NaN and infinities
/// In the resulting mantissa the binary point is after the leading 1 bit (currently always at position 9)
/// The exponent is debiased
pub(crate) fn f32_to_parts(val: f32) -> Result<(bool, u32, i32), TryFromFloatError> {
	// f32: 1-bit sign, 8-bits exponent (biased by 127), 23-bits mantissa (leading 1 not stored)
	const MANTISSA_MASK: u32 = (1u32 << 23) - 1u32;
	const MANTISSA_MISSING_ONE_MASK: u32 = 1u32 << 23;
	const EXPONENT_MASK: u32 = (1u32 << 8) - 1u32;
	const EXPONENT_DEBIAS: i32 = -127i32;

	match val.classify() {
		FpCategory::Nan => Err(TryFromFloatError::NaN),
		FpCategory::Infinite => Err(TryFromFloatError::Infinite),
		FpCategory::Zero => Ok((false, 0, 0)),
		FpCategory::Subnormal => {
			let bits = val.to_bits();
			let is_negative = val.is_sign_negative();
			let mut exp = EXPONENT_DEBIAS + 1;
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
			let exp = ((bits >> 23) & EXPONENT_MASK) as i32 + EXPONENT_DEBIAS;
			let mant = bits & MANTISSA_MASK | MANTISSA_MISSING_ONE_MASK;
			Ok((is_negative, mant, exp))
		}
	}
}

/// Converts f64 to (is_negative, mantissa, exponent)
/// Returns Err for NaN and infinities
/// In the resulting mantissa the binary point is after the leading 1 bit (currently always at position 12)
/// The exponent is debiased
pub(crate) fn f64_to_parts(val: f64) -> Result<(bool, u64, i64), TryFromFloatError> {
	// f64: 1-bit sign, 11-bits exponent (biased by 1023), 52-bits mantissa (leading 1 not stored)
	const MANTISSA_MASK: u64 = (1u64 << 52) - 1u64;
	const MANTISSA_MISSING_ONE_MASK: u64 = 1u64 << 52;
	const EXPONENT_MASK: u64 = (1u64 << 11) - 1u64;
	const EXPONENT_DEBIAS: i64 = -1023i64;

	match val.classify() {
		FpCategory::Nan => Err(TryFromFloatError::NaN),
		FpCategory::Infinite => Err(TryFromFloatError::Infinite),
		FpCategory::Zero => Ok((false, 0, 0)),
		FpCategory::Subnormal => {
			let bits = val.to_bits();
			let is_negative = val.is_sign_negative();
			let mut exp = EXPONENT_DEBIAS + 1;
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
			let exp = ((bits >> 52) & EXPONENT_MASK) as i64 + EXPONENT_DEBIAS;
			let mant = bits & MANTISSA_MASK | MANTISSA_MISSING_ONE_MASK;
			Ok((is_negative, mant, exp))
		}
	}
}

/// Assembles a float from parts
/// Returns infinity for large exponents and zero for small ones
/// Treats the mantissa as if the binary point was after the leading 1 bit
/// The exponent is debiased
pub(crate) fn f32_from_parts(is_negative: bool, mut mant: u32, exp: i32) -> f32 {
	// f32: 1-bit sign, 8-bits exponent (biased by 127), 23-bits mantissa (leading 1 not stored)
	const MAX_EXPONENT: i32 = 127;
	const MIN_NORMAL_EXPONENT: i32 = -126;
	const MIN_EXPONENT: i32 = -149;
	const MANTISSA_MASK: u32 = (1u32 << 23) - 1u32;
	const EXPONENT_BIAS: i32 = 127i32;

	if mant == 0 {
		return if is_negative { -0.0_f32 } else { 0.0_f32 };
	}

	// Put leading 1 bit at position 9
	let shift = mant.leading_zeros() as i32 - 8i32;
	if shift > 0 {
		mant <<= shift;
	} else {
		mant >>= -shift;
	}

	if exp > MAX_EXPONENT {
		if is_negative {
			f32::NEG_INFINITY
		} else {
			f32::INFINITY
		}
	} else if exp < MIN_EXPONENT {
		if is_negative { -0.0_f32 } else { 0.0_f32 }
	} else if exp < MIN_NORMAL_EXPONENT {
		let shift = MIN_NORMAL_EXPONENT - exp;
		let mant_corrected = (mant >> shift) & MANTISSA_MASK;
		let sign = if is_negative { 1u32 } else { 0u32 } << 31;
		let bits = sign | mant_corrected;
		f32::from_bits(bits)
	} else {
		let mant_corrected = mant & MANTISSA_MASK;
		let exp_corrected = ((exp + EXPONENT_BIAS) << 23) as u32;
		let sign = if is_negative { 1u32 } else { 0u32 } << 31;
		let bits = sign | exp_corrected | mant_corrected;
		f32::from_bits(bits)
	}
}

/// Assembles a float from parts
/// Returns infinity for large exponents and zero for small ones
/// Treats the mantissa as if the binary point was after the leading 1 bit
/// The exponent is debiased
pub(crate) fn f64_from_parts(is_negative: bool, mut mant: u64, exp: i64) -> f64 {
	// f64: 1-bit sign, 11-bits exponent (biased by 1023), 52-bits mantissa (leading 1 not stored)
	const MAX_EXPONENT: i64 = 1023;
	const MIN_NORMAL_EXPONENT: i64 = -1022;
	const MIN_EXPONENT: i64 = -1074;
	const MANTISSA_MASK: u64 = (1u64 << 52) - 1u64;
	const EXPONENT_BIAS: i64 = 1023i64;

	if mant == 0 {
		return if is_negative { -0.0_f64 } else { 0.0_f64 };
	}

	let shift = mant.leading_zeros() as i64 - 11i64;
	if shift > 0 {
		mant <<= shift;
	} else {
		mant >>= -shift;
	}

	if exp > MAX_EXPONENT {
		if is_negative {
			f64::NEG_INFINITY
		} else {
			f64::INFINITY
		}
	} else if exp < MIN_EXPONENT {
		if is_negative { -0.0_f64 } else { 0.0_f64 }
	} else if exp < MIN_NORMAL_EXPONENT {
		let shift = MIN_NORMAL_EXPONENT - exp;
		let mant_corrected = (mant >> shift) & MANTISSA_MASK;
		let sign = if is_negative { 1u64 } else { 0u64 } << 63;
		let bits = sign | mant_corrected;
		f64::from_bits(bits)
	} else {
		let mant_corrected = mant & MANTISSA_MASK;
		let exp_corrected = ((exp + EXPONENT_BIAS) << 52) as u64;
		let sign = if is_negative { 1u64 } else { 0u64 } << 63;
		let bits = sign | exp_corrected | mant_corrected;
		f64::from_bits(bits)
	}
}

pub(crate) fn digit_to_ascii(d: u8, uppercase: bool) -> char {
	match d {
		0..=9 => (b'0' + d) as char,
		10..=35 => ((if uppercase { b'A' } else { b'a' }) + (d - 10)) as char,
		_ => panic!("invalid digit for radix"),
	}
}

pub(crate) fn parse_ascii_digit(c: u8) -> Option<u8> {
	match c {
		b'0'..=b'9' => Some(c - b'0'),
		b'a'..=b'z' => Some(c - b'a' + 10),
		b'A'..=b'Z' => Some(c - b'A' + 10),
		_ => None,
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
	fn test_f32_to_parts_normal() {
		let (sign, mant, exp) = f32_to_parts(3.14159).unwrap();
		assert!(!sign);
		assert_eq!(exp, 1);
		assert_eq!(mant, 0b1100_1001_0000_1111_1101_0000);
	}

	#[test]
	fn test_f64_to_parts_normal() {
		let (sign, mant, exp) = f64_to_parts(3.14159).unwrap();
		assert!(!sign);
		assert_eq!(exp, 1);
		assert_eq!(
			mant,
			0b1_1001_0010_0001_1111_1001_1111_0000_0001_1011_1000_0110_0110_1110
		);
	}

	#[test]
	fn test_f32_from_parts_normal() {
		let test_values = [1.0f32, -1.0f32, 3.14159f32, -3.14159f32];
		for val in test_values {
			let (sign, mant, exp) = f32_to_parts(val).unwrap();
			let res = f32_from_parts(sign, mant, exp);
			assert_eq!(res, val);
		}
	}

	#[test]
	fn test_f32_from_parts_subnormal() {
		let test_values = [f32::from_bits(1), f32::from_bits(0x007FFFFF)];
		for val in test_values {
			let (sign, mant, exp) = f32_to_parts(val).unwrap();
			let res = f32_from_parts(sign, mant, exp);
			assert_eq!(res, val);
		}
	}

	#[test]
	fn test_f32_from_parts_special_cases() {
		assert_eq!(f32_from_parts(false, 0, 0), 0.0f32);
		assert_eq!(f32_from_parts(true, 0, 0), -0.0f32);
		assert_eq!(f32_from_parts(false, 1, 128), f32::INFINITY);
		assert_eq!(f32_from_parts(true, 1, 128), f32::NEG_INFINITY);
	}

	#[test]
	fn test_f64_from_parts_normal() {
		let test_values = [1.0f64, -1.0f64, 3.14159f64, -3.14159f64];
		for val in test_values {
			let (sign, mant, exp) = f64_to_parts(val).unwrap();
			let res = f64_from_parts(sign, mant, exp);
			assert_eq!(res, val);
		}
	}

	#[test]
	fn test_f64_from_parts_subnormal() {
		let test_values = [f64::from_bits(1), f64::from_bits(0x000FFFFFFFFFFFFF)];
		for val in test_values {
			let (sign, mant, exp) = f64_to_parts(val).unwrap();
			let res = f64_from_parts(sign, mant, exp);
			assert_eq!(res, val);
		}
	}

	#[test]
	fn test_f64_from_parts_special_cases() {
		assert_eq!(f64_from_parts(false, 0, 0), 0.0f64);
		assert_eq!(f64_from_parts(true, 0, 0), -0.0f64);
		assert_eq!(f64_from_parts(false, 1, 1024), f64::INFINITY);
		assert_eq!(f64_from_parts(true, 1, 1024), f64::NEG_INFINITY);
	}
}
