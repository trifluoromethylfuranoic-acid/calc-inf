use alloc::vec::Vec;
use core::ops::Index;

use smallvec::SmallVec;

type Data = SmallVec<[u64; 2]>;

/// Dynamic, arbitrary sized signed integer type
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BigInt {
	// Little-endian
	// Two's complement
	// Invariant: non-empty
	// Invariant: minimum leading zeros/ones
	data: Data,
}

impl BigInt {
	pub const ZERO: Self = Self {
		data: unsafe { SmallVec::from_const_with_len_unchecked([0u64; 2], 1) },
	};
	pub const ONE: Self = Self {
		data: unsafe { SmallVec::from_const_with_len_unchecked([1u64; 2], 1) },
	};

	/// Length of underlying storage, in units of mem::sizeof::<u64>()
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Capacity of underlying storage, in units of mem::sizeof::<u64>()
	pub fn capacity(&self) -> usize {
		self.data.capacity()
	}

	/// Creates a bigint from a SmallVec<[u64; 2]>, which stores its digits in little-endian.
	/// Empty vector corresponds to 0.
	pub fn from_smallvec_le(mut data: SmallVec<[u64; 2]>) -> Self {
		if data.is_empty() {
			data.push(0u64)
		}
		let mut res = Self { data };
		res.truncate_leading();
		res
	}

	/// Creates a bigint from a SmallVec<[u64; 2]>, which stores its digits in little-endian.
	/// Callers must ensure that data is non-empty and has minimum leading ones/zeros
	pub unsafe fn from_smallvec_le_unchecked(data: SmallVec<[u64; 2]>) -> Self {
		Self { data }
	}

	/// Creates a bigint from a Vec<u64>, which stores its digits in little-endian.
	/// Empty vector corresponds to 0.
	pub fn from_vec_le(data: Vec<u64>) -> Self {
		Self::from_smallvec_le(data.into())
	}

	/// Creates a bigint from a Vec<u64>, which stores its digits in little-endian.
	/// Callers must ensure that data is non-empty and has minimum leading ones/zeros
	pub unsafe fn from_vec_le_unchecked(data: Vec<u64>) -> Self {
		unsafe { Self::from_smallvec_le_unchecked(data.into()) }
	}

	pub fn as_inner(&self) -> &SmallVec<[u64; 2]> {
		&self.data
	}

	pub fn as_inner_mut(&mut self) -> &mut SmallVec<[u64; 2]> {
		&mut self.data
	}

	pub fn into_inner(self) -> SmallVec<[u64; 2]> {
		self.data
	}

	fn truncate_leading(&mut self) {
		let leading_bit_ = leading_bit(self[self.len() - 1]);
		let leading_bit_repeated = if leading_bit_ == 0 { 0u64 } else { u64::MAX };
		while self.len() > 1 {
			if self[self.len() - 1] == leading_bit_repeated
				&& leading_bit(self[self.len() - 2]) == leading_bit_
			{
				self.data.pop();
			} else {
				break;
			}
		}
	}
}

fn leading_bit(x: u64) -> u64 {
	x >> (u64::BITS - 1)
}

impl Default for BigInt {
	fn default() -> Self {
		Self::ZERO
	}
}

macro_rules! impl_from_i {
	($($t:ty),*) => {
		$(impl From<$t> for BigInt {
			fn from(value: $t) -> Self {
				let value: i64 = value.into();
				Self {
					data: smallvec![value as u64]
				}
			}
		})*
	}
}

impl_from_i! { i8, i16, i32, i64, u8, u16, u32 }

impl From<i128> for BigInt {
	fn from(value: i128) -> Self {
		let bytes = value.to_le_bytes();
		let lo = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
		let hi = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
		let mut res = Self {
			data: smallvec![lo, hi],
		};
		res.truncate_leading();
		res
	}
}

impl From<isize> for BigInt {
	fn from(value: isize) -> Self {
		#[cfg(target_pointer_width = "16")]
		let value = value as i16;
		#[cfg(target_pointer_width = "32")]
		let value = value as i32;
		#[cfg(target_pointer_width = "64")]
		let value = value as i64;
		#[cfg(not(any(
			target_pointer_width = "32",
			target_pointer_width = "64",
			target_pointer_width = "16"
		)))]
		compile_error!("This crate only supports 16, 32 and 64 bit targets.");
		value.into()
	}
}

impl From<u64> for BigInt {
	fn from(value: u64) -> Self {
		Self {
			data: if leading_bit(value) == 0 {
				smallvec![value]
			} else {
				smallvec![value, 0u64]
			},
		}
	}
}

impl From<u128> for BigInt {
	fn from(value: u128) -> Self {
		let bytes = value.to_le_bytes();
		let lo = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
		let hi = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
		let mut res = Self {
			data: if leading_bit(hi) == 0 {
				smallvec![lo, hi]
			} else {
				smallvec![lo, hi, 0u64]
			},
		};
		res.truncate_leading();
		res
	}
}

impl From<usize> for BigInt {
	fn from(value: usize) -> Self {
		#[cfg(target_pointer_width = "16")]
		let value = value as u16;
		#[cfg(target_pointer_width = "32")]
		let value = value as u32;
		#[cfg(target_pointer_width = "64")]
		let value = value as u64;
		#[cfg(not(any(
			target_pointer_width = "32",
			target_pointer_width = "64",
			target_pointer_width = "16"
		)))]
		compile_error!("This crate only supports 16, 32 and 64 bit targets.");
		value.into()
	}
}

impl Index<usize> for BigInt {
	type Output = u64;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create() {
		let zero: BigInt = Default::default();
		let cmp = BigInt::from_vec_le(vec![0u64]);
		assert_eq!(zero, cmp);

		let from_u32 = BigInt::from(128u32);
		let cmp = BigInt::from_vec_le(vec![128u32 as u64]);
		assert_eq!(from_u32, cmp);

		let from_u64 = BigInt::from(u64::MAX);
		let cmp = BigInt::from_vec_le(vec![u64::MAX, 0u64]);
		assert_eq!(from_u64, cmp);

		let from_u128 = BigInt::from(u128::MAX);
		let cmp = BigInt::from_vec_le(vec![u64::MAX, u64::MAX, 0u64]);
		assert_eq!(from_u128, cmp);

		let from_i32 = BigInt::from(128i32);
		let cmp = BigInt::from_vec_le(vec![128u32 as u64]);
		assert_eq!(from_i32, cmp);

		let from_i64 = BigInt::from(-1i64);
		let cmp = BigInt::from_vec_le(vec![-1i64 as u64]);
		assert_eq!(from_i64, cmp);

		let from_i128 = BigInt::from(i128::MAX);
		let cmp = BigInt::from_vec_le(vec![u64::MAX, i64::MAX as u64]);
		assert_eq!(from_i128, cmp);

		let from_empty_vec = BigInt::from_vec_le(vec![]);
		let cmp = BigInt::from_vec_le(vec![0u64]);
		assert_eq!(from_empty_vec, cmp);

		let from_defective_vec = BigInt::from_vec_le(vec![u64::MAX, u64::MAX]);
		let cmp = BigInt::from_vec_le(vec![u64::MAX]);
		assert_eq!(from_defective_vec, cmp);
	}
}
