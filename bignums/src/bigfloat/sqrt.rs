use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;

impl BigFloat {
	pub fn sqrt(&self, prec: i64) -> BigFloat {
		if self.is_negative() {
			panic!("Cannot take sqrt of negative number");
		}
		if self.is_zero() {
			return BigFloat::ZERO;
		}
		if self.is_one() {
			return BigFloat::ONE;
		}

		todo!()
	}

	fn est_sqrt(x: BigFloat) -> BigFloat {
		let mut shift = x.m.magnitude.ilog2() as i64;
		if (x.e + shift) % 2 != 0 {
			shift += 1;
		}
		let n = (x.e + shift) / 2;

		let a = BigFloat::from_mantissa_exponent(BigInt::ONE, n - 1);
		let b = BigFloat::from_mantissa_exponent(x.m, n - shift - 1);
		&a + &b
	}
}
