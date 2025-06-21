use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;

impl BigFloat {
	/// Divides self by rhs. Absolute error < 2^-prec.
	pub fn div(&self, rhs: &BigFloat, prec: i64) -> BigFloat {
		let r = rhs.reciprocal(prec + self.ilog2() + 1);
		self.mul_with_precision(&r, prec)
	}

	/// Divides 1 by self. Absolute error < 2^-prec.
	pub fn reciprocal(&self, prec: i64) -> BigFloat {
		if self.is_zero() {
			panic!("Cannot divide by zero");
		}

		if let Some(_) = self.ilog2_exact() {
			return BigFloat::from_mantissa_exponent(
				if self.is_negative() {
					BigInt::NEG_ONE
				} else {
					BigInt::ONE
				},
				-self.e,
			);
		}

		// Estimate the quotient using integer division.
		// Extra precision bits for the estimate. Has to be at least 2.
		let shift = self.m.magnitude.ilog2() + 64;
		let mut n = BigInt::ONE << shift;
		let mut d = self.m.clone();
		let m_est = &mut n / &mut d;
		let est_0 = BigFloat::from_mantissa_exponent(m_est, -self.e - shift as i64);

		// Use Newton-Raphson method to correct estimate.
		let log_epsilon = (&BigFloat::from(1) - &(self * &est_0)).ilog2() + 1;
		debug_assert!(log_epsilon < 0, "bad estimate for reciprocal");
		let target_prec = self.ilog2() - prec;
		let mut current_prec = log_epsilon;
		let mut est_n = est_0;
		// Hopefully enough... 🙏
		let working_prec = prec + 16;
		// Loop until (ilog2(1 - self * est_0) + 1) * 2^n <= ilog2(self) - prec
		while current_prec > target_prec {
			// est_n+1 = est_n * (2 - self * est_n)
			let prod = self.mul_with_precision(&est_n, working_prec);
			let diff = BigFloat::from(2).sub_with_precision(&prod, working_prec);
			est_n = est_n.mul_with_precision(&diff, working_prec);
			current_prec *= 2;
		}
		est_n.round_to_precision(prec);
		est_n
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::biguint::BigUInt;
	use crate::rational::Rational;

	#[test]
	fn test_reciprocal() {
		let pos = BigFloat::from(3);
		test_reciprocal_helper(&pos, 1024);

		// Test negative numbers
		let neg = BigFloat::from(-5);
		test_reciprocal_helper(&neg, 1024);

		// Test powers of two
		let pow2 = BigFloat::from(4);
		test_reciprocal_helper(&pow2, 1024);

		// Test larger numbers
		let large = BigFloat::from(1000);
		test_reciprocal_helper(&large, 1024);

		// Test smaller numbers
		let small = BigFloat::from_mantissa_exponent(BigInt::from(1), -10);
		test_reciprocal_helper(&small, 1024);

		// Test should panic for zero
		let zero = BigFloat::from(0);
		assert!(std::panic::catch_unwind(|| zero.reciprocal(1024)).is_err());
	}

	fn test_reciprocal_helper(d: &BigFloat, prec: i64) {
		let q = d.reciprocal(prec).to_rational();
		let q_rat = d.to_rational().reciprocal();
		let epsilon = Rational::new(BigInt::ONE, BigUInt::ONE << prec);
		let delta = (&q - &q_rat).abs();

		print!("q = {q}\nq_rat={q_rat}\nepsilon={epsilon}\ndelta={delta}\n\n");

		assert!(delta < epsilon, "|{q} - {q_rat}| = {delta} > {epsilon}")
	}

	#[test]
	fn test_div() {
		// Test positive numbers
		let n = BigFloat::from(10);
		let d = BigFloat::from(3);
		test_div_helper(&n, &d, 1024);

		// Test negative numbers
		let n = BigFloat::from(-15);
		let d = BigFloat::from(7);
		test_div_helper(&n, &d, 1024);

		// Test both negative numbers
		let n = BigFloat::from(-20);
		let d = BigFloat::from(-3);
		test_div_helper(&n, &d, 1024);

		// Test powers of two
		let n = BigFloat::from(16);
		let d = BigFloat::from(4);
		test_div_helper(&n, &d, 1024);

		// Test larger numbers
		let n = BigFloat::from(1000);
		let d = BigFloat::from(7);
		test_div_helper(&n, &d, 1024);

		// Test smaller numbers
		let n = BigFloat::from_mantissa_exponent(BigInt::from(1), -10);
		let d = BigFloat::from_mantissa_exponent(BigInt::from(1), -5);
		test_div_helper(&n, &d, 1024);

		// Test division by zero should panic
		let n = BigFloat::from(1);
		let d = BigFloat::from(0);
		assert!(std::panic::catch_unwind(|| n.div(&d, 1024)).is_err());
	}

	fn test_div_helper(n: &BigFloat, d: &BigFloat, prec: i64) {
		let q = n.div(d, prec).to_rational();
		let q_rat = &n.to_rational() / &d.to_rational();
		let epsilon = Rational::new(BigInt::ONE, BigUInt::ONE << prec);
		let delta = (&q - &q_rat).abs();

		print!("q = {q}\nq_rat={q_rat}\nepsilon={epsilon}\ndelta={delta}\n\n");

		assert!(delta < epsilon, "|{q} - {q_rat}| = {delta} > {epsilon}")
	}
}
