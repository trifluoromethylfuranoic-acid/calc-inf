mod add;
mod bits;
mod cmp;
mod consts;
mod convert;
mod div;
mod log;
mod misc;
mod mul;
mod pow;
mod round;
mod set_val;
mod sqrt;
mod str;
mod sub;

use crate::bigint::BigInt;

/// An arbitrary precision floating-point number.
#[derive(Debug, Hash)]
pub struct BigFloat {
	// Representation: m * 2^e.
	// The mantissa is normalized, i.e., it has no trailing zeros.
	// Zero is represented by m = 0 and e = 0. m is never zero otherwise.
	m: BigInt,
	e: i64,
}

impl BigFloat {
	pub const ZERO: Self = Self {
		m: BigInt::ZERO,
		e: 0,
	};

	pub const ONE: Self = Self {
		m: BigInt::ONE,
		e: 0,
	};

	pub const NEG_ONE: Self = Self {
		m: BigInt::NEG_ONE,
		e: 0,
	};

	pub fn from_mantissa_exponent(mantissa: BigInt, exponent: i64) -> Self {
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

	pub fn exponent(&self) -> i64 {
		self.e
	}

	pub fn inner(&self) -> (&BigInt, i64) {
		(&self.m, self.e)
	}

	pub unsafe fn inner_mut(&mut self) -> (&mut BigInt, &mut i64) {
		(&mut self.m, &mut self.e)
	}

	pub fn into_inner(self) -> (BigInt, i64) {
		(self.m, self.e)
	}

	pub fn is_zero(&self) -> bool {
		self.m.is_zero()
	}

	pub fn set_zero(&mut self) {
		self.m.set_zero();
		self.e = 0;
	}

	pub fn is_one(&self) -> bool {
		self.m.is_one() && self.e == 0
	}

	pub fn set_one(&mut self) {
		self.m.set_one();
		self.e = 0;
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

	pub fn is_integer(&self) -> bool {
		!self.e.is_negative()
	}

	fn normalize(&mut self) {
		if self.m.is_zero() {
			self.e = 0;
		} else {
			let trailing_zeros = self.m.magnitude.trailing_zeros();
			self.e = self.e.strict_add(i64::try_from(trailing_zeros).unwrap());
			self.m.magnitude >>= trailing_zeros;
		}
	}
}

impl Default for BigFloat {
	fn default() -> Self {
		Self::ZERO
	}
}
