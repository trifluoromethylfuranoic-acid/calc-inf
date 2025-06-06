use alloc::string::{String, ToString};
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex};
use core::str::FromStr;

use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::ParseIntError;

impl FromStr for BigInt {
	type Err = ParseIntError;

	fn from_str(src: &str) -> Result<Self, Self::Err> {
		Self::from_ascii(src.as_bytes())
	}
}

impl Debug for BigInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "BigInt({self})")
	}
}

impl Display for BigInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(
			!self.is_negative(),
			"",
			&self.magnitude.to_string_radix(10, false),
		)
	}
}

impl LowerHex for BigInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(
			!self.is_negative(),
			"0x",
			&self.magnitude.to_string_radix(16, false),
		)
	}
}

impl UpperHex for BigInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(
			!self.is_negative(),
			"0x",
			&self.magnitude.to_string_radix(16, true),
		)
	}
}

impl Octal for BigInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(
			!self.is_negative(),
			"0o",
			&self.magnitude.to_string_radix(8, false),
		)
	}
}

impl Binary for BigInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(
			!self.is_negative(),
			"0b",
			&self.magnitude.to_string_radix(2, false),
		)
	}
}

impl BigInt {
	pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
		Self::from_ascii_radix(src.as_bytes(), radix)
	}

	pub fn from_ascii(src: &[u8]) -> Result<Self, ParseIntError> {
		Self::from_ascii_radix(src, 10)
	}

	pub fn from_ascii_radix(mut src: &[u8], radix: u32) -> Result<Self, ParseIntError> {
		if src.is_empty() {
			return Err(ParseIntError::Empty);
		}

		let mut is_negative = false;

		if src[0] == b'-' {
			is_negative = true;
			src = src.split_at(1).1;
		}

		if src[0] == b'+' {
			src = src.split_at(1).1;
		}
		let mag = BigUInt::parse_helper(src, radix)?;
		Ok(Self::from_sign_and_magnitude(is_negative, mag))
	}

	pub fn to_string_radix(&self, radix: u32, uppercase: bool) -> String {
		let mut res = if self.is_negative() {
			"-".to_string()
		} else {
			String::new()
		};
		res += &self.magnitude.to_string_radix(radix, uppercase);
		res
	}
}

#[cfg(test)]
mod tests {
	use core::assert_matches::assert_matches;

	use super::*;

	#[test]
	fn test_from_str_radix() {
		assert_eq!(
			BigInt::from_str("435453453453123211").unwrap(),
			BigInt::from(435453453453123211i64)
		);
		assert_eq!(
			BigInt::from_str("-435453453453123211").unwrap(),
			BigInt::from(-435453453453123211i64)
		);
		assert_eq!(
			BigInt::from_str("+999999999").unwrap(),
			BigInt::from(999999999i64)
		);
		assert_eq!(
			BigInt::from_str_radix("acd56df", 16).unwrap(),
			BigInt::from(0xacd56dfi64)
		);
		assert_eq!(
			BigInt::from_str_radix("-acd56df", 16).unwrap(),
			BigInt::from(-0xacd56dfi64)
		);
		assert_matches!(BigInt::from_str("684684g68486"), Err(_));
		assert_matches!(BigInt::from_str(""), Err(_));
	}

	#[test]
	fn test_to_string_radix() {
		assert_eq!(
			"435453453453123211",
			BigInt::from(435453453453123211i64).to_string_radix(10, false)
		);
		assert_eq!(
			"-435453453453123211",
			BigInt::from(-435453453453123211i64).to_string_radix(10, false)
		);
		assert_eq!(
			"999999999",
			BigInt::from(999999999i64).to_string_radix(10, false)
		);
		assert_eq!(
			"-999999999",
			BigInt::from(-999999999i64).to_string_radix(10, false)
		);
		assert_eq!(
			"acd56df",
			BigInt::from(0xacd56dfi64).to_string_radix(16, false)
		);
		assert_eq!(
			"-acd56df",
			BigInt::from(-0xacd56dfi64).to_string_radix(16, false)
		);
	}
}
