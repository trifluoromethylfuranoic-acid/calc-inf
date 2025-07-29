use crate::bigfloat::BigFloat;
use crate::real::Real;

impl Real {
	pub fn ln2() -> Real {
		Real::new(BigFloat::ln2)
	}

	pub fn sqrt2() -> Real {
		Real::new(BigFloat::sqrt2)
	}

	pub fn inv_sqrt2() -> Real {
		Real::new(BigFloat::inv_sqrt2)
	}

	pub fn pi() -> Real {
		Real::new(BigFloat::pi)
	}
}
