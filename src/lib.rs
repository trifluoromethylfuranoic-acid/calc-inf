#![feature(box_vec_non_null)]
#![feature(assert_matches)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
#[macro_use]
extern crate smallvec;

pub mod bigint;
pub mod biguint;
pub mod error;
mod util;

/// Trait for setting the value of self to some other value
/// Can help reuse memory and avoid reallocations in certain scenarios
pub trait SetVal<Src> {
	/// Set the value of self to that of src
	/// Similar to Clone::clone_from but generic
	fn set_val(&mut self, src: Src);
}

/// Trait for fallibly setting the value of self to some other value
/// Can help reuse memory and avoid reallocations in certain scenarios
pub trait TrySetVal<Src> {
	type Error;
	/// Try set the value of self to that of src
	/// On failure self must be unchanged
	/// Similar to Clone::clone_from but generic
	fn try_set_val(&mut self, src: Src) -> Result<(), Self::Error>;
}

impl<Src, Dst> TrySetVal<Src> for Dst
where
	Dst: SetVal<Src>,
{
	type Error = core::convert::Infallible;

	fn try_set_val(&mut self, src: Src) -> Result<(), Self::Error> {
		self.set_val(src);
		Ok(())
	}
}

#[cfg(test)]
mod tests {}
