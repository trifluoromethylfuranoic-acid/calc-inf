use core::fmt::Display;
use core::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseIntError {
	Empty,
	InvalidDigit,
	Negative,
}

impl Display for ParseIntError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", match self {
			ParseIntError::Empty => "cannot parse integer from empty string",
			ParseIntError::InvalidDigit => "invalid digit found in string",
			ParseIntError::Negative => "can't construct BigUInt from a negative value",
		})
	}
}

impl Error for ParseIntError{}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct TryFromIntError;

impl Display for TryFromIntError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "can't construct BigUInt from a negative value")
	}
}

impl Error for TryFromIntError {}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct TryIntoIntError;

impl Display for TryIntoIntError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "out of range big integral type conversion attempted")
	}
}

impl Error for TryIntoIntError {}