use core::cmp::Ordering;

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::rational::Rational;

impl BigFloat {
	pub fn cmp_abs(&self, other: &BigFloat) -> Ordering {
		match (self.is_zero(), other.is_zero()) {
			(true, true) => Ordering::Equal,
			(true, false) => Ordering::Less,
			(false, true) => Ordering::Greater,
			(false, false) => cmp_abs_non_zero(self, other),
		}
	}
}

impl PartialEq for BigFloat {
	fn eq(&self, other: &Self) -> bool {
		self.e == other.e && self.m == other.m
	}
}

impl Eq for BigFloat {}

impl PartialEq<BigInt> for BigFloat {
	fn eq(&self, other: &BigInt) -> bool {
		if !self.is_integer() {
			false
		} else if self.is_negative() != other.is_negative() {
			false
		} else if Some(other.magnitude.ilog2())
			!= (self.e as u64).checked_add(self.m.magnitude.ilog2())
		{
			false
		} else {
			*self == BigFloat::from(other)
		}
	}
}

impl PartialEq<BigFloat> for BigInt {
	fn eq(&self, other: &BigFloat) -> bool {
		other == self
	}
}

impl PartialEq<BigUInt> for BigFloat {
	fn eq(&self, other: &BigUInt) -> bool {
		if !self.is_integer() {
			false
		} else if self.is_negative() {
			false
		} else if Some(other.ilog2()) != (self.e as u64).checked_add(self.m.magnitude.ilog2()) {
			false
		} else {
			*self == BigFloat::from(other)
		}
	}
}

impl PartialEq<BigFloat> for BigUInt {
	fn eq(&self, other: &BigFloat) -> bool {
		other == self
	}
}

macro_rules! impl_partial_eq {
	($($t:ty),*) => {$(
		impl PartialEq<$t> for BigFloat {
			fn eq(&self, other: &$t) -> bool {
				self == &BigFloat::from(*other)
			}
		}

		impl PartialEq<BigFloat> for $t {
			fn eq(&self, other: &BigFloat) -> bool {
				other == self
			}
		}
	)*};
}

impl_partial_eq! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl PartialEq<Rational> for BigFloat {
	fn eq(&self, other: &Rational) -> bool {
		if self.is_negative() != other.is_negative() {
			return false;
		}
		if self.is_zero() {
			return other.is_zero();
		}
		if other.is_zero() {
			return self.is_zero();
		}

		let mut other = other.clone();
		other.reduce();
		let Some(log_d) = other.denominator().ilog2_exact() else {
			return false;
		};

		if self.e.is_negative() {
			if self.e != -(log_d as i64) {
				return false;
			}
			self.m.magnitude == other.numerator().magnitude
		} else {
			if !other.denominator().is_one() {
				return false;
			}
			if self.e as u64 + self.m.magnitude.ilog2() != other.n.magnitude.ilog2() {
				return false;
			}
			let m = self.m.clone() << self.e;
			m == other.n
		}
	}
}

impl PartialEq<BigFloat> for Rational {
	fn eq(&self, other: &BigFloat) -> bool {
		other == self
	}
}

impl PartialOrd for BigFloat {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for BigFloat {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self.is_negative(), other.is_negative()) {
			(false, false) => self.cmp_abs(other),
			(false, true) => Ordering::Greater,
			(true, false) => Ordering::Less,
			(true, true) => cmp_abs_non_zero(self, other).reverse(),
		}
	}
}

impl PartialOrd<BigInt> for BigFloat {
	fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
		Some(self.cmp(&BigFloat::from(other)))
	}
}

impl PartialOrd<BigFloat> for BigInt {
	fn partial_cmp(&self, other: &BigFloat) -> Option<Ordering> {
		Some(BigFloat::from(self).cmp(other))
	}
}

impl PartialOrd<BigUInt> for BigFloat {
	fn partial_cmp(&self, other: &BigUInt) -> Option<Ordering> {
		Some(self.cmp(&BigFloat::from(other)))
	}
}

impl PartialOrd<BigFloat> for BigUInt {
	fn partial_cmp(&self, other: &BigFloat) -> Option<Ordering> {
		Some(BigFloat::from(self).cmp(other))
	}
}

macro_rules! impl_partial_ord {
	($($t:ty),*) => {$(
		impl PartialOrd<$t> for BigFloat {
			fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
				self.partial_cmp(&BigFloat::from(*other))
			}
		}

		impl PartialOrd<BigFloat> for $t {
			fn partial_cmp(&self, other: &BigFloat) -> Option<Ordering> {
				BigFloat::from(*self).partial_cmp(other)
			}
		}
	)*};
}

impl_partial_ord! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

fn cmp_abs_non_zero(a: &BigFloat, b: &BigFloat) -> Ordering {
	let a_e_real = (a.e as i128) + (a.m.magnitude.ilog2() as i128);
	let b_e_real = (b.e as i128) + (b.m.magnitude.ilog2() as i128);
	Ord::cmp(&a_e_real, &b_e_real).then_with(|| match Ord::cmp(&a.e, &b.e) {
		Ordering::Less => {
			let b_m = b.m.magnitude.clone() << (b.e - a.e);
			Ord::cmp(&a.m.magnitude, &b_m)
		}
		Ordering::Equal => Ord::cmp(&a.m.magnitude, &b.m.magnitude),
		Ordering::Greater => {
			let a_m = a.m.magnitude.clone() << (a.e - b.e);
			Ord::cmp(&a_m, &b.m.magnitude)
		}
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_eq() {
		let a = BigFloat::from(123);
		let b = BigFloat::from(123);
		let c = BigFloat::from(124);
		assert_eq!(a, b);
		assert_ne!(a, c);
	}

	#[test]
	fn test_eq_bigint() {
		let a = BigFloat::from(123);
		let b = BigInt::from(123);
		let c = BigInt::from(124);
		assert_eq!(a, b);
		assert_eq!(b, a);
		assert_ne!(a, c);
		assert_ne!(c, a);
	}

	#[test]
	fn test_eq_biguint() {
		let a = BigFloat::from(123u32);
		let b = BigUInt::from(123u32);
		let c = BigUInt::from(124u32);
		assert_eq!(a, b);
		assert_eq!(b, a);
		assert_ne!(a, c);
		assert_ne!(c, a);
	}

	#[test]
	fn test_ord() {
		let a = BigFloat::from(123);
		let c = BigFloat::from(124);
		let d = BigFloat::from(-123);
		assert!(a < c);
		assert!(c > a);
		assert!(d < a);
		assert!(a > d);
	}

	#[test]
	fn test_eq_rational() {
		let a = BigFloat::ZERO;
		let b = Rational::ZERO;
		assert_eq!(a, b);
		assert_eq!(b, a);

		let a = BigFloat::from(123);
		let b = Rational::from(123);
		assert_eq!(a, b);
		assert_eq!(b, a);

		let a = BigFloat::from(-123);
		let b = Rational::from(-123);
		assert_eq!(a, b);
		assert_eq!(b, a);

		let a = BigFloat::from_mantissa_exponent(BigInt::from(1), -1);
		let b = Rational::new(BigInt::from(1), BigUInt::from(2u32));
		assert_eq!(a, b);
		assert_eq!(b, a);

		let a = BigFloat::from_mantissa_exponent(BigInt::from(-1), -1);
		let b = Rational::new(BigInt::from(-1), BigUInt::from(2u32));
		assert_eq!(a, b);
		assert_eq!(b, a);
	}
}
