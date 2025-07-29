use crate::bigfloat::BigFloat;
use crate::real::Real;

impl Real {
	pub fn ln(self, tol: i64) -> Result<Real, Real> {
		let x_round = self.eval(tol);
		let tau = BigFloat::ONE >> tol;
		if x_round <= tau {
			return Err(self);
		}

		Ok(Real::new(move |prec| {
			let actual_prec = prec + 1;
			let x_prec = i64::max(actual_prec, 64) + tol;
			let x = self.eval(x_prec);

			x.ln(actual_prec)
		}))
	}
}
