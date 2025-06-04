use core::cmp::Ordering;
use core::ops::{Add, AddAssign};

use crate::bigint::BigInt;
use crate::biguint::BigUInt;

impl AddAssign<&BigUInt> for BigInt {
	fn add_assign(&mut self, other: &BigUInt) {
		if !self.is_negative() {
			self.magnitude += other;
		} else {
			match Ord::cmp(&self.magnitude, other) {
				Ordering::Less => {
					self.magnitude.checked_sub_from_assign(other);
					self.is_negative = false;
				}
				Ordering::Equal => {
					self.set_zero();
				}
				Ordering::Greater => {
					self.magnitude -= other;
				}
			}
		}
	}
}

impl AddAssign<&BigInt> for BigUInt {
	fn add_assign(&mut self, other: &BigInt) {
		if !other.is_negative() {
			*self += &other.magnitude;
		} else {
			match Ord::cmp(self, &other.magnitude) {
				Ordering::Less => {
					panic!("attempt to add with overflow");
				}
				Ordering::Equal => {
					self.set_zero();
				}
				Ordering::Greater => {
					*self -= &other.magnitude;
				}
			}
		}
	}
}

impl AddAssign<&BigInt> for BigInt {
	fn add_assign(&mut self, other: &BigInt) {
		if self.is_negative() == other.is_negative() {
			self.magnitude += &other.magnitude;
		} else {
			match Ord::cmp(&self.magnitude, &other.magnitude) {
				Ordering::Less => {
					self.magnitude.checked_sub_from_assign(&other.magnitude);
					self.is_negative = other.is_negative();
				}
				Ordering::Equal => {
					self.set_zero();
				}
				Ordering::Greater => {
					self.magnitude -= &other.magnitude;
				}
			}
		}
	}
}

impl Add<&BigInt> for BigInt {
	type Output = BigInt;

	fn add(mut self, rhs: &BigInt) -> Self::Output {
		self += rhs;
		self
	}
}

impl Add<BigInt> for &BigInt {
	type Output = BigInt;

	fn add(self, mut rhs: BigInt) -> Self::Output {
		rhs += self;
		rhs
	}
}

impl Add<&BigUInt> for BigInt {
	type Output = BigInt;

	fn add(mut self, rhs: &BigUInt) -> Self::Output {
		self += rhs;
		self
	}
}

impl Add<BigInt> for &BigUInt {
	type Output = BigInt;

	fn add(self, mut rhs: BigInt) -> Self::Output {
		rhs += self;
		rhs
	}
}

impl Add<BigUInt> for &BigInt {
	type Output = BigInt;

	fn add(self, rhs: BigUInt) -> Self::Output {
		let rhs = BigInt::from(rhs);
		rhs + self
	}
}

impl Add<&BigInt> for BigUInt {
	type Output = BigInt;

	fn add(self, rhs: &BigInt) -> Self::Output {
		let lhs = BigInt::from(self);
		lhs + rhs
	}
}

macro_rules! impl_add {
	($($t:ty),*) => {$(
		impl AddAssign<$t> for BigInt {
			fn add_assign(&mut self, rhs: $t) {
				*self += &BigInt::from(rhs);
			}
		}

		impl Add<$t> for BigInt {
			type Output = BigInt;

			fn add(self, rhs: $t) -> Self::Output {
				self + &BigInt::from(rhs)
			}
		}

		impl Add<BigInt> for $t {
			type Output = BigInt;

			fn add(self, rhs: BigInt) -> Self::Output {
				rhs + &BigInt::from(self)
			}
		}
	)*};
}

impl_add! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }