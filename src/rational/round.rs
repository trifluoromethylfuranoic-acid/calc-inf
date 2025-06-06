use crate::bigint::BigInt;
use crate::biguint::DivRem;
use crate::rational::Rational;

impl Rational {
	pub fn floor(&mut self) -> Rational {
		Rational::from(self.floor_to_int())
	}

	pub fn floor_to_int(&mut self) -> BigInt {
		let (mut q, r) = (&mut self.n).div_rem(&mut self.d);
		if self.is_negative() && !r.is_zero() {
			q -= 1;
		}
		q
	}

	pub fn ceil(&mut self) -> Rational {
		Rational::from(self.ceil_to_int())
	}

	pub fn ceil_to_int(&mut self) -> BigInt {
		let (mut q, r) = (&mut self.n).div_rem(&mut self.d);
		if !self.is_negative() && !r.is_zero() {
			q += 1;
		}
		q
	}

	pub fn round(&mut self) -> Rational {
		Rational::from(self.round_to_int())
	}

	pub fn round_to_int(&mut self) -> BigInt {
		let (mut q, r) = (&mut self.n).div_rem(&mut self.d);
		if r.is_zero() {
			return q;
		}
		let r_times_2 = r.unsigned_abs() << 1;
		if r_times_2 >= self.d {
			q.magnitude += 1;
		}
		q
	}
}

#[cfg(test)]
mod tests {
	use alloc::vec;

	use super::*;
	use crate::biguint::BigUInt;

	#[test]
	fn test_floor() {
		let test_cases = vec![
			((2, 1), 2),   // 2/1 -> 2
			((5, 2), 2),   // 5/2 -> 2
			((-5, 2), -3), // -5/2 -> -3
			((7, 3), 2),   // 7/3 -> 2
			((-7, 3), -3), // -7/3 -> -3
			((-2, 1), -2), // -2/1 -> -2
			((-3, 1), -3), // -3/1 -> -3
		];

		for ((n, d), expected) in test_cases {
			let mut r = Rational::new(BigInt::from(n), BigUInt::try_from(d).unwrap());
			assert_eq!(r.floor_to_int(), BigInt::from(expected));
		}
	}

	#[test]
	fn test_ceil() {
		let test_cases = vec![
			((2, 1), 2),   // 2/1 -> 2
			((5, 2), 3),   // 5/2 -> 3
			((-5, 2), -2), // -5/2 -> -2
			((7, 3), 3),   // 7/3 -> 3
			((-7, 3), -2), // -7/3 -> -2
			((-2, 1), -2), // -2/1 -> -2
			((-3, 1), -3), // -3/1 -> -3
		];

		for ((n, d), expected) in test_cases {
			let mut r = Rational::new(BigInt::from(n), BigUInt::try_from(d).unwrap());
			assert_eq!(r.ceil_to_int(), BigInt::from(expected));
		}
	}

	#[test]
	fn test_round() {
		let test_cases = vec![
			((2, 1), 2),    // 2/1 -> 2
			((5, 2), 3),    // 5/2 -> 3
			((-5, 2), -3),  // -5/2 -> -3
			((7, 3), 2),    // 7/3 -> 2
			((-7, 3), -2),  // -7/3 -> -2
			((11, 3), 4),   // 11/3 -> 4
			((-11, 3), -4), // -11/3 -> -4
		];

		for ((n, d), expected) in test_cases {
			let mut r = Rational::new(BigInt::from(n), BigUInt::try_from(d).unwrap());
			assert_eq!(r.round_to_int(), BigInt::from(expected));
		}
	}
}
