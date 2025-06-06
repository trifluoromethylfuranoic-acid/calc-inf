use core::ops::{Mul, MulAssign};

use crate::biguint::BigUInt;
use crate::util::{VecExt, carrying_mul};

pub trait MulTo {
	/// Calculates lhs * rhs, saves result into self
	/// Unlike self = &lhs * &rhs, avoids allocations if possible
	fn mul_to(&mut self, lhs: &BigUInt, rhs: &BigUInt);
}

impl MulTo for BigUInt {
	fn mul_to(&mut self, lhs: &BigUInt, rhs: &BigUInt) {
		let new_len = lhs.len() + rhs.len();
		self.data.set_len_fill_zero(new_len);
		for (i, &a_i) in lhs.data.iter().enumerate() {
			let mut carry = 0u64;
			for (j, &b_j) in rhs.data.iter().enumerate() {
				let (lo, hi) = carrying_mul(a_i, b_j);
				let (sum1, carry1) = lo.overflowing_add(carry);
				let (sum2, carry2) = self[i + j].overflowing_add(sum1);
				self.data[i + j] = sum2;
				// Can't overflow because magic
				// u64 * u64 + u64 + u64 fits in 2 u64s
				carry = hi + carry1 as u64 + carry2 as u64;
			}
			self.data[i + rhs.len()] = carry;
		}
		self.truncate_leading_zeros();
	}
}

impl Mul<&BigUInt> for &BigUInt {
	type Output = BigUInt;

	fn mul(self, rhs: &BigUInt) -> Self::Output {
		let mut res = BigUInt::ZERO;
		res.mul_to(self, rhs);
		res
	}
}

macro_rules! impl_mul_u {
	($($t:ty),*) => {
		$(
			impl Mul<$t> for &BigUInt {
				type Output = BigUInt;
				fn mul(self, rhs: $t) -> Self::Output {
					self * &BigUInt::from(rhs)
				}
			}

			impl Mul<&BigUInt> for $t {
				type Output = BigUInt;
				fn mul(self, rhs: &BigUInt) -> Self::Output {
					rhs * &BigUInt::from(self)
				}
			}
		)*
	}
}

impl_mul_u! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_mul_i {
	($($t:ty),*) => {
		$(
			impl Mul<$t> for &BigUInt {
				type Output = BigUInt;
				fn mul(self, rhs: $t) -> Self::Output {
					self * &BigUInt::try_from(rhs).expect("multiplication of BigUInt by negative value")
				}
			}

			impl Mul<&BigUInt> for $t {
				type Output = BigUInt;
				fn mul(self, rhs: &BigUInt) -> Self::Output {
					rhs * &BigUInt::try_from(self).expect("multiplication of BigUInt by negative value")
				}
			}
		)*
	}
}

impl_mul_i! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_mul_assign {
	($($t:ty),*) => {
		$(
			impl MulAssign<$t> for BigUInt {
				fn mul_assign(&mut self, rhs: $t) {
					*self = &*self * rhs
				}
			}
		)*
	}
}

impl_mul_assign! { u8, u16, u32, u64, u128, usize, &BigUInt,
i8, i16, i32, i64, i128, isize}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::{from_foreign_biguint, to_foreign_biguint};

	#[test]
	fn test_mul() {
		mul_helper(
			BigUInt::from_vec_le(vec![
				6848468468486468486,
				6851351684844315148,
				87951463548843415,
				6848468135153,
			]),
			BigUInt::from_vec_le(vec![
				486468153601531,
				484684416531315,
				468431513584864,
				84686484684864,
			]),
		);

		mul_helper(
			BigUInt::from_vec_le(vec![
				u64::MAX - 10,
				u64::MAX,
				u64::MAX - 1,
				u64::MAX - 4564564,
			]),
			BigUInt::from_vec_le(vec![u64::MAX - 1, u64::MAX - 156456, u64::MAX, u64::MAX]),
		);

		mul_helper(BigUInt::from(u128::MAX), BigUInt::from(u128::MAX));
	}

	fn mul_helper(a: BigUInt, b: BigUInt) {
		let res_native = &a * &b;
		let res_foreign = from_foreign_biguint(to_foreign_biguint(a) * to_foreign_biguint(b));
		assert_eq!(res_native, res_foreign)
	}
}
