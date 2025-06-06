use core::cmp::Ordering;
use core::ops::{AddAssign, Neg, Sub, SubAssign};

use crate::bigint::BigInt;
use crate::biguint::BigUInt;

impl BigInt {
	pub fn neg_in_place(&mut self) {
		if !self.magnitude.is_zero() {
			self.is_negative = !self.is_negative;
		}
	}

	pub fn sub_from_assign(&mut self, lhs: &BigInt) {
		self.neg_in_place();
		*self += lhs;
	}

	pub fn sub_from_assign_u(&mut self, lhs: &BigUInt) {
		self.neg_in_place();
		*self += lhs;
	}
}

impl Neg for BigInt {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		self.neg_in_place();
		self
	}
}

impl SubAssign<&BigInt> for BigInt {
	fn sub_assign(&mut self, rhs: &BigInt) {
		self.neg_in_place();
		self.add_assign(rhs);
		self.neg_in_place();
	}
}

impl SubAssign<&BigUInt> for BigInt {
	fn sub_assign(&mut self, rhs: &BigUInt) {
		self.neg_in_place();
		self.add_assign(rhs);
		self.neg_in_place();
	}
}

impl SubAssign<&BigInt> for BigUInt {
	fn sub_assign(&mut self, rhs: &BigInt) {
		if rhs.is_negative() {
			*self += &rhs.magnitude;
		} else {
			match Ord::cmp(self, &rhs.magnitude) {
				Ordering::Less => {
					panic!("attempt to substruct with overflow");
				}
				Ordering::Equal => {
					self.set_zero();
				}
				Ordering::Greater => {
					*self -= &rhs.magnitude;
				}
			}
		}
	}
}

impl Sub<&BigInt> for BigInt {
	type Output = BigInt;

	fn sub(mut self, rhs: &BigInt) -> Self::Output {
		self -= rhs;
		self
	}
}

impl Sub<BigInt> for &BigInt {
	type Output = BigInt;

	fn sub(self, mut rhs: BigInt) -> Self::Output {
		rhs.sub_from_assign(self);
		rhs
	}
}

impl Sub<&BigUInt> for BigInt {
	type Output = BigInt;

	fn sub(mut self, rhs: &BigUInt) -> Self::Output {
		self -= rhs;
		self
	}
}

impl Sub<BigInt> for &BigUInt {
	type Output = BigInt;

	fn sub(self, mut rhs: BigInt) -> Self::Output {
		rhs.sub_from_assign_u(self);
		rhs
	}
}

impl Sub<&BigInt> for BigUInt {
	type Output = BigInt;

	fn sub(self, rhs: &BigInt) -> Self::Output {
		let lhs = BigInt::from(self);
		lhs - rhs
	}
}

impl Sub<BigUInt> for &BigInt {
	type Output = BigInt;

	fn sub(self, rhs: BigUInt) -> Self::Output {
		let rhs = BigInt::from(rhs);
		self - rhs
	}
}

macro_rules! impl_sub {
	($($t:ty),*) => {$(
		impl SubAssign<$t> for BigInt {
			fn sub_assign(&mut self, rhs: $t) {
				*self -= &BigInt::from(rhs);
			}
		}

		impl Sub<$t> for BigInt {
			type Output = BigInt;

			fn sub(self, rhs: $t) -> Self::Output {
				self - &BigInt::from(rhs)
			}
		}

		impl Sub<BigInt> for $t {
			type Output = BigInt;

			fn sub(self, rhs: BigInt) -> Self::Output {
				&BigInt::from(self) - rhs
			}
		}
	)*};
}

impl_sub! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bigint_subtraction() {
		let a = BigInt::from(100);
		let b = BigInt::from(30);
		assert_eq!(a - &b, BigInt::from(70));

		let a = BigInt::from(-100);
		let b = BigInt::from(30);
		assert_eq!(a - &b, BigInt::from(-130));

		let a = BigInt::from(100);
		let b = BigInt::from(-30);
		assert_eq!(a - &b, BigInt::from(130));
	}

	#[test]
	fn test_bigint_biguint_subtraction() {
		let a = BigInt::from(100);
		let b = BigUInt::from(30u32);
		assert_eq!(a - &b, BigInt::from(70));

		let a = BigUInt::from(100u32);
		let b = BigInt::from(30);
		assert_eq!(a - &b, BigInt::from(70));
	}

	#[test]
	fn test_bigint_primitive_subtraction() {
		let a = BigInt::from(100);
		assert_eq!(a.clone() - 30u32, BigInt::from(70));
		assert_eq!(a - (-30i32), BigInt::from(130));

		assert_eq!(100i32 - BigInt::from(30), BigInt::from(70));
		assert_eq!(100u32 - BigInt::from(30), BigInt::from(70));
	}

	#[test]
	#[should_panic]
	fn test_bigint_subtraction_overflow() {
		let mut a = BigUInt::from(30u32);
		let b = BigInt::from(100);
		a -= &b;
	}
}
