use core::iter;

use smallvec::SmallVec;

macro_rules! x_to_ys_le {
	($($name:ident: $big:ty => $small:ty),*) => {$(
		#[allow(dead_code)]
		pub(crate) fn $name(val: $big) -> [$small; 2] {
			let arr: [$small; 2] = unsafe { core::mem::transmute(val) };
			#[cfg(target_endian = "big")]
				return [arr[1], arr[0]];
			#[cfg(target_endian = "little")]
				return arr;
		}
	)*}
}

x_to_ys_le! { u128_to_u64s: u128 => u64, i128_to_i64s: i128 => i64, i128_to_u64s: i128 => u64 }

macro_rules! xs_to_y_le {
	($($name:ident: $small:ty => $big:ty),*) => {$(
		#[allow(dead_code)]
		pub(crate) fn $name(val: [$small; 2]) -> $big {
			#[cfg(target_endian = "big")]
				{ (val[0], val[1]) = (val[1], val[0]); }
			unsafe { core::mem::transmute(val) }
		})*
	}
}

xs_to_y_le! { u64s_to_u128: u64 => u128, i64s_to_i128: i64 => i128, u64s_to_i128: i64 => u128 }

pub(crate) trait VecExt<T>
where
	T: Default + Clone,
{
	fn get_or_default(&self, idx: usize) -> T;
	fn set_or_insert(&mut self, idx: usize, val: T);

	fn extend_zero(&mut self, by: usize);
}

impl<T, const N: usize> VecExt<T> for SmallVec<[T; N]>
where
	T: Default + Clone,
{
	fn get_or_default(&self, idx: usize) -> T {
		self.get(idx).cloned().unwrap_or_default()
	}

	fn set_or_insert(&mut self, idx: usize, val: T) {
		if idx < self.len() {
			self[idx] = val;
		} else {
			self.extend_zero(idx - self.len());
			self.push(val);
		}
	}

	fn extend_zero(&mut self, by: usize) {
		self.extend(iter::repeat_n(T::default(), by));
	}
}

pub(crate) fn carrying_mul(lhs: u64, rhs: u64) -> (u64, u64) {
	const MASK: u64 = 0x00000000ffffffff;
	let (a, b, c, d);

	a = (lhs & MASK) * (rhs & MASK);
	b = (lhs >> 32) * (rhs & MASK);
	c = (lhs & MASK) * (rhs >> 32);
	d = (lhs >> 32) * (rhs >> 32);

	let (lo, carry1) = u64::overflowing_add(a, b << 32);
	let (lo, carry2) = u64::overflowing_add(lo, c << 32);
	let hi = d + (b >> 32) + (c >> 32) + (carry1 as u64) + (carry2 as u64);
	(lo, hi)
}

#[cfg(test)]
use crate::biguint::BigUInt;

#[cfg(test)]
pub(crate) fn to_foreign_biguint(a: BigUInt) -> num_bigint::BigUint {
	let data = a.into_bytes_le();
	num_bigint::BigUint::from_bytes_le(&data)
}

#[cfg(test)]
pub(crate) fn from_foreign_biguint(a: num_bigint::BigUint) -> BigUInt {
	let data = a.to_bytes_le();
	BigUInt::from_bytes_le(data)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_carrying_mul() {
		assert_eq!(
			carrying_mul(u64::MAX, u64::MAX),
			(1u64, 0xffff_ffff_ffff_fffe_u64)
		);
		carrying_mul_helper(45646544848615, 8468481531548);
		carrying_mul_helper(783735, 453422222222222225);
		carrying_mul_helper(1111111111111111, 999999999999999999);
		carrying_mul_helper(u64::MAX, u64::MAX);
	}

	fn carrying_mul_helper(a: u64, b: u64) {
		let res_u128 = (a as u128) * (b as u128);
		let (x, y) = carrying_mul(a, b);
		let res = u64s_to_u128([x, y]);
		assert_eq!(res, res_u128);
	}
}
