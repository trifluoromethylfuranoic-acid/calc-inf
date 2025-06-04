use core::cmp::Ordering;

use crate::bigint::BigInt;
use crate::biguint::BigUInt;

impl PartialOrd for BigUInt {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for BigUInt {
	fn cmp(&self, other: &Self) -> Ordering {
		match Ord::cmp(&self.len(), &other.len()) {
			Ordering::Less => Ordering::Less,
			Ordering::Greater => Ordering::Greater,
			Ordering::Equal =>
			// rev because little-endian
			{
				Iterator::cmp(self.data.iter().rev(), other.data.iter().rev())
			}
		}
	}
}

impl PartialEq<BigInt> for BigUInt {
	fn eq(&self, other: &BigInt) -> bool {
		!other.is_negative() && *other.inner() == *self
	}
}

impl PartialOrd<BigInt> for BigUInt {
	fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
		if other.is_negative() {
			Some(Ordering::Greater)
		} else {
			self.partial_cmp(other.inner())
		}
	}
}

macro_rules! impl_partial_eq_u {
	($($t:ty),*) => {$(
		impl PartialEq<$t> for BigUInt {
			fn eq(&self, other: &$t) -> bool {
				match self.len() {
					0usize => *other == (0 as $t),
					1usize => self[0] == (*other as u64),
					_ => false
				}
			}
		}

		impl PartialEq<BigUInt> for $t {
			fn eq(&self, other: &BigUInt) -> bool {
				other == self
			}
		}
	)*}
}

impl_partial_eq_u! { u8, u16, u32, u64, usize }

impl PartialEq<u128> for BigUInt {
	fn eq(&self, other: &u128) -> bool {
		self == &BigUInt::from(*other)
	}
}

impl PartialEq<BigUInt> for u128 {
	fn eq(&self, other: &BigUInt) -> bool {
		other == self
	}
}

macro_rules! impl_partial_eq_i {
	($($i:ty => $u:ty),*) => {$(
		impl PartialEq<$i> for BigUInt {
			fn eq(&self, other: &$i) -> bool {
				if *other < 0 { false } else {
					*self == (*other as $u)
				}
			}
		}

		impl PartialEq<BigUInt> for $i {
			fn eq(&self, other: &BigUInt) -> bool {
				other == self
			}
		}
	)*}
}

impl_partial_eq_i! { i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128, isize => usize }

macro_rules! impl_partial_ord_u {
	($($t:ty),*) => {$(
		impl PartialOrd<$t> for BigUInt {
			fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
				Some(self.cmp(&BigUInt::from(*other)))
			}
		}

		impl PartialOrd<BigUInt> for $t {
			fn partial_cmp(&self, other: &BigUInt) -> Option<Ordering> {
				Some(BigUInt::from(*self).cmp(other))
			}
		}
	)*}
}

impl_partial_ord_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_partial_ord_i {
	($($t:ty),*) => {$(
		impl PartialOrd<$t> for BigUInt {
			fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
				match BigUInt::try_from(*other) {
					Ok(rhs) => self.partial_cmp(&rhs),
					Err(_) => Some(Ordering::Greater)
				}
			}
		}

		impl PartialOrd<BigUInt> for $t {
			fn partial_cmp(&self, other: &BigUInt) -> Option<Ordering> {
				match BigUInt::try_from(*self) {
					Ok(lhs) => lhs.partial_cmp(other),
					Err(_) => Some(Ordering::Less)
				}
			}
		}
	)*}
}

impl_partial_ord_i! { i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ord() {
		assert!(BigUInt::from(2846486u64) > 456);
		assert!(BigUInt::from(48646848448648623323234234348646846486u128) > 48648464);
		assert!(BigUInt::from(1u64) > -1);
		assert!(BigUInt::from(0u64) >= 0u64);
	}
}
