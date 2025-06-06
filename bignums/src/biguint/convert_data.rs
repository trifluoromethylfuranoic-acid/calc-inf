use alloc::vec::Vec;
use core::ptr;

use smallvec::{SmallVec, ToSmallVec};

use crate::biguint::BigUInt;

impl BigUInt {
	/// Creates a bigint from a SmallVec<[u64; 2]>, which stores its digits in little-endian
	/// Empty vector corresponds to 0
	pub fn from_smallvec_le(data: SmallVec<[u64; 2]>) -> Self {
		let mut res = Self { data };
		res.truncate_leading_zeros();
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
			data.push(0u8);
		}

		#[cfg(target_endian = "big")]
		remap_endianness(&mut data);

		let ptr = data.as_ptr().cast();
		let len = data.len() / size_of::<u64>();
		let slice = unsafe { ptr::slice_from_raw_parts(ptr, len).as_ref().unwrap() };
		let vec = SmallVec::from_slice(slice);
		Self::from_smallvec_le(vec)

		// let (ptr, len, cap) = data.into_parts();
		// let ptr_new = ptr.cast();
		// let len_new = len / size_of::<u64>();
		// let cap_new = cap / size_of::<u64>();
		// let data_new = unsafe { Vec::from_parts(ptr_new, len_new, cap_new) };
		// Self::from_vec_le(data_new)
	}

	pub fn into_bytes_le(mut self) -> Vec<u8> {
		// let data = self.into_inner().into_vec();

		let ptr = self.data.as_mut_ptr().cast();
		let len = self.data.len() * size_of::<u64>();
		let slice = unsafe { ptr::slice_from_raw_parts_mut(ptr, len).as_mut().unwrap() };

		#[cfg(target_endian = "big")]
		remap_endianness(slice);

		slice.to_vec()

		// let (ptr, len, cap) = data.into_parts();
		// let ptr_new = ptr.cast();
		// let len_new = len * size_of::<u64>();
		// let cap_new = cap * size_of::<u64>();
		// unsafe { Vec::from_parts(ptr_new, len_new, cap_new) }
	}
}

#[cfg(target_endian = "big")]
fn remap_endianness(data: &mut [u8]) {
	let mut iter = data.chunks_exact_mut(size_of::<u64>());
	for chunk in &mut iter {
		chunk.reverse();
	}
	debug_assert!(
		iter.into_remainder().is_empty(),
		"something went wrong remapping byte vec endianness"
	);
}
