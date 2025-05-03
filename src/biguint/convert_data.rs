use alloc::vec::Vec;

use smallvec::{SmallVec, ToSmallVec};

use crate::biguint::BigUInt;

impl BigUInt {
	/// Creates a bigint from a SmallVec<[u64; 2]>, which stores its digits in little-endian
	/// Empty vector corresponds to 0
	pub fn from_smallvec_le(data: SmallVec<[u64; 2]>) -> Self {
		let mut res = Self { data };
		res.truncate_leading();
		res
	}

	/// Creates a bigint from a SmallVec<[u64; 2]>, which stores its digits in little-endian
	/// # Safety
	/// Callers must ensure that data has minimum leading zeros
	pub unsafe fn from_smallvec_le_unchecked(data: SmallVec<[u64; 2]>) -> Self {
		Self { data }
	}

	/// Creates a bigint from a Vec<u64>, which stores its digits in little-endian
	/// # Safety
	/// Callers must ensure that data has minimum leading zeros
	pub fn from_vec_le(data: Vec<u64>) -> Self {
		Self::from_smallvec_le(data.to_smallvec())
	}

	/// Creates a bigint from a Vec<u64>, which stores its digits in little-endian
	/// # Safety
	/// Callers must ensure that data has minimum leading zeros
	pub unsafe fn from_vec_le_unchecked(data: Vec<u64>) -> Self {
		// SAFETY: ensured by caller
		unsafe { Self::from_smallvec_le_unchecked(data.into()) }
	}

	pub fn from_bytes_le(mut data: Vec<u8>) -> Self {
		while data.len() % size_of::<u64>() != 0 {
			data.push(0);
		}
		data.shrink_to_fit();
		assert_eq!(
			data.capacity() % size_of::<u64>(),
			0,
			"capacity is not a multiple of size_of::<u64>"
		);
		let (ptr, len, cap) = data.into_parts();
		let ptr_new = ptr.cast();
		let len_new = len / size_of::<u64>();
		let cap_new = cap / size_of::<u64>();
		let data_new = unsafe { Vec::from_parts(ptr_new, len_new, cap_new) };
		Self::from_vec_le(data_new)
	}

	pub fn into_bytes_le(self) -> Vec<u8> {
		let data = self.into_inner().into_vec();
		let (ptr, len, cap) = data.into_parts();
		let ptr_new = ptr.cast();
		let len_new = len * size_of::<u64>();
		let cap_new = cap * size_of::<u64>();
		unsafe { Vec::from_parts(ptr_new, len_new, cap_new) }
	}
}
