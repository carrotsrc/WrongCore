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

#[lang = "panic"]
pub fn panic(expr_file_line: &(&'static str, &'static str, u32)) { }

#[lang = "panic_bounds_check"]
pub fn panic_bounds_check(file_line: &(&'static str, u32),
                     index: usize, len: usize) { }


pub mod lang;
/* Quick note:
 *
 * Used the rust-lang as a basis for some of this
 * Which was original distributed under Apache2/MIT
 */

pub enum Ordering {
	Less = -1,
	Equal = 0,
	Greater = 1,
}


use ::Option::*;
pub enum Option<T> {
	None,
	Some(T),
}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
	/// This method tests for `self` and `other` values to be equal, and is used by `==`.
	fn eq(&self, other: &Rhs) -> bool;
	/// This method tests for `!=`.
	#[inline]
	fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

#[lang = "ord"]
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
	#[inline]
	fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

	#[inline]
	fn lt(&self, other: &Rhs) -> bool {
		match self.partial_cmp(other)  {
			Some(Ordering::Less) 	=> true,
			_ 		=> false
		}	
	}
	#[inline]
	fn le(&self, other: &Rhs) -> bool {
		match self.partial_cmp(other)  {
			Some(Ordering::Less)	=> true,
			Some(Ordering::Equal) 	=> true,
			_ 		=> false
		}	
	}
	#[inline]
	fn gt(&self, other: &Rhs) -> bool {
		match self.partial_cmp(other)  {
			Some(Ordering::Greater) 	=> true,
			_ 		=> false
		}	
	}
	#[inline]
	fn ge(&self, other: &Rhs) -> bool {
		match self.partial_cmp(other)  {
			Some(Ordering::Greater)	=> true,
			Some(Ordering::Equal) 	=> true,
			_ 		=> false
		}	
	}
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

macro_rules! partial_ord_impl {
	($($t:ty)*) => ($(

		impl PartialOrd for $t {

			#[inline]	
			fn partial_cmp(&self, other: &$t) -> Option<Ordering> {

				match (*self <= *other, *self >= *other) {
					(false,false) => None,
					(false, true) => Some(Ordering::Greater),
					(true, false) => Some(Ordering::Less),
					(true, true) => Some(Ordering::Equal)
				}

			}

			#[inline]
			fn lt(&self, other: &$t) -> bool { (*self) <  (*other) }
			#[inline]			
			fn le(&self, other: &$t) -> bool { (*self) <= (*other) }

			#[inline]
			fn gt(&self, other: &$t) -> bool { (*self) >  (*other) }
			#[inline]
			fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
		
		}

	)*)
}

 macro_rules! eq_impl {
($($t:ty)*) => ($(
	impl Eq for $t {}
	)*)
}

partial_eq_impl! {u8 u32 i32}

eq_impl! {u8 u32 i32}

partial_ord_impl! {u8 u32 i32}

/* ---- */

pub mod types {
	pub type wr_char = u8;
}

/* WrongCore kernel modules */
pub mod kernel;
//pub mod fs;

