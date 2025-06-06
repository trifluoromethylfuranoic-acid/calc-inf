use core::error::Error;
use core::fmt::Display;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ParseIntError {
	Empty,
	InvalidDigit,
	Negative,
}

impl ParseIntError {
	pub(crate) fn to_rational_error(&self) -> ParseRationalError {
		match self {
			ParseIntError::Empty => ParseRationalError::Empty,
			ParseIntError::InvalidDigit => ParseRationalError::InvalidDigit,
			ParseIntError::Negative => ParseRationalError::InvalidDigit,
		}
	}
}

impl Display for ParseIntError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				ParseIntError::Empty => "cannot parse from empty string",
				ParseIntError::InvalidDigit => "invalid digit found in string",
				ParseIntError::Negative => "can't construct BigUInt from a negative value",
			}
		)
	}
}

impl Error for ParseIntError {}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ParseRationalError {
	Empty,
	InvalidDigit,
	DenominatorZero,
}

impl Display for ParseRationalError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				ParseRationalError::Empty => "cannot parse from empty string",
				ParseRationalError::InvalidDigit => "invalid digit found in string",
				ParseRationalError::DenominatorZero => "cannot divide by zero",
			}
		)
	}
}

impl Error for ParseRationalError {}
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub struct TryFromIntError;

impl Display for TryFromIntError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "can't construct BigUInt from a negative value")
	}
}

impl Error for TryFromIntError {}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub struct TryIntoIntError;

impl Display for TryIntoIntError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "out of range big integral type conversion attempted")
	}
}

impl Error for TryIntoIntError {}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TryFromFloatError {
	NaN,
	Infinite,
}

impl Display for TryFromFloatError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				TryFromFloatError::NaN => {
					"NaN is not a valid floating-point number"
				}
				TryFromFloatError::Infinite => {
					"infinity is not a valid floating-point number"
				}
			}
		)
	}
}
