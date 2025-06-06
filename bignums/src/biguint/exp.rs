use crate::biguint::{BigUInt, MulTo};
use crate::SetVal;

impl BigUInt {
	pub fn pow(&self, mut power: u64) -> Self {
		let mut tmp = BigUInt::ZERO;
		let mut power_of_self = self.clone();
		let mut res = BigUInt::ONE;
		while power != 0 {
			if power & 1 == 1 {
				tmp.mul_to(&res, &power_of_self);
				res.set_val(&tmp);
			}
			power >>= 1;
			
			tmp.mul_to(&power_of_self, &power_of_self);
			power_of_self.set_val(&tmp);
		}
		
		res
	}

	/// Returns log2(self) if self is a power of 2, otherwise None.
	pub fn ilog2_exact(&self) -> Option<u64> {
		let hi = self.data.last().copied()?;
		if hi.count_ones() != 1 {
			return None;
		}
		for &digit in self.data.iter().rev().skip(1) {
			if digit != 0 {
				return None;
			}
		}
		Some(hi.trailing_zeros() as u64 + (self.len() as u64 - 1u64) * (u64::BITS as u64))
	}

	pub fn ilog2(&self) -> u64 {
		let hi = self.data.last().copied().expect("attempt to take ilog2(0)");
		(u64::BITS as u64 - hi.leading_zeros() as u64 - 1u64)
			+ (self.len() as u64 - 1u64) * (u64::BITS as u64)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ilog2_exact() {
		assert_eq!(BigUInt::from(1u64).ilog2_exact(), Some(0));
		assert_eq!(BigUInt::from(2u64).ilog2_exact(), Some(1));
		assert_eq!(BigUInt::from(4u64).ilog2_exact(), Some(2));
		assert_eq!(BigUInt::from(8u64).ilog2_exact(), Some(3));
		assert_eq!(BigUInt::from(16u64).ilog2_exact(), Some(4));
		assert_eq!(BigUInt::from(3u64).ilog2_exact(), None);
		assert_eq!(BigUInt::from(6u64).ilog2_exact(), None);
		assert_eq!(BigUInt::from(10u64).ilog2_exact(), None);
		assert_eq!(BigUInt::from(1u64 << 63).ilog2_exact(), Some(63));
		assert_eq!(BigUInt::from(1u64 << 32).ilog2_exact(), Some(32));
		assert_eq!(
			(&BigUInt::from(1u64 << 63) * &BigUInt::from(2u64)).ilog2_exact(),
			Some(64)
		);
		assert_eq!(
			(&BigUInt::from(1u64 << 63) * &BigUInt::from(4u64)).ilog2_exact(),
			Some(65)
		);
		assert_eq!(
			(&BigUInt::from(1u64 << 63) * &BigUInt::from(3u64)).ilog2_exact(),
			None
		);
		assert_eq!(
			(&BigUInt::from(1u64 << 32) * &BigUInt::from(5u64)).ilog2_exact(),
			None
		);
	}

	#[test]
	fn test_ilog2() {
		assert_eq!(BigUInt::from(1u64).ilog2(), 0);
		assert_eq!(BigUInt::from(2u64).ilog2(), 1);
		assert_eq!(BigUInt::from(3u64).ilog2(), 1);
		assert_eq!(BigUInt::from(4u64).ilog2(), 2);
		assert_eq!(BigUInt::from(5u64).ilog2(), 2);
		assert_eq!(BigUInt::from(8u64).ilog2(), 3);
		assert_eq!(BigUInt::from(9u64).ilog2(), 3);
		assert_eq!(BigUInt::from(1u64 << 63).ilog2(), 63);
		assert_eq!(
			(&BigUInt::from(1u64 << 63) * &BigUInt::from(2u64)).ilog2(),
			64
		);
	}

	#[test]
	fn test_pow() {
		assert_eq!(BigUInt::from(2u64).pow(0), BigUInt::from(1u64));
		assert_eq!(BigUInt::from(2u64).pow(1), BigUInt::from(2u64));
		assert_eq!(BigUInt::from(2u64).pow(2), BigUInt::from(4u64));
		assert_eq!(BigUInt::from(2u64).pow(3), BigUInt::from(8u64));
		assert_eq!(BigUInt::from(3u64).pow(4), BigUInt::from(81u64));
		assert_eq!(BigUInt::from(5u64).pow(2), BigUInt::from(25u64));
	}
}
