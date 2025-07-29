use crate::real::Real;

impl Real {
	pub fn sqrt(self) -> Self {
		Real::new(move |prec| {
			let actual_prec = prec + 1;
			let prec_x = actual_prec * 2;

			let x = self.eval(prec_x).abs();

			x.sqrt(actual_prec)
		})
	}
}
