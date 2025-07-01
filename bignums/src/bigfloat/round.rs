use crate::bigfloat::BigFloat;
use crate::bigint::BigInt;

impl BigFloat {
	/// Rounds the number s.t. the absolute error is less than 2^-prec.
	pub fn round_to_precision(&mut self, prec: i64) {
		if self.is_zero() {
			return;
		}

		let cur_lsb_weight = self.e;
		let new_lsb_weight = -prec;

		if new_lsb_weight <= cur_lsb_weight {
			return;
		}

		let shift = new_lsb_weight - cur_lsb_weight;

		let round_up = self.m.magnitude.bit((shift - 1) as usize);

		self.m.magnitude >>= shift;
		if round_up {
			self.m.magnitude += 1;
		}

		self.e = new_lsb_weight;
		self.normalize();
	}

	/// Rounds the number down s.t. the absolute error is less than 2^-prec.
	pub fn floor_to_precision(&mut self, prec: i64) {
		if self.is_zero() {
			return;
		}

		let cur_lsb_weight = self.e;
		let new_lsb_weight = -prec;

		if new_lsb_weight <= cur_lsb_weight {
			return;
		}

		let shift = new_lsb_weight - cur_lsb_weight;

		self.m >>= shift;

		self.e = new_lsb_weight;
		self.normalize();
	}

	/// Rounds the number up s.t. the absolute error is less than 2^-prec.
	pub fn ceil_to_precision(&mut self, prec: i64) {
		if self.is_zero() {
			return;
		}

		let cur_lsb_weight = self.e;
		let new_lsb_weight = -prec;

		if new_lsb_weight <= cur_lsb_weight {
			return;
		}

		let shift = new_lsb_weight - cur_lsb_weight;

		self.m >>= shift;
		self.m += 1;

		self.e = new_lsb_weight;
		self.normalize();
	}

	pub fn round(&mut self) {
		self.round_to_precision(0);
	}

	pub fn floor(&mut self) {
		self.floor_to_precision(0);
	}

	pub fn ceil(&mut self) {
		self.ceil_to_precision(0);
	}

	pub fn trunc(&mut self) {
		if self.is_negative() {
			self.ceil();
		} else {
			self.floor();
		}
	}

	pub fn round_to_int(&self) -> BigInt {
		let mut x = self.clone();
		x.round();
		x.m << x.e
	}

	pub fn floor_to_int(&self) -> BigInt {
		let mut x = self.clone();
		x.floor();
		x.m << x.e
	}

	pub fn ceil_to_int(&self) -> BigInt {
		let mut x = self.clone();
		x.ceil();
		x.m << x.e
	}

	pub fn trunc_to_int(&self) -> BigInt {
		let mut x = self.clone();
		x.trunc();
		x.m << x.e
	}

	pub fn fract(&self) -> BigFloat {
		let mut whole = self.clone();
		whole.floor();
		self - &whole
	}

	pub fn trunc_fract(&self) -> (BigInt, BigFloat) {
		let mut whole = self.clone();
		whole.trunc();
		let fract = (self - &whole).abs();
		(whole.m << whole.e, fract)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_round_to_precision() {
		let mut f = BigFloat::try_from(1.5).unwrap();
		f.round_to_precision(0);
		assert_eq!(f, BigFloat::from(2));

		let mut f = BigFloat::try_from(2.25).unwrap();
		f.round_to_precision(1);
		assert_eq!(f, BigFloat::try_from(2.5).unwrap());

		let mut f = BigFloat::try_from(-1.5).unwrap();
		f.round_to_precision(0);
		assert_eq!(f, BigFloat::from(-2));

		let mut f = BigFloat::try_from(-2.25).unwrap();
		f.round_to_precision(1);
		assert_eq!(f, BigFloat::try_from(-2.5).unwrap());
	}

	#[test]
	fn test_floor_to_precision() {
		let mut f = BigFloat::try_from(1.75).unwrap();
		f.floor_to_precision(0);
		assert_eq!(f, BigFloat::from(1));

		let mut f = BigFloat::try_from(2.25).unwrap();
		f.floor_to_precision(1);
		assert_eq!(f, BigFloat::try_from(2.0).unwrap());

		let mut f = BigFloat::try_from(-1.75).unwrap();
		f.floor_to_precision(0);
		assert_eq!(f, BigFloat::from(-2));

		let mut f = BigFloat::try_from(-2.25).unwrap();
		f.floor_to_precision(1);
		assert_eq!(f, BigFloat::try_from(-2.5).unwrap());
	}

	#[test]
	fn test_ceil_to_precision() {
		let mut f = BigFloat::try_from(1.25).unwrap();
		f.ceil_to_precision(0);
		assert_eq!(f, BigFloat::from(2));

		let mut f = BigFloat::try_from(2.25).unwrap();
		f.ceil_to_precision(1);
		assert_eq!(f, BigFloat::try_from(2.5).unwrap());

		let mut f = BigFloat::try_from(-1.25).unwrap();
		f.ceil_to_precision(0);
		assert_eq!(f, BigFloat::from(-1));

		let mut f = BigFloat::try_from(-2.25).unwrap();
		f.ceil_to_precision(1);
		assert_eq!(f, BigFloat::try_from(-2.0).unwrap());
	}

	#[test]
	fn test_basic_rounding() {
		let mut f = BigFloat::try_from(1.5).unwrap();
		f.round();
		assert_eq!(f, BigFloat::from(2));

		let mut f = BigFloat::try_from(1.5).unwrap();
		f.floor();
		assert_eq!(f, BigFloat::from(1));

		let mut f = BigFloat::try_from(1.5).unwrap();
		f.ceil();
		assert_eq!(f, BigFloat::from(2));

		let mut f = BigFloat::try_from(-1.5).unwrap();
		f.round();
		assert_eq!(f, BigFloat::from(-2));

		let mut f = BigFloat::try_from(-1.5).unwrap();
		f.floor();
		assert_eq!(f, BigFloat::from(-2));

		let mut f = BigFloat::try_from(-1.5).unwrap();
		f.ceil();
		assert_eq!(f, BigFloat::from(-1));
	}

	#[test]
	fn test_zero_rounding() {
		let mut f = BigFloat::from(0);
		f.round_to_precision(10);
		assert_eq!(f, BigFloat::from(0));

		f.floor_to_precision(10);
		assert_eq!(f, BigFloat::from(0));

		f.ceil_to_precision(10);
		assert_eq!(f, BigFloat::from(0));
	}
}
