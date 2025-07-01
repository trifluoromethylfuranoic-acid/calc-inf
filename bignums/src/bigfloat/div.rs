use core::cmp::Ordering;
use core::ops::Div;

use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;
use crate::biguint::DivRem;

impl BigFloat {
	/// Divides self by rhs. Absolute error < 2^-prec.
	pub fn div(&self, rhs: &BigFloat, prec: i64) -> BigFloat {
		let r = rhs.reciprocal(prec + self.ilog2() + 1);
		self.mul_with_precision(&r, prec + 1)
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
		let est = BigFloat::from_mantissa_exponent(m_est, -self.e - shift as i64);

		// Use Newton-Raphson method to correct estimate.
		// Need more precision to account for error from 3 sources: stopping after
		// finitely many iterations, imprecise arithmetic and rounding the final result.
		let actual_prec = prec + 2;

		let log_epsilon = (&BigFloat::from(1) - &(self * &est)).ilog2() + 1;
		debug_assert!(log_epsilon < 0, "bad estimate for reciprocal");
		let target_log_epsilon = self.ilog2() - actual_prec;
		if log_epsilon < target_log_epsilon {
			return est;
		}
		
		let q = target_log_epsilon / log_epsilon + 1;
		// Estimated number of iterations. Log_epsilon roughly doubles each iteration.
		let n = q.ilog2() as i64 + 2;
		let log_s = self.ilog2();

		let mut x = est;
		// Hopefully enough... 🙏
		let working_prec = actual_prec + n + i64::max(0, x.ilog2()) + 16;
		let mut i = 0;
		loop {
			// x_n+1 = x_n * (2 - s * x_n)
			let prod = self.mul_with_precision(&x, working_prec);

			let delta = BigFloat::from(1).sub_with_precision(&prod, working_prec);

			let diff = BigFloat::from(2).sub_with_precision(&prod, working_prec);
			x = x.mul_with_precision(&diff, working_prec);

			i += 1;

			if delta.is_zero() || delta.ilog2() <= -actual_prec + log_s - 1 {
				break;
			}
		}
		#[cfg(test)]
		println!("est_iter: {n}, actual_iter: {i}");

		x.round_to_precision(actual_prec);
		x
	}

	pub fn div_int(&self, rhs: &BigFloat) -> BigInt {
		if rhs.is_zero() {
			panic!("Cannot divide by zero");
		}
		match self.cmp_abs(rhs) {
			Ordering::Less => return BigInt::ZERO,
			Ordering::Equal => {
				return if self.is_negative() == rhs.is_negative() {
					BigInt::ONE
				} else {
					BigInt::NEG_ONE
				};
			}
			Ordering::Greater => {}
		}

		let (mut n_m, mut d_m) = self.div_int_helper(rhs);

		let q_int = (&mut n_m).div(&mut d_m);
		q_int
	}

	pub fn div_rem(&self, rhs: &BigFloat) -> (BigInt, BigFloat) {
		let q = self.div_int(rhs);
		let n = &q * rhs;
		(q, self - &n)
	}

	pub fn div_floor(&self, rhs: &BigFloat) -> BigInt {
		if rhs.is_zero() {
			panic!("Cannot divide by zero");
		}
		match self.cmp_abs(rhs) {
			Ordering::Less => return BigInt::ZERO,
			Ordering::Equal => {
				return if self.is_negative() == rhs.is_negative() {
					BigInt::ONE
				} else {
					BigInt::NEG_ONE
				};
			}
			Ordering::Greater => {}
		}

		let (mut n_m, mut d_m) = self.div_int_helper(rhs);
		let (q_int, _) = (&mut n_m).div_rem_floor(&mut d_m);
		q_int
	}

	pub fn div_rem_floor(&self, rhs: &BigFloat) -> (BigInt, BigFloat) {
		let q = self.div_floor(rhs);
		let n = &q * rhs;
		(q, self - &n)
	}

	fn div_int_helper(&self, rhs: &BigFloat) -> (BigInt, BigInt) {
		let mut n_m = self.m.clone();
		let mut d_m = rhs.m.clone();

		match Ord::cmp(&self.e, &rhs.e) {
			Ordering::Less => {
				d_m <<= rhs.e - self.e;
			}
			Ordering::Equal => {}
			Ordering::Greater => {
				n_m <<= self.e - rhs.e;
			}
		}

		(n_m, d_m)
	}
}

#[cfg(test)]
mod tests {
	use core::str::FromStr;
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

		let small = BigFloat::from_str("0.00000000000000000000000001561").unwrap();
		test_reciprocal_helper(&small, 10240);

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

	#[test]
	fn test_div_int() {
		// Test positive numbers
		let n = BigFloat::from(10);
		let d = BigFloat::from(3);
		test_div_int_helper(&n, &d);

		// Test negative numbers
		let n = BigFloat::from(-15);
		let d = BigFloat::from(7);
		test_div_int_helper(&n, &d);

		// Test both negative numbers
		let n = BigFloat::from(-20000);
		let d = BigFloat::from(-3);
		test_div_int_helper(&n, &d);

		// Test division by zero should panic
		let n = BigFloat::from(1);
		let d = BigFloat::from(0);
		assert!(std::panic::catch_unwind(|| n.div_int(&d)).is_err());

		// Test when result is zero
		let n = BigFloat::from(2);
		let d = BigFloat::from(3);
		assert_eq!(n.div_int(&d), BigInt::ZERO);
	}

	fn test_div_int_helper(n: &BigFloat, d: &BigFloat) {
		let q = n.div_int(d);
		let mut expected = &n.to_rational() / &d.to_rational();
		let expected = if expected.is_negative() {
			expected.ceil_to_int()
		} else {
			expected.floor_to_int()
		};
		assert_eq!(q, expected);
	}
}
