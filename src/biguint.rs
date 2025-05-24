use core::ops::Index;

use smallvec::SmallVec;

mod add;
mod div;
mod bits;
mod cmp;
mod convert;
mod convert_data;
mod set_val;
mod str;
mod sub;
mod mul;

pub use sub::*;
pub use mul::*;

type Data = SmallVec<[u64; 2]>;

/// Dynamic, arbitrary-sized unsigned integer type
#[derive(Eq, PartialEq, Debug)]
pub struct BigUInt {
	// Little-endian
	// Invariant: minimum leading zeros
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

	pub fn as_inner(&self) -> &SmallVec<[u64; 2]> {
		&self.data
	}

	pub unsafe fn as_inner_mut(&mut self) -> &mut SmallVec<[u64; 2]> {
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
