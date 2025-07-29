mod add;
mod consts;
mod convert;
mod div;
mod log;
mod mul;
mod set_val;
mod sqrt;
mod str;
mod sub;

use alloc::boxed::Box;
use core::fmt::Debug;

use dyn_clone::DynClone;

use crate::bigfloat::BigFloat;

pub trait RealInner: Fn(i64) -> BigFloat + DynClone {}

impl<T> RealInner for T where T: Fn(i64) -> BigFloat + DynClone {}

dyn_clone::clone_trait_object!(RealInner);

#[derive(Clone)]
pub struct Real {
	pub eval: Box<dyn RealInner>,
}

impl Real {
	pub fn zero() -> Self {
		Real::new(|_prec| BigFloat::ZERO)
	}

	pub fn one() -> Self {
		Self::new(|_prec| BigFloat::ONE)
	}

	pub fn neg_one() -> Self {
		Self::new(Box::new(|_prec| BigFloat::NEG_ONE))
	}

	pub fn new(eval: impl RealInner + 'static) -> Self {
		Self {
			eval: Box::new(eval),
		}
	}

	pub fn eval(&self, prec: i64) -> BigFloat {
		(self.eval)(prec)
	}

	pub fn set_zero(&mut self) {
		*self = Self::zero();
	}

	pub fn set_one(&mut self) {
		*self = Self::one();
	}

	pub fn abs(&self) -> Self {
		let x = self.clone();
		Self::new(move |prec| x.eval(prec).abs())
	}
}

impl Default for Real {
	fn default() -> Self {
		Self::zero()
	}
}
