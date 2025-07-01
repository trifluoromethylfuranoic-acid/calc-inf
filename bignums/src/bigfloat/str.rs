use alloc::string::{String, ToString};
use core::fmt::{Debug, Display, Formatter};
use core::ops::Mul;
use core::str::FromStr;

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::ParseFloatError;
use crate::util::digit_to_ascii;

impl FromStr for BigFloat {
	type Err = ParseFloatError;

	fn from_str(src: &str) -> Result<Self, Self::Err> {
		Self::from_str_radix(src, 10)
	}
}

impl Display for BigFloat {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.to_string_radix(10, false))
	}
}

impl BigFloat {
	pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseFloatError> {
		Self::from_ascii_radix(src.as_bytes(), radix)
	}

	pub fn from_ascii(src: &[u8]) -> Result<Self, ParseFloatError> {
		Self::from_ascii_radix(src, 10)
	}

	pub fn from_ascii_radix(src: &[u8], radix: u32) -> Result<Self, ParseFloatError> {
		let (_, fract) = src.split_once(|c| *c == b'.').unwrap_or((src, b""));
		let prec = (fract.len() as i64 + 16) * (radix.ilog2() as i64 + 1);
		Self::from_ascii_radix_with_precision(src, radix, prec)
	}

	pub fn from_str_with_precision(src: &str, prec: i64) -> Result<Self, ParseFloatError> {
		Self::from_str_radix_with_precision(src, 10, prec)
	}

	pub fn from_str_radix_with_precision(
		src: &str,
		radix: u32,
		prec: i64,
	) -> Result<Self, ParseFloatError> {
		Self::from_ascii_radix_with_precision(src.as_bytes(), radix, prec)
	}

	pub fn from_ascii_with_precision(src: &[u8], prec: i64) -> Result<Self, ParseFloatError> {
		Self::from_ascii_radix_with_precision(src, 10, prec)
	}

	pub fn from_ascii_radix_with_precision(
		src: &[u8],
		radix: u32,
		prec: i64,
	) -> Result<Self, ParseFloatError> {
		let (mut whole, fract) = src.split_once(|c| *c == b'.').unwrap_or((src, b"0"));

		let is_negative = whole.get(0).copied() == Some(b'-');

		let whole_i = BigInt::from_ascii_radix(whole, radix).map_err(|e| e.to_float_error())?;
		let fract_i = BigUInt::from_ascii_radix(fract, radix).map_err(|e| e.to_float_error())?;

		let whole_f = BigFloat::from(whole_i);
		if fract_i.is_zero() {
			return Ok(whole_f);
		}
		let mut fract_f = BigFloat::from(fract_i);

		fract_f.set_sign(is_negative);

		let fract_d = BigUInt::from(radix).pow(fract.len() as u64).into();

		let fract_final = fract_f.div(&fract_d, prec + 16);
		let mut res = whole_f.add_with_precision(&fract_final, prec + 16);

		res.round_to_precision(prec);
		Ok(res)
	}

	pub fn to_string_radix(&self, radix: u32, uppercase: bool) -> String {
		assert!((2..=36).contains(&radix), "radix must be between 2 and 36");

		let mut s = String::new();

		let (whole, mut fract) = self.trunc_fract();

		if self.is_negative() {
			s.push('-');
		}

		s.push_str(&whole.abs().to_string_radix(radix, uppercase));
		if !fract.is_zero() {
			s.push('.');

			let radix_f = BigFloat::from(radix);

			while !fract.is_zero() {
				let whole;
				(whole, fract) = fract.mul(&radix_f).trunc_fract();
				let digit = u8::try_from(&whole).unwrap();
				s.push(digit_to_ascii(digit, uppercase));
			}
		}

		s
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::ToString;

	use super::*;

	#[test]
	fn test_from_str() {
		// Power of 2
		let a = BigFloat::from_str("0.125").unwrap();
		assert_eq!(a, BigFloat::try_from(0.125).unwrap());

		test_from_str_helper("132.13215", 132.13215, 0);
		test_from_str_helper("132.13215", 132.13215, 13);
		test_from_str_helper("0.65548448948", 0.65548448948, 0);
		test_from_str_helper("0.65548448948", 0.65548448948, 16);
		test_from_str_helper("-4546454", -4546454.0, 0);
		test_from_str_helper("-4546454", -4546454.0, 16);
		test_from_str_helper("-0.65548448948", -0.65548448948, 0);
		test_from_str_helper("-0.65548448948", -0.65548448948, 16);
		test_from_str_helper("-4546454.567", -4546454.567, 0);
		test_from_str_helper("-4546454.567", -4546454.567, 16);
	}

	fn test_from_str_helper(s: &str, f: f64, prec: i64) {
		let a = BigFloat::from_str_with_precision(s, prec).unwrap();
		let b = BigFloat::try_from(f).unwrap();
		let delta = (&a - &b).abs();
		let epsilon = BigFloat::ONE >> prec;

		print!("a={a:?}\nb={b:?}\nepsilon={epsilon:?}\ndelta={delta:?}\n\n");

		assert!(delta < epsilon);
	}

	#[test]
	fn test_to_string_radix() {
		// Test decimal
		assert_eq!(BigFloat::ZERO.to_string_radix(10, false), "0");
		assert_eq!(
			BigFloat::from_str("123")
				.unwrap()
				.to_string_radix(10, false),
			"123"
		);
		assert_eq!(
			BigFloat::from_str("-123")
				.unwrap()
				.to_string_radix(10, false),
			"-123"
		);
		assert_eq!(
			BigFloat::from_str("0.125")
				.unwrap()
				.to_string_radix(10, false),
			"0.125"
		);
		assert_eq!(
			BigFloat::from_str("-0.125")
				.unwrap()
				.to_string_radix(10, false),
			"-0.125"
		);
		assert_eq!(
			BigFloat::from_str("-123.125")
				.unwrap()
				.to_string_radix(10, false),
			"-123.125"
		);

		// Test hexadecimal
		assert_eq!(
			BigFloat::from_str("16").unwrap().to_string_radix(16, false),
			"10"
		);
		assert_eq!(
			BigFloat::from_str("16").unwrap().to_string_radix(16, true),
			"10"
		);
		assert_eq!(
			BigFloat::from_str("255")
				.unwrap()
				.to_string_radix(16, false),
			"ff"
		);
		assert_eq!(
			BigFloat::from_str("255").unwrap().to_string_radix(16, true),
			"FF"
		);
		assert_eq!(
			BigFloat::from_str("-255")
				.unwrap()
				.to_string_radix(16, false),
			"-ff"
		);
		assert_eq!(
			BigFloat::from_str("-255")
				.unwrap()
				.to_string_radix(16, true),
			"-FF"
		);
		assert_eq!(
			BigFloat::from_str("0.5")
				.unwrap()
				.to_string_radix(16, false),
			"0.8"
		);
		assert_eq!(
			BigFloat::from_str("-0.5")
				.unwrap()
				.to_string_radix(16, false),
			"-0.8"
		);
	}
}
