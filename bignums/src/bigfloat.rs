mod convert;
mod set_val;

use crate::bigint::BigInt;

/// An arbitrary precision floating-point number.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BigFloat {
	// Representation: m * 2^e.
	// The implicit binary point in the mantissa is after the leading 1 bit.
	// The mantissa is normalized, i.e., it has no trailing zeros.
	// Zero is represented by m = 0 and e = 0. m is never zero otherwise.
	m: BigInt,
	e: BigInt,
}

impl BigFloat {
	pub const ZERO: Self = Self {
		m: BigInt::ZERO,
		e: BigInt::ZERO,
	};

	pub const ONE: Self = Self {
		m: BigInt::ONE,
		e: BigInt::ZERO,
	};

	pub const NEG_ONE: Self = Self {
		m: BigInt::NEG_ONE,
		e: BigInt::ZERO,
	};

	pub fn from_mantissa_exponent(mantissa: BigInt, exponent: BigInt) -> Self {
		let mut res = Self {
			m: mantissa,
			e: exponent,
		};
		res.normalize();
		res
	}

	pub fn mantissa(&self) -> &BigInt {
		&self.m
	}

	pub fn exponent(&self) -> &BigInt {
		&self.e
	}

	pub fn inner(&self) -> (&BigInt, &BigInt) {
		(&self.m, &self.e)
	}

	pub unsafe fn inner_mut(&mut self) -> (&mut BigInt, &mut BigInt) {
		(&mut self.m, &mut self.e)
	}

	pub fn into_inner(self) -> (BigInt, BigInt) {
		(self.m, self.e)
	}

	pub fn is_zero(&self) -> bool {
		self.m.is_zero()
	}

	pub fn set_zero(&mut self) {
		self.m.set_zero();
		self.e.set_zero();
	}

	pub fn is_one(&self) -> bool {
		self.m.is_one() && self.e.is_zero()
	}

	pub fn set_one(&mut self) {
		self.m.set_one();
		self.e.set_zero();
	}

	pub fn is_negative(&self) -> bool {
		self.m.is_negative()
	}

	pub fn is_positive(&self) -> bool {
		self.m.is_positive()
	}

	pub fn set_sign(&mut self, is_negative: bool) {
		self.m.set_sign(is_negative);
	}

	pub fn abs_in_place(&mut self) {
		self.m.abs_in_place();
	}

	pub fn abs(mut self) -> Self {
		self.abs_in_place();
		self
	}

	fn normalize(&mut self) {
		if self.m.is_zero() {
			self.e.set_zero();
		} else {
			let trailing_zeros = self.m.magnitude.trailing_zeros();
			self.e += trailing_zeros;
			self.m >>= trailing_zeros;
		}
	}
}
