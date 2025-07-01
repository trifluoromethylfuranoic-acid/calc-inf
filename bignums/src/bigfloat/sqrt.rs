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

		let actual_prec = prec.max(8);
		let working_prec = actual_prec + 16;

		let mut x = Self::est_sqrt(self.clone());

		#[cfg(test)]
		println!("Estimated sqrt: {x}");

		let mut i = 0;
		loop {
			let q = self.div(&x, working_prec);
			let delta = x.sub_with_precision(&q, working_prec);
			x = x.add_with_precision(&q, working_prec) >> 1;

			i += 1;
			if delta.is_zero() || delta.ilog2() <= -actual_prec {
				#[cfg(test)]
				println!(
					"Iterations: {i}, log2(delta): {0}",
					if delta.is_zero() {
						i64::MIN
					} else {
						delta.ilog2()
					}
				);
				break;
			}
		}

		x.round_to_precision(prec);
		x
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

#[cfg(test)]
mod tests {
	use core::ops::Sub;
	use core::str::FromStr;

	use super::*;

	#[test]
	fn test_sqrt() {
		let a = "2";
		let a_sqrt = "1.41421356237309504880168872420969807856967187537694807317667973799073\
			247846210703885038753432764157273501384623091229702492483605585073721264412";
		test_sqrt_helper(a, a_sqrt, 1000);

		let a = "10000";
		let a_sqrt = "100";
		test_sqrt_helper(a, a_sqrt, 1000);

		let a = "0.0000000000045";
		let a_sqrt = "0.00000212132034355964257320253308631454711785450781306542210976501960\
			69860987176931605582755813014914623591025207693463684455373872540837761058189661822456\
			49903747119833998891258389133699925751729173090857205164339957408955411801895293022092\
			776127961333797914328493845726463071482176259643905";
		test_sqrt_helper(a, a_sqrt, 200);
	}

	fn test_sqrt_helper(x: &str, expected: &str, prec: i64) {
		let x = BigFloat::from_str(x).unwrap();
		let expected = BigFloat::from_str(expected).unwrap();
		let actual = x.sqrt(prec);
		let delta = actual.sub(&expected).abs();
		let epsilon = BigFloat::ONE >> prec;

		print!("expected: {expected}\nactual: {actual}\ndelta: {delta}\nepsilon: {epsilon}\n\n");
		assert!(delta < epsilon);
	}
}
