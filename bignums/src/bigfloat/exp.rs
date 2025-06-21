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
}
