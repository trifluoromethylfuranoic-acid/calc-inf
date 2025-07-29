use crate::bigfloat::BigFloat;

impl BigFloat {
	pub fn ln2(prec: i64) -> BigFloat {
		Self::ln2_underestimate(prec)
	}

	pub(crate) fn ln2_underestimate(prec: i64) -> BigFloat {
		let mut k = 1i64;
		let mut res = BigFloat::ZERO;
		let working_prec = prec + prec.ilog2() as i64 + 16;

		// ln2 = sum(1/(n*2^n))
		while k <= prec {
			let a = BigFloat::from(k).reciprocal(working_prec);
			let b = a >> k;
			res += &b;
			k += 1;
		}

		res.round_to_precision(prec);
		res
	}

	pub fn sqrt2(prec: i64) -> BigFloat {
		BigFloat::from(2).sqrt(prec)
	}

	pub fn inv_sqrt2(prec: i64) -> BigFloat {
		(BigFloat::ONE >> 1u32).sqrt(prec)
	}

	pub fn pi(prec: i64) -> BigFloat {
		let actual_prec = prec + 2;
		let working_prec = actual_prec + 16;

		// Gauss-Legendre algorithm
		// https://arxiv.org/pdf/1802.07558, page 7

		let mut a = BigFloat::ONE;
		let mut b = BigFloat::inv_sqrt2(working_prec);
		let mut c;
		let mut s = BigFloat::ONE >> 2u32; // 0.25

		let mut a_new;

		let mut n = 0;

		let mut res = loop {
			a_new = a.add_with_precision(&b, working_prec) >> 1u32;
			b = a.mul_with_precision(&b, working_prec).sqrt(working_prec);

			/*let delta = a.sub_with_precision(&b, working_prec);
			if delta.is_zero() || delta.ilog2() < -actual_prec {
				#[cfg(test)]
				println!("n={n}");
				a = a_new;
				break;
			}*/

			let a2 = a.mul_with_precision(&a, working_prec);
			let a_new2 = a_new.mul_with_precision(&a_new, working_prec);
			let s_recip = s.reciprocal(working_prec);
			let lo_bound = a_new2.mul_with_precision(&s_recip, working_prec);
			let hi_bound = a2.mul_with_precision(&s_recip, working_prec);

			let delta = hi_bound.sub_with_precision(&lo_bound, working_prec);
			if delta.is_zero() || delta.ilog2() + 1 < -actual_prec {
				break lo_bound;
			}

			c = a.sub_with_precision(&a_new, working_prec);

			let c2 = c.mul_with_precision(&c, working_prec);
			s = s.sub_with_precision(&(c2 << n), working_prec);

			a = a_new;

			n += 1;
		};

		/*let a2 = a.mul_with_precision(&a, working_prec);
		let mut res = a2.div(&s, working_prec);*/

		res.round_to_precision(actual_prec);
		res
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ln2() {
		test_ln2_helper(64);
		test_ln2_helper(128);
		test_ln2_helper(256);
	}

	fn test_ln2_helper(prec: i64) {
		let ln2 = BigFloat::ln2(prec);
		let epsilon = BigFloat::ONE >> prec;
		let known_ln2 = BigFloat::from_str_with_precision(
			"0.693147180559945309417232121458176568075500134360255254120680009493393621969694\
			71560586332699641868754200148102057068573368552023575813055703267075163507596193072757\
			08283714351903070386238916734711233501153644979552391204751726815749320651555247341395\
			2588295045300709532636664265410423916",
			prec + 64
		).unwrap();
		let delta = (&ln2 - &known_ln2).abs();

		print!("ln2 = {ln2}\nknown_ln2={known_ln2}\nepsilon={epsilon}\ndelta={delta}\n\n");

		assert!(delta < epsilon);
	}

	#[test]
	fn test_pi() {
		test_pi_helper(50);
		test_pi_helper(200);
		test_pi_helper(1000);
	}

	fn test_pi_helper(prec: i64) {
		let pi = BigFloat::pi(prec);
		let epsilon = BigFloat::ONE >> prec;
		let known_pi = BigFloat::from_str_with_precision(
			"3.1415926535897932384626433832795028841971693993751058209749445923078164062862089\
			98628034825342117067982148086513282306647093844609550582231725359408128481117450284102\
			70193852110555964462294895493038196442881097566593344612847564823378678316527120190914\
			56485669234603486104543266482133936072602491412737245870066063155881748815209209628292\
			54091715364367892590360011330530548820466521384146951941511609433057270365759591953092\
			18611738193261179310511854807446237996274956735188575272489122793818301194912983367336\
			24406566430860213949463952247371907021798609437027705392171763",
			prec + 64
		).unwrap();
		let delta = (&pi - &known_pi).abs();

		println!("pi = {pi}\nknown_pi={known_pi}\nepsilon={epsilon}\ndelta={delta}\n");

		assert!(delta < epsilon);
	}
}
