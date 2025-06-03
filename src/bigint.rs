mod set_val;

use core::ops::Index;
use crate::biguint::BigUInt;

/// Dynamic, arbitrary-sized signed integer type
#[derive(Eq, PartialEq)]
pub struct BigInt {
	// Invariant: if val == 0, is_negative should be false
	is_negative: bool,
	val: BigUInt
}

impl BigInt {
	pub const ZERO: Self = Self {
		is_negative: false,
		val: BigUInt::ZERO
	};
	pub const ONE: Self = Self {
		is_negative: false,
		val: BigUInt::ONE
	};
	pub const NEG_ONE: Self = Self {
		is_negative: true,
		val: BigUInt::ONE
	};

	/// Length of underlying storage, in units of mem::sizeof::<u64>()
	#[allow(clippy::len_without_is_empty)]
	pub fn len(&self) -> usize {
		self.val.len()
	}

	/// Capacity of underlying storage, in units of mem::sizeof::<u64>()
	pub fn capacity(&self) -> usize {
		self.val.capacity()
	}

	pub fn inner(&self) -> &BigUInt {
		&self.val
	}

	pub fn is_zero(&self) -> bool {
		self.val.is_zero()
	}

	pub fn set_zero(&mut self) {
		self.val.set_zero();
		self.is_negative = false;
	}
	
	pub fn is_negative(&self) -> bool {
		self.is_negative
	}
	
	pub fn abs_in_place(&mut self) {
		self.is_negative = false;
	}
	
	pub fn abs(&self) -> Self {
		let mut x = self.clone();
		x.abs_in_place();
		x
	}
	
	pub fn unsigned_abs(&self) -> BigUInt {
		self.val.clone()
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
		&self.val[index]
	}
}
