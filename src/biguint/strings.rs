use core::str::FromStr;
use crate::biguint::BigUInt;
use crate::error::ParseIntError;
use crate::SetVal;

impl FromStr for BigUInt {
	type Err = ParseIntError;

	fn from_str(src: &str) -> Result<Self, Self::Err> {
		Self::from_ascii(src.as_bytes())
	}
}

impl BigUInt {
	pub fn from_ascii(mut src: &[u8]) -> Result<Self, ParseIntError> {
		if src.is_empty() {
			return Err(ParseIntError::Empty);
		}

		if src[0] == b'-' {
			return Err(ParseIntError::Negative);
		}
		
		if src[0] == b'+' {
			src = src.split_at(1).1;
		}
		Self::parse_helper(src)
	}

	pub(crate) fn parse_helper(src: &[u8]) -> Result<Self, ParseIntError> {
		fn parse_ascii_digit(c: u8) -> Option<u8> {
			if !c.is_ascii_digit() {
				return None;
			}
			Some(c - b'0')
		}

		if src.is_empty() {
			return Err(ParseIntError::Empty);
		}
		
		// Magic number is approximate ratio between len of integer in base 10 and base 2^64
		let expected_cap = src.len() / 19 + 1;
		
		let mut res = Self::ZERO;
		res.data.grow(expected_cap);
		
		let mut power_of_10 = BigUInt::from(1u64);
		power_of_10.data.grow(expected_cap);
		
		// To reduce allocations
		let mut tmp = Self::ZERO;
		tmp.data.grow(expected_cap);
		
		for &c in src.iter().rev() {
			let d = parse_ascii_digit(c).ok_or(ParseIntError::InvalidDigit)?.into();
			tmp.mul_to(&d, &power_of_10);
			res += &tmp;
			tmp.mul_to(&power_of_10, &10u64.into());
			power_of_10.set_val(&tmp);
		}
		res.data.shrink_to_fit();
		Ok(res)
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::ToString;
	use core::assert_matches::assert_matches;
	use super::*;
	
	#[test]
	fn test_from_str() {
		assert_eq!(BigUInt::from_str("435453453453123211").unwrap(), BigUInt::from(435453453453123211u64));
		assert_eq!(BigUInt::from_str(&u128::MAX.to_string()).unwrap(), BigUInt::from(u128::MAX));
		assert_eq!(BigUInt::from_str("+999999999").unwrap(), BigUInt::from(999999999u64));
		assert_matches!(BigUInt::from_str("-999999999"), Err(_));
		assert_matches!(BigUInt::from_str("684684g68486"), Err(_));
		assert_matches!(BigUInt::from_str(""), Err(_));
	}
}