use crate::SetVal;
use crate::biguint::{BigUInt, DivRem};

impl BigUInt {
	pub fn gcd(self, other: BigUInt) -> BigUInt {
		let mut a = self;
		let mut b = other;

		let mut tmp1 = BigUInt::ZERO;
		let mut tmp2 = BigUInt::ZERO;
		let mut tmp3 = BigUInt::ZERO;

		while !b.is_zero() {
			tmp1.set_val(&b);
			(&mut a).div_rem_to(&mut b, &mut tmp3, &mut tmp2);
			b.set_val(&tmp2);
			a.set_val(&tmp1);
		}

		a
	}

	pub fn lcm(self, other: BigUInt) -> BigUInt {
		&mut (&self * &other) / &mut self.gcd(other)
	}

	pub fn factorial(&self) -> BigUInt {
		assert!(self.len() <= 1, "factorial is too big");
		let mut result = BigUInt::ONE;
		let mut i = 1u64;
		while i <= *self {
			result *= i;
			i += 1;
		}
		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_gcd() {
		let a = BigUInt::from(48u32);
		let b = BigUInt::from(18u32);
		assert_eq!(a.gcd(b), BigUInt::from(6u32));

		let a = BigUInt::from(54u32);
		let b = BigUInt::from(24u32);
		assert_eq!(a.gcd(b), BigUInt::from(6u32));

		let a = BigUInt::from(7u32);
		let b = BigUInt::from(13u32);
		assert_eq!(a.gcd(b), BigUInt::from(1u32));

		let a = BigUInt::from(0u32);
		let b = BigUInt::from(5u32);
		assert_eq!(a.gcd(b), BigUInt::from(5u32));

		let a = BigUInt::from(5u32);
		let b = BigUInt::from(0u32);
		assert_eq!(a.gcd(b), BigUInt::from(5u32));
	}

	#[test]
	fn test_factorial() {
		assert_eq!(BigUInt::from(0u32).factorial(), BigUInt::from(1u32));
		assert_eq!(BigUInt::from(1u32).factorial(), BigUInt::from(1u32));
		assert_eq!(BigUInt::from(2u32).factorial(), BigUInt::from(2u32));
		assert_eq!(BigUInt::from(3u32).factorial(), BigUInt::from(6u32));
		assert_eq!(BigUInt::from(4u32).factorial(), BigUInt::from(24u32));
		assert_eq!(BigUInt::from(5u32).factorial(), BigUInt::from(120u32));
	}
}
