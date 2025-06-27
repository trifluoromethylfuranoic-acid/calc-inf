use crate::bigfloat::BigFloat;

impl BigFloat {
	pub fn ln2(prec: i64) -> BigFloat {
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
}

#[cfg(test)]
mod tests {
	use super::*;
	use core::str::FromStr;

	#[test]
	fn test_ln2() {
		test_ln2_helper(64);
		test_ln2_helper(128);
		test_ln2_helper(256);
	}

	fn test_ln2_helper(prec: i64) {
		let ln2 = BigFloat::ln2(prec);
		let epsilon = BigFloat::ONE >> prec;
		// Known value of ln(2) to high precision
		let known_ln2 = BigFloat::from_str(
			"0.693147180559945309417232121458176568075500134360255254120680009493393621969694\
			71560586332699641868754200148102057068573368552023575813055703267075163507596193072757\
			08283714351903070386238916734711233501153644979552391204751726815749320651555247341395\
			2588295045300709532636664265410423916"
		).unwrap();
		let delta = (&ln2 - &known_ln2).abs();

		print!("ln2 = {ln2:?}\nknown_ln2={known_ln2:?}\nepsilon={epsilon:?}\ndelta={delta:?}\n\n");

		assert!(
			delta < epsilon,
			"|{ln2:?} - {known_ln2:?}| = {delta:?} > {epsilon:?}"
		);
	}
}
