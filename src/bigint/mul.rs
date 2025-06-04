use core::ops::{Mul, MulAssign};
use crate::bigint::BigInt;
use crate::biguint::{BigUInt, MulTo};

impl BigInt {
	pub fn mul_to(&mut self, lhs: &BigInt, rhs: &BigInt) {
		if lhs.is_zero() || rhs.is_zero() {
			self.set_zero();
			return;
		}

		self.magnitude.mul_to(&lhs.magnitude, &rhs.magnitude);
		self.is_negative = lhs.is_negative() ^ rhs.is_negative();
	}

	pub fn mul_to_u(&mut self, lhs: &BigInt, rhs: &BigUInt) {
		if lhs.is_zero() || rhs.is_zero() {
			self.set_zero();
			return;
		}

		self.magnitude.mul_to(&lhs.magnitude, rhs);
		self.is_negative = lhs.is_negative();
	}
}

impl Mul<&BigInt> for &BigInt {
	type Output = BigInt;

	fn mul(self, rhs: &BigInt) -> Self::Output {
		let mut res = BigInt::ZERO;
		res.mul_to(self, rhs);
		res
	}
}

impl Mul<&BigUInt> for &BigInt {
	type Output = BigInt;

	fn mul(self, rhs: &BigUInt) -> Self::Output {
		let mut res = BigInt::ZERO;
		res.mul_to_u(self, rhs);
		res
	}
}

impl Mul<&BigInt> for &BigUInt {
	type Output = BigInt;

	fn mul(self, rhs: &BigInt) -> Self::Output {
		let mut res = BigInt::ZERO;
		res.mul_to_u(rhs, self);
		res
	}
}

impl MulAssign<&BigInt> for BigInt {
	fn mul_assign(&mut self, rhs: &BigInt) {
		let mut res = BigInt::ZERO;
		res.mul_to(self, rhs);
		*self = res;
	}
}

impl MulAssign<&BigUInt> for BigInt {
	fn mul_assign(&mut self, rhs: &BigUInt) {
		let mut res = BigInt::ZERO;
		res.mul_to_u(self, rhs);
		*self = res;
	}
}

impl MulAssign<&BigInt> for BigUInt {
	fn mul_assign(&mut self, rhs: &BigInt) {
		let mut res = BigInt::ZERO;
		res.mul_to_u(rhs, self);
		*self = res.try_into().expect("attempt to multiply with overflow");
	}
}

macro_rules! impl_mul {
	($($t:ty),*) => {$(
		impl MulAssign<$t> for BigInt {
			fn mul_assign(&mut self, rhs: $t) {
				*self *= &BigInt::from(rhs);
			}
		}
	
		impl Mul<$t> for &BigInt {
			type Output = BigInt;
			
			fn mul(self, rhs: $t) -> Self::Output {
				self * &BigInt::from(rhs)
			}
		}

		impl Mul<&BigInt> for $t {
			type Output = BigInt;
			
			fn mul(self, rhs: &BigInt) -> Self::Output {
				&BigInt::from(self) * rhs
			}
		}
	)*};
}

impl_mul! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bigint_mul() {
		let a = BigInt::from(100);
		let b = BigInt::from(200);
		assert_eq!(&a * &b, BigInt::from(20000));

		let a = BigInt::from(-100);
		let b = BigInt::from(200);
		assert_eq!(&a * &b, BigInt::from(-20000));

		let a = BigInt::from(-100);
		let b = BigInt::from(-200);
		assert_eq!(&a * &b, BigInt::from(20000));
		
		let a = BigInt::from(0);
		let b = BigInt::from(-200);
		assert_eq!(&a * &b, BigInt::from(0));
	}

	#[test]
	fn test_bigint_mul_biguint() {
		let a = BigInt::from(100);
		let b = BigUInt::from(200u32);
		assert_eq!(&a * &b, BigInt::from(20000));

		let a = BigInt::from(-100);
		let b = BigUInt::from(200u32);
		assert_eq!(&a * &b, BigInt::from(-20000));
	}

	#[test]
	fn test_bigint_mul_primitive() {
		let a = BigInt::from(100);
		assert_eq!(&a * 200i32, BigInt::from(20000));
		assert_eq!(&a * 200u32, BigInt::from(20000));
		assert_eq!(200i32 * &a, BigInt::from(20000));
		assert_eq!(200u32 * &a, BigInt::from(20000));

		let mut b = BigInt::from(100);
		b *= 200i32;
		assert_eq!(b, BigInt::from(20000));
	}
}



