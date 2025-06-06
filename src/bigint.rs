mod add;
mod bits;
mod cmp;
mod convert;
mod div;
mod mul;
mod set_val;
mod str;
mod sub;

use core::ops::Index;

use crate::biguint::BigUInt;

/// Dynamic, arbitrary-sized signed integer type
#[derive(Eq, PartialEq, Hash)]
pub struct BigInt {
	// Invariant: if magnitude == 0, is_negative should be false
	is_negative: bool,
	magnitude: BigUInt,
}

impl BigInt {
	pub const ZERO: Self = Self {
		is_negative: false,
		magnitude: BigUInt::ZERO,
	};
	pub const ONE: Self = Self {
		is_negative: false,
		magnitude: BigUInt::ONE,
	};
	pub const NEG_ONE: Self = Self {
		is_negative: true,
		magnitude: BigUInt::ONE,
	};

	pub fn from_sign_and_magnitude(mut is_negative: bool, magnitude: BigUInt) -> Self {
		if magnitude.is_zero() {
			is_negative = false;
		}
		Self {
			is_negative,
			magnitude,
		}
	}

	/// Length of underlying storage, in units of mem::sizeof::<u64>()
	#[allow(clippy::len_without_is_empty)]
	pub fn len(&self) -> usize {
		self.magnitude.len()
	}

	/// Capacity of underlying storage, in units of mem::sizeof::<u64>()
	pub fn capacity(&self) -> usize {
		self.magnitude.capacity()
	}

	pub fn inner(&self) -> &BigUInt {
		&self.magnitude
	}

	pub unsafe fn inner_mut(&mut self) -> &mut BigUInt {
		&mut self.magnitude
	}

	pub fn into_inner(self) -> BigUInt {
		self.magnitude
	}

	pub fn is_zero(&self) -> bool {
		self.magnitude.is_zero()
	}

	pub fn set_zero(&mut self) {
		self.magnitude.set_zero();
		self.is_negative = false;
	}

	pub fn is_one(&self) -> bool {
		self.magnitude.is_one() && !self.is_negative
	}

	pub fn set_one(&mut self) {
		self.magnitude.set_one();
		self.is_negative = false;
	}

	pub fn is_negative(&self) -> bool {
		self.is_negative
	}

	pub fn abs_in_place(&mut self) {
		self.is_negative = false;
	}

	pub fn abs(mut self) -> Self {
		self.abs_in_place();
		self
	}

	pub fn unsigned_abs(self) -> BigUInt {
		self.magnitude
	}

	fn normalize(&mut self) {
		if self.is_zero() {
			self.is_negative = false;
		}
	}
}

impl Default for BigInt {
	fn default() -> Self {
		Self::ZERO
	}
}

impl Index<usize> for BigInt {
	type Output = u64;

	fn index(&self, index: usize) -> &Self::Output {
		&self.magnitude[index]
	}
}
