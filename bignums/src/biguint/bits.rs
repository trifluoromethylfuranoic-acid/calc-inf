use core::iter;
use core::ops::{
	BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
	ShrAssign,
};

use crate::biguint::BigUInt;
use crate::util::VecExt;

impl BigUInt {
	/// Shift left by whole digits
	/// A digit is 64 bits
	pub fn shl_digits(&mut self, digits: usize) {
		if self.is_zero() {
			return;
		}

		let old_len = self.len();
		self.data.extend_zero(digits);

		self.data.copy_within(0..old_len, digits);
		self.data[0..digits].fill(0u64);
	}

	/// Shift right by whole digits
	/// A digit is 64 bits
	pub fn shr_digits(&mut self, digits: usize) {
		if digits >= self.len() {
			self.set_zero();
			return;
		}

		self.data.copy_within(digits.., 0);
		self.data.truncate(self.len() - digits);
	}

	pub fn not_in_place(&mut self) {
		for x in self.data.iter_mut() {
			*x = !*x;
		}
		self.truncate_leading_zeros();
	}

	/// Return the number of zeros in the binary representation of the number.
	pub fn count_zeros(&self) -> u64 {
		self.data
			.iter()
			.copied()
			.map(|digit| digit.count_zeros() as u64)
			.sum()
	}

	/// Return the number of ones in the binary representation of the number.
	pub fn count_ones(&self) -> u64 {
		self.data
			.iter()
			.copied()
			.map(|digit| digit.count_ones() as u64)
			.sum()
	}

	/// Return the number of trailing zeros in the binary representation of the number.
	/// For 0 returns 0
	pub fn trailing_zeros(&self) -> u64 {
		let mut res = 0u64;
		for &digit in &self.data {
			if digit == 0 {
				res += u64::BITS as u64;
			} else {
				return res + digit.trailing_zeros() as u64;
			}
		}
		res
	}

	/// Return the number of trailing ones in the binary representation of the number.
	pub fn trailing_ones(&self) -> u64 {
		let mut res = 0u64;
		for &digit in &self.data {
			if digit == u64::MAX {
				res += u64::BITS as u64;
			} else {
				return res + digit.trailing_ones() as u64;
			}
		}
		res
	}

	/// Return the number of leading zeros in the binary representation of the number.
	/// For 0 returns 0
	pub fn leading_zeros(&self) -> u64 {
		self.data
			.last()
			.map(|&x| x.leading_zeros() as u64)
			.unwrap_or(0)
	}

	/// Return the number of leading ones in the binary representation of the number.
	pub fn leading_ones(&self) -> u64 {
		let mut res = 0u64;
		for &digit in self.data.iter().rev() {
			if digit == u64::MAX {
				res += u64::BITS as u64;
			} else {
				return res + digit.leading_ones() as u64;
			}
		}
		res
	}

	/// Returns the bit at the given position, counting from the least significant bit.
	pub fn bit(&self, pos: usize) -> bool {
		let mult64 = pos / (u64::BITS as usize);
		let rem64 = pos % (u64::BITS as usize);

		self.data
			.get(mult64)
			.map(|&x| (x >> rem64) & 1u64 != 0)
			.unwrap_or(false)
	}
}
macro_rules! impl_shl {
	($($t:ty),*) => {$(
		impl Shl<$t> for BigUInt {
			type Output = BigUInt;

			fn shl(mut self, rhs: $t) -> Self::Output {
				self <<= rhs;
				self
			}
		}
	)*}
}

impl_shl! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

macro_rules! impl_shl_assign {
    ($($t:ty),*) => {$(
	    impl ShlAssign<$t> for BigUInt {
		    fn shl_assign(&mut self, rhs: $t) {
			    // Sanity check
				assert!((rhs as i128) < (isize::MAX as i128), "attempt to bitshift left by an insane amount: {rhs}");
				#[allow(unused_comparisons)]
				let is_non_negative = rhs >= 0;
				assert!(is_non_negative, "attempt to bitshift left by negative amount: {rhs}");

			    if self.is_zero() { return; }

				const BITS: u64 = u64::BITS as u64;
				let mult64 = (rhs / (BITS as $t)) as usize;
				let rem    = (rhs % (BITS as $t)) as u64;

			    // Shift by mult64 digits
			    self.shl_digits(mult64);

				// Shift by rem bits
				if rem != 0 {
					let mut carry = 0u64;
					for x in &mut self.data[mult64..] {
						let new_val = (*x << rem) | carry;
						carry = *x >> (BITS - rem);

						*x = new_val;
					}
					if carry != 0 {
						self.data.push(carry);
					}
				}
			}
		}
	)*}
}

impl_shl_assign! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

macro_rules! impl_shr {
	($($t:ty),*) => {$(
		impl Shr<$t> for BigUInt {
			type Output = BigUInt;

			fn shr(mut self, rhs: $t) -> Self::Output {
				self >>= rhs;
				self
			}
		}
	)*}
}

impl_shr! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

macro_rules! impl_shr_assign {
	($($t:ty),*) => {$(
		impl ShrAssign<$t> for BigUInt {
			fn shr_assign(&mut self, rhs: $t) {
				#[allow(unused_comparisons)]
				let is_non_negative = rhs >= 0;
				assert!(is_non_negative, "attempt to bitshift right by negative amount: {rhs}");

				const BITS: u64 = u64::BITS as u64;
				let mult64 = (rhs / (BITS as $t)) as usize;
				let rem    = (rhs % (BITS as $t)) as u64;

				// Shift by mult64 digits
				self.shr_digits(mult64);

				// Shift by rem bits
				if rem != 0 {
					let mut carry = 0u64;
					for x in self.data.iter_mut().rev() {
						let new_val = (*x >> rem) | carry;
						carry = *x << (BITS - rem);

						*x = new_val;
					}
					if self.data.last() == Some(&0u64) {
						self.data.pop();
					}
				}
			}
		}
	)*}
}

impl_shr_assign! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl BitAndAssign<&BigUInt> for BigUInt {
	fn bitand_assign(&mut self, rhs: &BigUInt) {
		// Treat everything above len() as zeros
		self.data.truncate(rhs.len());
		for (x, y) in iter::zip(self.data.iter_mut(), rhs.data.iter()) {
			*x &= y;
		}
		self.truncate_leading_zeros();
	}
}

impl BitAnd<&BigUInt> for BigUInt {
	type Output = BigUInt;

	fn bitand(mut self, rhs: &BigUInt) -> Self::Output {
		self &= rhs;
		self
	}
}

impl BitAnd<BigUInt> for &BigUInt {
	type Output = BigUInt;

	fn bitand(self, mut rhs: BigUInt) -> Self::Output {
		rhs &= self;
		rhs
	}
}

impl BitOrAssign<&BigUInt> for BigUInt {
	fn bitor_assign(&mut self, rhs: &BigUInt) {
		if rhs.len() > self.len() {
			self.data.extend_zero(rhs.len() - self.len());
		}
		for (x, y) in iter::zip(self.data.iter_mut(), rhs.data.iter()) {
			*x |= y;
		}
	}
}

impl BitOr<&BigUInt> for BigUInt {
	type Output = BigUInt;

	fn bitor(mut self, rhs: &BigUInt) -> Self::Output {
		self |= rhs;
		self
	}
}

impl BitOr<BigUInt> for &BigUInt {
	type Output = BigUInt;

	fn bitor(self, mut rhs: BigUInt) -> Self::Output {
		rhs |= self;
		rhs
	}
}

impl BitXorAssign<&BigUInt> for BigUInt {
	fn bitxor_assign(&mut self, rhs: &BigUInt) {
		for (x, y) in iter::zip(self.data.iter_mut(), rhs.data.iter()) {
			*x ^= y;
		}
		if rhs.len() > self.len() {
			self.data.extend(rhs.data[self.len()..].iter().copied());
		}
		self.truncate_leading_zeros();
	}
}

impl BitXor<&BigUInt> for BigUInt {
	type Output = BigUInt;

	fn bitxor(mut self, rhs: &BigUInt) -> Self::Output {
		self ^= rhs;
		self
	}
}

impl BitXor<BigUInt> for &BigUInt {
	type Output = BigUInt;

	fn bitxor(self, mut rhs: BigUInt) -> Self::Output {
		rhs ^= self;
		rhs
	}
}

impl Not for BigUInt {
	type Output = BigUInt;

	fn not(mut self) -> Self::Output {
		self.not_in_place();
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::{from_foreign_biguint, to_foreign_biguint};

	#[test]
	fn test_shl() {
		test_shl_helper(
			"6846846153131516846848484878712315485461581468541664586"
				.parse()
				.unwrap(),
			789,
		);
		test_shl_helper("48646451651461645156847987135120".parse().unwrap(), 4515);
		test_shl_helper(
			"8797984464683153151318697879779797841387879"
				.parse()
				.unwrap(),
			0,
		);

		test_shl_helper("351105168485616848948".parse().unwrap(), 64);
		test_shl_helper(BigUInt::ZERO, 68);
	}

	#[test]
	fn test_shr() {
		test_shr_helper(
			"6846846153131516846848484878712315485461581468541664586"
				.parse()
				.unwrap(),
			45,
		);
		test_shr_helper("48646451651461645156847987135120".parse().unwrap(), 4515);
		test_shr_helper(
			"8797984464683153151318697879779797841387879"
				.parse()
				.unwrap(),
			0,
		);

		test_shr_helper(
			"3511051684856168464684684864684864351848948"
				.parse()
				.unwrap(),
			64,
		);
		test_shr_helper(BigUInt::ZERO, 68);
	}

	#[test]
	#[should_panic]
	fn test_shl2() {
		let _ = BigUInt::from(456u64) << -1;
	}

	fn test_shl_helper(a: BigUInt, b: u64) {
		let res_native = a.clone() << b;
		let res_foreign = from_foreign_biguint(to_foreign_biguint(a).shl(b));
		assert_eq!(res_native, res_foreign)
	}

	fn test_shr_helper(a: BigUInt, b: u64) {
		let res_native = a.clone() >> b;
		let res_foreign = from_foreign_biguint(to_foreign_biguint(a).shr(b));
		assert_eq!(res_native, res_foreign)
	}

	#[test]
	fn test_leading_zeros() {
		assert_eq!(BigUInt::ZERO.leading_zeros(), 0);
		assert_eq!(BigUInt::from(1u64).leading_zeros(), 63);
		assert_eq!(BigUInt::from(u64::MAX).leading_zeros(), 0);
		assert_eq!(BigUInt::from(1u64 << 32).leading_zeros(), 31);

		// Multi-digit cases
		let mut x = BigUInt::from(1u64);
		x.shl_digits(1);
		assert_eq!(x.leading_zeros(), 63);

		let mut y = BigUInt::from(u64::MAX);
		y.shl_digits(2);
		assert_eq!(y.leading_zeros(), 0);
	}

	#[test]
	fn test_leading_ones() {
		assert_eq!(BigUInt::ZERO.leading_ones(), 0);
		assert_eq!(BigUInt::from(u64::MAX).leading_ones(), 64);
		assert_eq!(BigUInt::from(1u64).leading_ones(), 0);
		assert_eq!(BigUInt::from(!0u32 as u64).leading_ones(), 0);

		// Multi-digit cases
		let mut x = BigUInt::from(u64::MAX);
		x.shl_digits(1);
		assert_eq!(x.leading_ones(), 64);

		let mut y = BigUInt::from(u64::MAX);
		y.shl_digits(1);
		y |= &BigUInt::from(u64::MAX << 32);
		assert_eq!(y.leading_ones(), 96);
	}

	#[test]
	fn test_trailing_zeros() {
		assert_eq!(BigUInt::ZERO.trailing_zeros(), 0);
		assert_eq!(BigUInt::from(1u64).trailing_zeros(), 0);
		assert_eq!(BigUInt::from(u64::MAX).trailing_zeros(), 0);
		assert_eq!(BigUInt::from(1u64 << 32).trailing_zeros(), 32);

		// Multi-digit cases
		let mut x = BigUInt::from(1u64);
		x.shl_digits(1);
		assert_eq!(x.trailing_zeros(), 64);

		let mut y = BigUInt::from(1u64 << 32);
		y.shl_digits(2);
		assert_eq!(y.trailing_zeros(), 160);
	}

	#[test]
	fn test_trailing_ones() {
		assert_eq!(BigUInt::ZERO.trailing_ones(), 0);
		assert_eq!(BigUInt::from(1u64).trailing_ones(), 1);
		assert_eq!(BigUInt::from(u64::MAX).trailing_ones(), 64);
		assert_eq!(BigUInt::from((1u64 << 32) - 1).trailing_ones(), 32);

		// Multi-digit cases
		let mut x = BigUInt::from(u64::MAX);
		x.shl_digits(1);
		x |= &BigUInt::from(u64::MAX);
		assert_eq!(x.trailing_ones(), 128);
	}
}
