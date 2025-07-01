use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex};
use core::str::FromStr;

use crate::SetVal;
use crate::biguint::BigUInt;
use crate::biguint::div::DivRem;
use crate::biguint::mul::MulTo;
use crate::error::ParseIntError;
use crate::util::{digit_to_ascii, parse_ascii_digit};

impl FromStr for BigUInt {
	type Err = ParseIntError;

	fn from_str(src: &str) -> Result<Self, Self::Err> {
		Self::from_ascii(src.as_bytes())
	}
}

impl Debug for BigUInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "BigUInt({self})")
	}
}

impl Display for BigUInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(true, "", &self.to_string_radix(10, false))
	}
}

impl LowerHex for BigUInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(true, "0x", &self.to_string_radix(16, false))
	}
}

impl UpperHex for BigUInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(true, "0x", &self.to_string_radix(16, true))
	}
}

impl Octal for BigUInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(true, "0o", &self.to_string_radix(8, false))
	}
}

impl Binary for BigUInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		f.pad_integral(true, "0b", &self.to_string_radix(2, false))
	}
}

impl BigUInt {
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

		if src[0] == b'-' {
			return Err(ParseIntError::Negative);
		}

		if src[0] == b'+' {
			src = src.split_at(1).1;
		}
		Self::parse_helper(src, radix)
	}

	pub(crate) fn parse_helper(src: &[u8], radix: u32) -> Result<Self, ParseIntError> {
		assert!((2..=36).contains(&radix), "radix must be between 2 and 36");

		if src.is_empty() {
			return Err(ParseIntError::Empty);
		}

		let mut res = Self::ZERO;

		let mut power_of_radix = BigUInt::from(1u64);

		// To reduce allocations
		let mut tmp = Self::ZERO;

		for &c in src.iter().rev() {
			let d = parse_ascii_digit(c).ok_or(ParseIntError::InvalidDigit)?;
			if d >= radix as u8 {
				return Err(ParseIntError::InvalidDigit);
			}
			let d = d.into();
			tmp.mul_to(&d, &power_of_radix);
			res += &tmp;
			tmp.mul_to(&power_of_radix, &radix.into());
			power_of_radix.set_val(&tmp);
		}
		Ok(res)
	}

	pub fn to_string_radix(&self, radix: u32, uppercase: bool) -> String {
		assert!((2..=36).contains(&radix), "radix must be between 2 and 36");

		if self.is_zero() {
			return "0".to_string();
		}

		let mut n = self.clone();
		let mut r = BigUInt::ZERO;
		let mut tmp = BigUInt::ZERO;
		let mut digits = Vec::new();
		let mut radix_big = BigUInt::from(radix);

		while !n.is_zero() {
			n.div_rem_to(&mut radix_big, &mut tmp, &mut r);
			n.set_val(&tmp);
			let digit = TryInto::<u8>::try_into(&r).unwrap();
			digits.push(digit_to_ascii(digit, uppercase));
		}

		digits.iter().rev().collect()
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::ToString;
	use core::assert_matches::assert_matches;

	use super::*;

	#[test]
	fn test_from_str_radix() {
		assert_eq!(
			BigUInt::from_str("435453453453123211").unwrap(),
			BigUInt::from(435453453453123211u64)
		);
		assert_eq!(
			BigUInt::from_str(&u128::MAX.to_string()).unwrap(),
			BigUInt::from(u128::MAX)
		);
		assert_eq!(
			BigUInt::from_str("+999999999").unwrap(),
			BigUInt::from(999999999u64)
		);
		assert_eq!(
			BigUInt::from_str_radix("acd56df", 16).unwrap(),
			BigUInt::from(0xacd56dfu64)
		);
		assert_matches!(BigUInt::from_str("-999999999"), Err(_));
		assert_matches!(BigUInt::from_str("684684g68486"), Err(_));
		assert_matches!(BigUInt::from_str(""), Err(_));
	}

	#[test]
	fn test_to_string_radix() {
		assert_eq!(
			"435453453453123211",
			BigUInt::from(435453453453123211u64).to_string_radix(10, false)
		);
		assert_eq!(
			u128::MAX.to_string(),
			BigUInt::from(u128::MAX).to_string_radix(10, false)
		);
		assert_eq!(
			"999999999",
			BigUInt::from(999999999u64).to_string_radix(10, false)
		);
		assert_eq!(
			"acd56df",
			BigUInt::from(0xacd56dfu64).to_string_radix(16, false)
		);
	}
}
