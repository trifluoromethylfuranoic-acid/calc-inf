use core::ops::Index;

use smallvec::SmallVec;

mod add;
mod bits;
mod cmp;
mod convert;
mod convert_data;
mod div;
mod exp;
mod mul;
mod set_val;
mod str;
mod sub;

pub use div::*;
pub use mul::*;
pub use sub::*;

type Data = SmallVec<[u64; 2]>;

/// Dynamic, arbitrary-sized unsigned integer type
#[derive(Eq, PartialEq, Hash)]
pub struct BigUInt {
	// Little-endian
	// Invariant: no leading zero digits
	data: Data,
}

impl BigUInt {
	pub const ZERO: Self = Self {
		data: SmallVec::new_const(),
	};
	pub const ONE: Self = Self {
		data: unsafe { SmallVec::from_const_with_len_unchecked([1u64; 2], 1) },
	};

	/// Length of underlying storage, in units of mem::sizeof::<u64>()
	#[allow(clippy::len_without_is_empty)]
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Capacity of underlying storage, in units of mem::sizeof::<u64>()
	pub fn capacity(&self) -> usize {
		self.data.capacity()
	}

	pub fn inner(&self) -> &SmallVec<[u64; 2]> {
		&self.data
	}

	pub unsafe fn inner_mut(&mut self) -> &mut SmallVec<[u64; 2]> {
		&mut self.data
	}

	pub fn into_inner(self) -> SmallVec<[u64; 2]> {
		self.data
	}

	pub fn is_zero(&self) -> bool {
		self.data.is_empty()
	}

	pub fn set_zero(&mut self) {
		self.data.clear();
	}

	pub fn is_one(&self) -> bool {
		self.data.len() == 1 && self.data[0] == 1
	}

	pub fn set_one(&mut self) {
		self.data.clear();
		self.data.push(1u64);
	}

	fn truncate_leading_zeros(&mut self) {
		while let Some(&0u64) = self.data.last() {
			self.data.pop();
		}
	}
}

impl Default for BigUInt {
	fn default() -> Self {
		Self::ZERO
	}
}

impl Index<usize> for BigUInt {
	type Output = u64;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

#[cfg(test)]
mod tests {}
