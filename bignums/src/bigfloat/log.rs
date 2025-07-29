use crate::bigfloat::BigFloat;

impl BigFloat {
	/// Returns log2(|self|) if self is a power of 2, otherwise None.
	pub fn ilog2_exact(&self) -> Option<i64> {
		if self.is_zero() {
			None
		} else if self.m.magnitude.is_one() {
			Some(self.e)
		} else {
			None
		}
	}

	/// Returns floor(log2(|self|)).
	/// Panics for 0.
	pub fn ilog2(&self) -> i64 {
		self.m.magnitude.ilog2() as i64 + self.e
	}

	pub fn ln(&self, prec: i64) -> BigFloat {
		if self.is_negative() {
			panic!("ln(negative)");
		}
		if self.is_zero() {
			panic!("ln(0)");
		}
		if self.is_one() {
			return BigFloat::ZERO;
		}

		// ln(x) = π / (2 * AGM(1, 4/x)) - shift * ln(2)
		let actual_prec = i64::max(prec + 2, -7);
		let working_prec = actual_prec * 2 + 16;

		let mut x = self.clone();
		let shift = 5 + actual_prec / 2 + (actual_prec + 8).ilog2() as i64 - x.ilog2();
		x <<= shift;

		let pi = BigFloat::pi(working_prec);
		let four_over_x = x.reciprocal(working_prec) << 2u32;
		let agm = BigFloat::agm(&BigFloat::ONE, &four_over_x, working_prec);
		let ln_x = pi.div(&agm, working_prec) >> 1u32;
		let ln2 = BigFloat::ln2(working_prec + shift.abs().max(1).ilog2() as i64);
		let ln2_times_shift = ln2.mul_with_precision(&BigFloat::from(shift), working_prec);
		let mut res = ln_x.sub_with_precision(&ln2_times_shift, working_prec);

		res.round_to_precision(actual_prec);
		res
	}
}

#[cfg(test)]
mod tests {
	use core::ops::Sub;

	use super::*;

	#[test]
	fn test_ln() {
		let a = "0.000000002323";
		let a_ln = "-19.88040638315813906652494635748918521587995496150723522047003407242586\
		618654108759312888526218022800790095502167768586465016577849230402845083884074305697870851\
		678338952169677441820085326324450733095675376259353858927130922816414965074414978063608055\
		858690656957630232393475200893213726";
		test_ln_helper(a, a_ln, 200);

		let a = "0.5";
		let a_ln = "-0.693147180559945309417232121458176568075500134360255254120680009493393\
		621969694715605863326996418687542001481020570685733685520235758130557032670751635075961930\
		727570828371435190307038623891673471123350115364497955239120475172681574932065155524734139\
		525882950453007095326366642654104239";
		test_ln_helper(a, a_ln, 200);

		let a = "1.01516156165";
		let a_ln = "0.0150477738663162482082904107532686210709481189116613800720673736761547\
		880333261934168339558281931255837542000841862940017158166207306782213535779330582450176164\
		371989964853184184648442258698201698796809915415493995708888181522109874729155173547185647\
		6676073307580262178016487596497160703";
		test_ln_helper(a, a_ln, 200);

		let a = "2";
		let a_ln = "0.6931471805599453094172321214581765680755001343602552541206800094933936\
		219696947156058633269964186875420014810205706857336855202357581305570326707516350759619307\
		275708283714351903070386238916734711233501153644979552391204751726815749320651555247341395\
		2588295045300709532636664265410423916";
		test_ln_helper(a, a_ln, 200);

		let a = "100000";
		let a_ln = "11.512925464970228420089957273421821038005507443143864880166639504837863\
		048386762401179986025447991491709838920211431243167047627325414033783331436845493908447414\
		536041627773404218999474131165992641967526544826888663144230816831111438491099433732718337\
		372021216371825775244671574696957398097022001110525508570874001844042006323540342783871608\
		114177610057402331857829560686725373928473841731808396050903222535324000138751342458373275\
		293428467836710335290568214612277202879462862104120657347844508379470128388155678459646016\
		688293570830115052851544817286037720185423734970084";
		test_ln_helper(a, a_ln, 1000);
	}

	fn test_ln_helper(x: &str, expected: &str, prec: i64) {
		let x = BigFloat::from_str_with_precision(x, prec + 64).unwrap();
		let expected = BigFloat::from_str_with_precision(expected, prec + 64).unwrap();
		let actual = x.ln(prec);
		let delta = actual.sub(&expected).abs();
		let epsilon = BigFloat::ONE >> prec;

		print!("expected: {expected}\nactual: {actual}\ndelta: {delta}\nepsilon: {epsilon}\n\n");
		assert!(delta < epsilon);
	}
}
