use core::ops::{Div, Rem};

use crate::bigint::BigInt;
use crate::biguint::{BigUInt, DivRem};

impl BigInt {
	pub fn div_rem_floor(&mut self, d: &mut BigInt) -> (BigInt, BigInt) {
		let (mut q, mut r) = self.div_rem(&mut *d);
		if r.is_zero() {
			return (q, r);
		}

		if q.is_negative() {
			q -= 1;
			r += &*d;
		}

		(q, r)
	}
}

impl DivRem for &mut BigInt {
	type Output = BigInt;

	fn div_rem(self, d: &mut BigInt) -> (Self::Output, Self::Output) {
		let mut q = BigInt::ZERO;
		let mut r = BigInt::ZERO;
		self.div_rem_to(d, &mut q, &mut r);
		(q, r)
	}

	fn div_rem_to(self, d: &mut BigInt, q: &mut Self::Output, r: &mut Self::Output) {
		self.magnitude
			.div_rem_to(&mut d.magnitude, &mut q.magnitude, &mut r.magnitude);
		q.is_negative = self.is_negative() ^ d.is_negative();
		r.is_negative = self.is_negative();

		q.normalize();
		r.normalize();
	}
}

impl DivRem<&mut BigUInt> for &mut BigInt {
	type Output = BigInt;

	fn div_rem(self, d: &mut BigUInt) -> (Self::Output, Self::Output) {
		let mut q = BigInt::ZERO;
		let mut r = BigInt::ZERO;
		self.div_rem_to(d, &mut q, &mut r);
		(q, r)
	}

	fn div_rem_to(self, d: &mut BigUInt, q: &mut Self::Output, r: &mut Self::Output) {
		self.magnitude
			.div_rem_to(d, &mut q.magnitude, &mut r.magnitude);
		q.is_negative = self.is_negative();
		r.is_negative = self.is_negative();

		q.normalize();
		r.normalize();
	}
}

impl DivRem<&mut BigInt> for &mut BigUInt {
	type Output = BigInt;

	fn div_rem(self, d: &mut BigInt) -> (Self::Output, Self::Output) {
		let mut q = BigInt::ZERO;
		let mut r = BigInt::ZERO;
		self.div_rem_to(d, &mut q, &mut r);
		(q, r)
	}

	fn div_rem_to(self, d: &mut BigInt, q: &mut Self::Output, r: &mut Self::Output) {
		self.div_rem_to(&mut d.magnitude, &mut q.magnitude, &mut r.magnitude);
		q.is_negative = d.is_negative();
		r.is_negative = false;

		q.normalize();
		r.normalize();
	}
}

macro_rules! impl_div_rem_u {
	($($t:ty),*) => {$(
		impl DivRem<$t> for &mut BigInt {
			type Output = BigInt;

			fn div_rem(self, d: $t) -> (Self::Output, Self::Output) {
				let mut d = BigInt::from(d);
				self.div_rem(&mut d)
			}

			fn div_rem_to(self, d: $t, q: &mut Self::Output, r: &mut Self::Output) {
				let mut d = BigInt::from(d);
				self.div_rem_to(&mut d, q, r)
			}
		}

		impl DivRem<&BigInt> for $t {
			type Output = BigInt;

			fn div_rem(self, d: &BigInt) -> (Self::Output, Self::Output) {
				let mut q = BigInt::ZERO;
				let mut r = BigInt::ZERO;
				self.div_rem_to(d, &mut q, &mut r);
				(q, r)
			}

			fn div_rem_to(self, d: &BigInt, q: &mut Self::Output, r: &mut Self::Output) {
				self.div_rem_to(&d.magnitude, &mut q.magnitude, &mut r.magnitude);
				q.is_negative = d.is_negative();
				r.is_negative = false;

				q.normalize();
				r.normalize();
			}
		}
	)*};
}

impl_div_rem_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_div_rem_i {
	($($t:ty),*) => {$(
		impl DivRem<$t> for &mut BigInt {
			type Output = BigInt;

			fn div_rem(self, d: $t) -> (Self::Output, Self::Output) {
				let mut d = BigInt::from(d);
				self.div_rem(&mut d)
			}

			fn div_rem_to(self, d: $t, q: &mut Self::Output, r: &mut Self::Output) {
				let mut d = BigInt::from(d);
				self.div_rem_to(&mut d, q, r)
			}
		}

		impl DivRem<&BigInt> for $t {
			type Output = BigInt;

			fn div_rem(self, d: &BigInt) -> (Self::Output, Self::Output) {
				let mut q = BigInt::ZERO;
				let mut r = BigInt::ZERO;
				self.div_rem_to(d, &mut q, &mut r);
				(q, r)
			}

			fn div_rem_to(self, d: &BigInt, q: &mut Self::Output, r: &mut Self::Output) {
				self.abs().div_rem_to(&d.magnitude, &mut q.magnitude, &mut r.magnitude);
				q.is_negative = d.is_negative() ^ self.is_negative();
				r.is_negative = self.is_negative();

				q.normalize();
				r.normalize();
			}
		}
	)*};
}

impl_div_rem_i! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_div_and_rem {
	($(($t1:ty | $t2:ty)),*$(,)?) => {$(
		impl Div<$t2> for $t1 {
			type Output = BigInt;
			fn div(self, rhs: $t2) -> BigInt {
				self.div_rem(rhs).0
			}
		}

		impl Rem<$t2> for $t1 {
			type Output = BigInt;
			fn rem(self, rhs: $t2) -> BigInt {
				self.div_rem(rhs).1
			}
		}
	)*};
}

impl_div_and_rem! {
	(&mut BigInt | &mut BigInt),
	(&mut BigInt | u8),
	(&mut BigInt | u16),
	(&mut BigInt | u32),
	(&mut BigInt | u64),
	(&mut BigInt | u128),
	(&mut BigInt | usize),
	(&mut BigInt | i8),
	(&mut BigInt | i16),
	(&mut BigInt | i32),
	(&mut BigInt | i64),
	(&mut BigInt | i128),
	(&mut BigInt | isize),
	(u8          | &BigInt),
	(u16         | &BigInt),
	(u32         | &BigInt),
	(u64         | &BigInt),
	(u128        | &BigInt),
	(usize       | &BigInt),
	(i8          | &BigInt),
	(i16         | &BigInt),
	(i32         | &BigInt),
	(i64         | &BigInt),
	(i128        | &BigInt),
	(isize       | &BigInt),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bigint_div_rem() {
		let mut a = BigInt::from(100);
		let mut b = BigInt::from(30);
		let (q, r) = a.div_rem(&mut b);
		assert_eq!(q, BigInt::from(3));
		assert_eq!(r, BigInt::from(10));

		let mut a = BigInt::from(-100);
		let mut b = BigInt::from(30);
		let (q, r) = a.div_rem(&mut b);
		assert_eq!(q, BigInt::from(-3));
		assert_eq!(r, BigInt::from(-10));

		let mut a = BigInt::from(100);
		let mut b = BigInt::from(-30);
		let (q, r) = a.div_rem(&mut b);
		assert_eq!(q, BigInt::from(-3));
		assert_eq!(r, BigInt::from(10));

		let mut a = BigInt::from(-100);
		let mut b = BigInt::from(-30);
		let (q, r) = a.div_rem(&mut b);
		assert_eq!(q, BigInt::from(3));
		assert_eq!(r, BigInt::from(-10));
	}

	#[test]
	fn test_bigint_div_rem_unsigned() {
		let mut a = BigInt::from(100);
		let (q, r) = a.div_rem(30u64);
		assert_eq!(q, BigInt::from(3));
		assert_eq!(r, BigInt::from(10));

		let mut a = BigInt::from(-100);
		let (q, r) = a.div_rem(30u64);
		assert_eq!(q, BigInt::from(-3));
		assert_eq!(r, BigInt::from(-10));
	}

	#[test]
	fn test_bigint_div_rem_signed() {
		let mut a = BigInt::from(100);
		let (q, r) = a.div_rem(-30i64);
		assert_eq!(q, BigInt::from(-3));
		assert_eq!(r, BigInt::from(10));

		let mut a = BigInt::from(-100);
		let (q, r) = a.div_rem(-30i64);
		assert_eq!(q, BigInt::from(3));
		assert_eq!(r, BigInt::from(-10));
	}

	#[test]
	#[should_panic]
	fn test_bigint_div_by_zero() {
		let mut a = BigInt::from(100);
		let mut b = BigInt::ZERO;
		let _ = a.div_rem(&mut b);
	}
}
