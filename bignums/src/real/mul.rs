use core::ops::{Mul, MulAssign};

use crate::real::Real;

impl Mul<Real> for Real {
	type Output = Real;

	fn mul(self, rhs: Real) -> Self::Output {
		let a_round = self.eval(0);
		let b_round = rhs.eval(0);
		let a_ilog2 = if a_round.is_zero() {
			0
		} else {
			a_round.ilog2()
		};
		let b_ilog2 = if b_round.is_zero() {
			0
		} else {
			b_round.ilog2()
		};
		Real::new(move |prec| {
			let actual_prec = prec + 1;

			let prec_a = actual_prec + 3 + b_ilog2;
			let prec_b = actual_prec + 3 + a_ilog2;
			let prec_b = i64::max(prec_b, -b_ilog2);

			let a = self.eval(prec_a);
			let b = rhs.eval(prec_b);

			a.mul_with_precision(&b, actual_prec)
		})
	}
}

impl MulAssign for Real {
	fn mul_assign(&mut self, rhs: Self) {
		*self = self.clone().mul(rhs);
	}
}

#[cfg(test)]
mod test {
	use core::str::FromStr;

	use super::*;
	use crate::bigfloat::BigFloat;

	#[test]
	fn test_mul() {
		let a = BigFloat::from_str("1351854615616.5681651616").unwrap();
		let b = BigFloat::from_str("51651.56165165151").unwrap();
		test_mul_helper(a, b);

		// Large positive numbers
		let a = BigFloat::from_str("999999999999999999999.999999999").unwrap();
		let b = BigFloat::from_str("888888888888888888888.888888888").unwrap();
		test_mul_helper(a, b);

		// Small decimal numbers
		let a = BigFloat::from_str("0.000000000000001").unwrap();
		let b = BigFloat::from_str("0.000000000000002").unwrap();
		test_mul_helper(a, b);

		// Negative numbers
		let a = BigFloat::from_str("-123456789.987654321").unwrap();
		let b = BigFloat::from_str("987654321.123456789").unwrap();
		test_mul_helper(a, b);

		let a = BigFloat::from_str("-42.42").unwrap();
		let b = BigFloat::from_str("-42.42").unwrap();
		test_mul_helper(a, b);

		// Zero multiplication
		let a = BigFloat::from_str("0").unwrap();
		let b = BigFloat::from_str("123456789.987654321").unwrap();
		test_mul_helper(a, b);
	}

	fn test_mul_helper(a: BigFloat, b: BigFloat) {
		let expected = &a * &b;
		let a_real = Real::from(a);
		let b_real = Real::from(b);
		let prod = a_real * b_real;
		let precs = [-256, 0, 256, 1024];

		for prec in precs {
			let actual = prod.eval(prec);
			let epsilon = BigFloat::ONE >> prec;
			let delta = (&actual - &expected).abs();

			println!("expected: {expected}\nactual: {actual}\ndelta: {delta}\nepsilon:{epsilon}\n");
			assert!(delta < epsilon);
		}
	}
}
