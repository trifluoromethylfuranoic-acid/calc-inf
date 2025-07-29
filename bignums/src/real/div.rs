use core::cmp::Ordering;

use crate::bigfloat::BigFloat;
use crate::real::Real;

impl Real {
	pub fn div(self, rhs: Self, tol: i64) -> Result<Real, Real> {
		let d_round = rhs.eval(tol);
		let tau = BigFloat::ONE >> tol;
		if d_round.cmp_abs(&tau) != Ordering::Greater {
			return Err(self);
		}
		let n_round = self.eval(0);
		let n_ilog2 = if n_round.is_zero() {
			0
		} else {
			n_round.ilog2()
		};
		let d_ilog2 = if d_round.is_zero() {
			0
		} else {
			d_round.ilog2()
		};

		let d_lower_bound_ilog2 = if d_round.is_negative() {
			(&d_round + &tau).ilog2()
		} else {
			(&d_round - &tau).ilog2()
		};

		Ok(Real::new(move |prec| {
			let actual_prec = prec + 1;
			let prec_d = actual_prec + 2 * d_lower_bound_ilog2 + n_ilog2 + 2;
			let prec_n = actual_prec + 2 * d_lower_bound_ilog2 + d_ilog2 + 2;

			let d = rhs.eval(prec_d);
			let n = self.eval(prec_n);

			n.div(&d, actual_prec)
		}))
	}
}
