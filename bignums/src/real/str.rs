use alloc::string::String;

use crate::bigfloat::BigFloat;
use crate::error::ParseFloatError;
use crate::real::Real;

impl Real {
	pub fn from_string(s: String) -> Result<Self, ParseFloatError> {
		Real::from_string_radix(s, 10)
	}

	pub fn from_string_radix(s: String, radix: u32) -> Result<Self, ParseFloatError> {
		assert!((2..=36).contains(&radix), "radix must be between 2 and 36");

		let mut is_point_encountered = false;
		let mut iter = s.chars();

		let first = iter.next().ok_or(ParseFloatError::Empty)?;
		if !first.is_digit(radix) && first != '-' && first != '+' {
			return Err(ParseFloatError::InvalidDigit);
		}

		for c in iter {
			if !c.is_digit(radix) {
				if c == '.' {
					if is_point_encountered {
						return Err(ParseFloatError::InvalidDigit);
					}
					is_point_encountered = true;
				} else {
					return Err(ParseFloatError::InvalidDigit);
				}
			}
		}

		Ok(Real::new(move |prec| {
			BigFloat::from_str_radix_with_precision(&s, radix, prec).unwrap()
		}))
	}

	pub fn to_string(&self, prec: i64) -> String {
		self.to_string_radix(10, false, prec)
	}

	pub fn to_string_radix(&self, radix: u32, uppercase: bool, prec: i64) -> String {
		self.eval(prec).to_string_radix(radix, uppercase)
	}
}
