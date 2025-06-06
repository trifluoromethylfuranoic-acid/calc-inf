mod add;
mod cmp;
mod convert;
mod div;
mod mul;
mod set_val;
mod sub;

use crate::bigint::BigInt;
use crate::biguint::BigUInt;

#[derive(Debug)]
/// Dynamic, arbitrary-sized rational type
pub struct Rational {
	// Invariant: d != 0
	n: BigInt,
	d: BigUInt,
}

impl Rational {
	pub const ZERO: Self = Self {
		n: BigInt::ZERO,
		d: BigUInt::ONE,
	};
	pub const ONE: Self = Self {
		n: BigInt::ONE,
		d: BigUInt::ONE,
	};
	pub const NEG_ONE: Self = Self {
		n: BigInt::NEG_ONE,
		d: BigUInt::ONE,
	};

	pub fn new(n: BigInt, d: BigUInt) -> Self {
		assert!(!d.is_zero(), "denominator must not be zero");
		Self { n, d }
	}

	pub fn numerator(&self) -> &BigInt {
		&self.n
	}

	pub fn numerator_mut(&mut self) -> &mut BigInt {
		&mut self.n
	}

	pub fn denominator(&self) -> &BigUInt {
		&self.d
	}

	pub unsafe fn denominator_mut(&mut self) -> &mut BigUInt {
		&mut self.d
	}

	pub fn inner(&self) -> (&BigInt, &BigUInt) {
		(&self.n, &self.d)
	}

	pub unsafe fn inner_mut(&mut self) -> (&mut BigInt, &mut BigUInt) {
		(&mut self.n, &mut self.d)
	}

	pub fn into_inner(self) -> (BigInt, BigUInt) {
		(self.n, self.d)
	}

	pub fn is_zero(&self) -> bool {
		self.n.is_zero()
	}

	pub fn set_zero(&mut self) {
		self.n.set_zero();
	}

	pub fn is_one(&self) -> bool {
		self.n == self.d
	}

	pub fn set_one(&mut self) {
		self.n.set_one();
		self.d.set_one();
	}

	pub fn is_negative(&self) -> bool {
		self.n.is_negative()
	}

	pub fn abs_in_place(&mut self) {
		self.n.abs_in_place();
	}

	pub fn abs(mut self) -> Self {
		self.abs_in_place();
		self
	}
}
