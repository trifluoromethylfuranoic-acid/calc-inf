use core::cmp::Ordering;

use crate::bigint::BigInt;
use crate::biguint::BigUInt;

macro_rules! impl_partial_eq {
	($($t:ty),*) => {$(
		impl PartialEq<$t> for BigInt {
			fn eq(&self, other: &$t) -> bool {
				#[allow(clippy::cmp_owned)]
				let res = *self == BigInt::from(*other);
				res
			}
		}
		impl PartialEq<BigInt> for $t {
			fn eq(&self, other: &BigInt) -> bool {
				#[allow(clippy::cmp_owned)]
				let res = *other == BigInt::from(*self);
				res
			}
		}
	)*};
}

impl_partial_eq! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl PartialEq<BigUInt> for BigInt {
	fn eq(&self, other: &BigUInt) -> bool {
		!self.is_negative() && self.magnitude == *other
	}
}

impl PartialOrd for BigInt {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for BigInt {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.is_negative() && !other.is_negative() {
			Ordering::Less
		} else if !self.is_negative() && other.is_negative() {
			Ordering::Greater
		} else if self.is_negative() && other.is_negative() {
			Ord::cmp(&self.magnitude, &other.magnitude).reverse()
		} else
		/*if !self.is_negative() && !other.is_negative()*/
		{
			Ord::cmp(&self.magnitude, &other.magnitude)
		}
	}
}

impl PartialOrd<BigUInt> for BigInt {
	fn partial_cmp(&self, other: &BigUInt) -> Option<Ordering> {
		if self.is_negative() {
			Some(Ordering::Less)
		} else {
			self.magnitude.partial_cmp(other)
		}
	}
}

macro_rules! impl_partial_ord {
	($($t:ty),*) => {$(
		impl PartialOrd<$t> for BigInt {
			fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
				#[allow(clippy::cmp_owned)]
				return self.partial_cmp(&BigInt::from(*other));
			}
		}
		impl PartialOrd<BigInt> for $t {
			fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
				#[allow(clippy::cmp_owned)]
				return BigInt::from(*self).partial_cmp(other);
			}
		}
	)*};
}

impl_partial_ord! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
