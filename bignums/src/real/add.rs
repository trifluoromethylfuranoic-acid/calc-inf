use core::ops::{Add, AddAssign};

use crate::real::Real;

impl Add<Real> for Real {
	type Output = Real;

	fn add(self, rhs: Real) -> Self::Output {
		Real::new(move |prec| {
			let a = self.eval(prec + 2);
			let b = rhs.eval(prec + 2);
			a.add_with_precision(&b, prec + 2)
		})
	}
}

impl AddAssign for Real {
	fn add_assign(&mut self, rhs: Self) {
		*self = self.clone().add(rhs);
	}
}
