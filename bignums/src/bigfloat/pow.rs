use crate::bigfloat::BigFloat;

impl BigFloat {
	pub fn exp(&self, prec: i64) -> BigFloat {
		if self.is_zero() {
			return BigFloat::ONE;
		}

		todo!()
	}

	pub fn powi_with_precision(&self, pow: i64, prec: i64) -> BigFloat {
		if self.is_zero() {
			return BigFloat::ZERO;
		}
		if self.is_one() {
			return BigFloat::ONE;
		}

		let working_prec = prec + 16;

		let is_pow_negative = pow.is_negative();
		let mut abs_pow = pow.unsigned_abs();

		let mut res = BigFloat::ONE;
		let mut power_of_self = self.clone();

		while abs_pow != 0 {
			if abs_pow & 1 == 1 {
				res = res.mul_with_precision(&power_of_self, working_prec);
			}
			abs_pow >>= 1;
			power_of_self = power_of_self.mul_with_precision(&power_of_self, working_prec);
		}

		if is_pow_negative {
			res = res.reciprocal(working_prec);
		}

		res.round_to_precision(prec);
		res
	}
}
