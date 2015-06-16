#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]


// TODO: Work out how to put this lot somewhere else!
#[lang="sized"]
pub trait Sized {}

#[lang="copy"]
pub trait Copy {}

#[lang="sync"]
pub trait Sync {}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
	/// This method tests for `self` and `other` values to be equal, and is used by `==`.
	fn eq(&self, other: &Rhs) -> bool;
	/// This method tests for `!=`.
	#[inline]
	fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

pub trait Eq: PartialEq<Self> {
	// FIXME #13101: this method is used solely by #[deriving] to
	// assert that every component of a type implements #[deriving]
	// itself, the current deriving infrastructure means doing this
	// assertion without using a method on this trait is nearly
	// impossible.
	//
	// This should never be implemented by hand.
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
partial_eq_impl! { i32 }
eq_impl! { i32}


#[repr(C)]
struct Mutex;

extern "C" {
	fn printk(fmt: &'static str, ...) -> i32;
	fn il_mutex_is_locked(mutex: *const Mutex) -> i32;
	fn mutex_lock(mutex: *const Mutex);
	fn mutex_unlock(mutex: *const Mutex);
	fn il_mutex_init(mutex: *const Mutex);
	fn il_mutex_allocate() -> *const Mutex;
}


#[no_mangle]
pub fn wrong_test_mutex() {

	// Have to settle for dynamic allocation
	let mutex: *const Mutex = unsafe{ il_mutex_allocate() };

	// runtime init
	unsafe { 
		il_mutex_init(mutex); 
		printk("WrongCore:1 - \n\0");
		mutex_lock(mutex);
	}	

	let rcode: i32 = unsafe { il_mutex_is_locked(mutex) };
	if rcode == 1 {
		 unsafe { printk("Mutex: Locked\n\0"); }
	} else {
		 unsafe { printk("Mutex: Unlocked\n\0"); }
	}

	unsafe{ mutex_unlock(mutex) };

	unsafe { printk("WrongCore:2 - \n\0") };
	let scode = unsafe { il_mutex_is_locked(mutex) };
	if scode == 1 {
		 unsafe { printk("Mutex: Locked\n\0"); }
	} else {
		 unsafe { printk("Mutex: Unlocked\n\0"); }
	}

}
