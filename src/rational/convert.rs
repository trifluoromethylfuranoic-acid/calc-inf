use crate::SetVal;
use crate::bigint::BigInt;
use crate::biguint::BigUInt;
use crate::rational::Rational;

impl From<BigUInt> for Rational {
	fn from(value: BigUInt) -> Self {
		Self::new(value.into(), BigUInt::ONE)
	}
}

impl From<BigInt> for Rational {
	fn from(value: BigInt) -> Self {
		Self::new(value, BigUInt::ONE)
	}
}

macro_rules! impl_from {
	($($t:ty),*) => {
		$(impl From<$t> for Rational {
			fn from(val: $t) -> Self {
				let mut res = Self::ZERO;
				res.set_val(val);
				res
			}
		})*
	}
}

impl_from! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
