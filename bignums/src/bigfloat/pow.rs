use crate::bigfloat::BigFloat;

impl BigFloat {
	pub fn exp(&self, prec: i64) -> BigFloat {
		if self.is_zero() {
			return BigFloat::ONE;
		}

		if self.is_negative() {
			todo!()
		}

		// At least self.ilog2() + 4
		let ln2 = BigFloat::ln2_underestimate(self.ilog2() + 16);
		// k is an overestimate, and at most by 1
		let (k, r) = self.div_rem_floor(&ln2);

		let mut i = 0i64;
		let mut res = BigFloat::ZERO;
		let mut a = BigFloat::ONE;

		loop {
			res += &a;
			a *= &r;
			a = a.div(&BigFloat::from(i), prec);
			i += 1;
		}

		res << i64::try_from(&k).expect("exponentiation would result in overflow");

		res
	}
}
