// TODO: impl stuff

use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::bigint::BigInt;

macro_rules! impl_shl_shr {
	($($t:ty),*) => {$(
		impl Shl<$t> for BigInt {
			type Output = BigInt;
			fn shl(mut self, rhs: $t) -> Self::Output {
				self <<= rhs;
				self
			}
		}

		impl ShlAssign<$t> for BigInt {
			fn shl_assign(&mut self, rhs: $t) {
				self.magnitude <<= rhs;
			}
		}

		impl Shr<$t> for BigInt {
			type Output = BigInt;
			fn shr(mut self, rhs: $t) -> Self::Output {
				self >>= rhs;
				self
			}
		}

		impl ShrAssign<$t> for BigInt {
			fn shr_assign(&mut self, rhs: $t) {
				if !self.is_negative() {
					self.magnitude >>= rhs;
					return;
				}

				let mult64 = (rhs / (u64::BITS as $t)) as usize;
				let rem64 = (rhs % (u64::BITS as $t)) as u64;

				let mut correction = false;

				for &digit in self.magnitude.inner().iter().take(mult64) {
					if digit != 0 {
						correction = true;
						break;
					}
				}

				if rem64 != 0 {
					if let Some(&digit) = self.magnitude.inner().get(mult64) {
						if digit << (u64::BITS as u64 - rem64) != 0 {
							correction = true;
						}
					}
				}

				self.magnitude >>= rhs;
				if correction {
					self.magnitude += 1;
				}
			}
		}
	)*}
}

impl_shl_shr! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
	use alloc::vec;
	use super::*;

	#[test]
	fn test_shl() {
		let cases = vec![
			(1, 1, 2),     // 1 << 1 = 2
			(1, 2, 4),     // 1 << 2 = 4
			(2, 1, 4),     // 2 << 1 = 4
			(5, 2, 20),    // 5 << 2 = 20
			(-1, 1, -2),   // -1 << 1 = -2
			(-2, 2, -8),   // -2 << 2 = -8
			(-5, 1, -10),  // -5 << 1 = -10
			(-3, 3, -24),  // -3 << 3 = -24
		];

		for (val, shift, expected) in cases {
			let n = BigInt::from(val);
			assert_eq!(n << shift, BigInt::from(expected));
		}
	}

	#[test]
	fn test_shr() {
		let cases = vec![
			(8, 1, 4),     // 8 >> 1 = 4
			(8, 2, 2),     // 8 >> 2 = 2
			(15, 2, 3),    // 15 >> 2 = 3
			(16, 3, 2),    // 16 >> 3 = 2
		];

		for (val, shift, expected) in cases {
			let n = BigInt::from(val);
			assert_eq!(n >> shift, BigInt::from(expected));
		}
	}

	#[test]
	fn test_shr_negative() {
		let cases = vec![
			(-8, 1, -4),     // -8 >> 1 = -4
			(-8, 2, -2),     // -8 >> 2 = -2
			(-15, 2, -4),    // -15 >> 2 = -4
			(-7, 1, -4),     // -7 >> 1 = -4
			(-13, 2, -4),    // -13 >> 2 = -4
			(-11, 3, -2),    // -11 >> 3 = -2
			(-25, 3, -4),    // -25 >> 3 = -4
			(-16, 3, -2),    // -16 >> 3 = -2
		];

		for (val, shift, expected) in cases {
			let n = BigInt::from(val);
			assert_eq!(n >> shift, BigInt::from(expected));
		}
	}
}


