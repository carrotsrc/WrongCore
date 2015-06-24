#![crate_type="lib"]
#![crate_name="wrcore"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]

#[lang="sized"]
pub trait Sized {}


#[lang="copy"]
pub trait Copy {}

#[lang="sync"]
pub trait Sync {}

/* Quick note:
 *
 * The following macros were lifted from
 * Rust-lang, released under MIT
 */

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
	/// This method tests for `self` and `other` values to be equal, and is used by `==`.
	fn eq(&self, other: &Rhs) -> bool;
	/// This method tests for `!=`.
	#[inline]
	fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

pub trait Eq: PartialEq<Self> {
	#[inline(always)]
	fn assert_receiver_is_total_eq(&self) {}
}


macro_rules! partial_eq_impl {
	($($t:ty)*) => ($(
		impl PartialEq for $t {
			#[inline]
			fn eq(&self, other: &$t) -> bool { (*self) == (*other) }
			#[inline]
			fn ne(&self, other: &$t) -> bool { (*self) != (*other) }
		}
	)*)
}

 macro_rules! eq_impl {
($($t:ty)*) => ($(
	impl Eq for $t {}
	)*)
}
partial_eq_impl! {i32}
eq_impl! {i32}

/* ---- */

pub mod types {
	pub type wr_char = u8;
}

/* WrongCore kernel modules */
pub mod kernel;
//pub mod fs;

