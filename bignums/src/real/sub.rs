use core::ops::{Neg, Sub, SubAssign};

use crate::real::Real;

impl Neg for Real {
	type Output = Real;

	fn neg(self) -> Self::Output {
		Real::new(move |prec| -self.eval(prec))
	}
}

impl Sub<Real> for Real {
	type Output = Real;

	fn sub(self, rhs: Real) -> Self::Output {
		Real::new(move |prec| {
			let a = self.eval(prec + 2);
			let b = rhs.eval(prec + 2);
			a.sub_with_precision(&b, prec + 2)
		})
	}
}

impl SubAssign for Real {
	fn sub_assign(&mut self, rhs: Self) {
		*self = self.clone().sub(rhs);
	}
}
