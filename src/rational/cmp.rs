use core::cmp::Ordering;

use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::rational::Rational;

impl PartialEq for Rational {
	fn eq(&self, other: &Self) -> bool {
		&self.n * &other.d == &other.n * &self.d
	}
}

impl Eq for Rational {}

impl PartialEq<BigUInt> for Rational {
	fn eq(&self, other: &BigUInt) -> bool {
		!self.is_negative() && self.n == &self.d * other
	}
}

impl PartialEq<Rational> for BigUInt {
	fn eq(&self, other: &Rational) -> bool {
		!other.is_negative() && other.n == &other.d * self
	}
}

impl PartialEq<BigInt> for Rational {
	fn eq(&self, other: &BigInt) -> bool {
		self.is_negative() == other.is_negative() && self.n == &self.d * other
	}
}

impl PartialEq<Rational> for BigInt {
	fn eq(&self, other: &Rational) -> bool {
		self.is_negative() == other.is_negative() && other.n == &other.d * self
	}
}

macro_rules! impl_partial_eq {
	($($t:ty),*) => {$(
		impl PartialEq<$t> for Rational {
			fn eq(&self, other: &$t) -> bool {
				self.n == &self.d * *other
			}
		}

		impl PartialEq<Rational> for $t {
			fn eq(&self, other: &Rational) -> bool {
				other.n == &other.d * *self
			}
		}
	)*}
}

impl_partial_eq! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl PartialOrd for Rational {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Rational {
	fn cmp(&self, other: &Self) -> Ordering {
		Ord::cmp(&(&self.n * &other.d), &(&other.n * &self.d))
	}
}

impl PartialOrd<BigUInt> for Rational {
	fn partial_cmp(&self, other: &BigUInt) -> Option<Ordering> {
		if self.is_negative() {
			Some(Ordering::Less)
		} else {
			PartialOrd::partial_cmp(&self.n, &(&self.d * other))
		}
	}
}

impl PartialOrd<Rational> for BigUInt {
	fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
		if other.is_negative() {
			Some(Ordering::Greater)
		} else {
			PartialOrd::partial_cmp(&(&other.d * self), &other.n)
		}
	}
}

impl PartialOrd<BigInt> for Rational {
	fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
		Some(match (self.is_negative(), other.is_negative()) {
			(false, false) | (true, true) => Ord::cmp(&self.n, &(&self.d * other)),
			(true, false) => Ordering::Less,
			(false, true) => Ordering::Greater,
		})
	}
}

impl PartialOrd<Rational> for BigInt {
	fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
		Some(match (self.is_negative(), other.is_negative()) {
			(false, false) | (true, true) => Ord::cmp(&(&other.d * self), &other.n),
			(true, false) => Ordering::Less,
			(false, true) => Ordering::Greater,
		})
	}
}

macro_rules! impl_partial_ord {
	($($t:ty),*) => {$(
		impl PartialOrd<$t> for Rational {
			fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
				PartialOrd::partial_cmp(&self.n, &(&self.d * *other))
			}
		}

		impl PartialOrd<Rational> for $t {
			fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
				PartialOrd::partial_cmp(&(&other.d * *self), &other.n)
			}
		}
	)*}
}

impl_partial_ord! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rational_eq() {
		let r1 = Rational::new(BigInt::from(1), BigUInt::from(2u64));
		let r2 = Rational::new(BigInt::from(2), BigUInt::from(4u64));
		let r3 = Rational::new(BigInt::from(1), BigUInt::from(3u64));
		assert_eq!(r1, r2);
		assert_ne!(r1, r3);
	}

	#[test]
	fn test_rational_cmp() {
		let r1 = Rational::new(BigInt::from(1), BigUInt::from(2u64));
		let r2 = Rational::new(BigInt::from(2), BigUInt::from(3u64));
		let r3 = Rational::new(BigInt::from(-1), BigUInt::from(2u64));
		assert!(r1 < r2);
		assert!(r3 < r1);
		assert!(r2 > r1);
	}

	#[test]
	fn test_rational_primitive_cmp() {
		let r = Rational::new(BigInt::from(5), BigUInt::from(1u64));
		assert_eq!(r, 5u8);
		assert_eq!(r, 5i32);
		assert!(r > 4i64);
		assert!(r < 6u32);
	}

	#[test]
	fn test_rational_bigint_cmp() {
		let r = Rational::new(BigInt::from(3), BigUInt::from(1u64));
		let bi = BigInt::from(3);
		assert_eq!(r, bi);
		assert!(r > BigInt::from(2));
		assert!(r < BigInt::from(4));
	}

	#[test]
	fn test_rational_biguint_cmp() {
		let r = Rational::new(BigInt::from(7), BigUInt::from(1u64));
		let bu = BigUInt::from(7u32);
		assert_eq!(r, bu);
		assert!(r > BigUInt::from(6u32));
		assert!(r < BigUInt::from(8u32));
	}
}
