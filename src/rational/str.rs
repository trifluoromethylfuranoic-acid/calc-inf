use core::str::FromStr;

use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::error::{ParseIntError, ParseRationalError};
use crate::rational::Rational;

impl Rational {
	pub fn from_decimal_str(src: &str) -> Result<Self, ParseRationalError> {
		Self::from_decimal_str_radix(src, 10)
	}

	pub fn from_decimal_str_radix(src: &str, radix: u32) -> Result<Self, ParseRationalError> {
		Self::from_decimal_ascii_radix(src.as_bytes(), radix)
	}

	pub fn from_decimal_ascii(src: &[u8]) -> Result<Self, ParseRationalError> {
		Self::from_decimal_ascii_radix(src, 10)
	}

	pub fn from_decimal_ascii_radix(src: &[u8], radix: u32) -> Result<Self, ParseRationalError> {
		todo!()
	}

	pub fn from_fraction_str(src: &str) -> Result<Self, ParseRationalError> {
		Self::from_fraction_str_radix(src, 10)
	}

	pub fn from_fraction_str_radix(src: &str, radix: u32) -> Result<Self, ParseRationalError> {
		Self::from_fraction_ascii_radix(src.as_bytes(), radix)
	}

	pub fn from_fraction_ascii(src: &[u8]) -> Result<Self, ParseRationalError> {
		Self::from_fraction_ascii_radix(src, 10)
	}

	pub fn from_fraction_ascii_radix(src: &[u8], radix: u32) -> Result<Self, ParseRationalError> {
		if let Some((n_str, d_str)) = src.split_once(|&c| c == b'/') {
			let n = BigInt::from_ascii(n_str).map_err(|e| e.to_rational_error())?;
			let d = BigUInt::from_ascii(d_str).map_err(|e| e.to_rational_error())?;
			if d.is_zero() {
				Err(ParseRationalError::DenominatorZero)
			} else {
				Ok(Self::new(n, d))
			}
		} else {
			Ok(Self::from(
				BigInt::from_ascii(src).map_err(|e| e.to_rational_error())?,
			))
		}
	}
}
