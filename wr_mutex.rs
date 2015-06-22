#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]
extern crate wrcore;

#[repr(C)]
struct Mutex;

extern "C" {
	fn mutex_lock(mutex: *const Mutex);
	fn mutex_trylock(mutex: *const Mutex) -> i32;
	fn mutex_unlock(mutex: *const Mutex);

	fn il_mutex_init(mutex: *const Mutex);
	fn il_mutex_allocate() -> *const Mutex;
	fn il_mutex_is_locked(mutex: *const Mutex) -> i32;
}


#[no_mangle]
pub fn wrcore_test_mutex() {

	// Have to settle for dynamic allocation
	wrcore::kernel::print("WrongCore: Init OK\n\0");
	let mutex: *const Mutex = unsafe{ il_mutex_allocate() };
	wrcore::kernel::print("WrongCore: Allocated mutex\n\0");


	// runtime init
	unsafe { il_mutex_init(mutex); }

//	wrcore::kernel::print("WrongCore: init and ready\n\0");

//	wrcore::kernel::print("WrongCore: Initialised Mutex\n\0");
/*
	unsafe{ mutex_lock(mutex); }	
	wrcore::kernel::print("WrongCore:1 - \n\0");
	let rcode: i32 = unsafe { il_mutex_is_locked(mutex) };
	match rcode {
		1 => wrcore::kernel::print("Mutex: Locked\n\0"),
		_ => wrcore::kernel::print("Mutex: Unlocked\n\0")
	};


	unsafe{ mutex_unlock(mutex) };

	wrcore::kernel::print("WrongCore:2 - \n\0");
	let scode = unsafe { il_mutex_is_locked(mutex) };
	match scode {
		1 => wrcore::kernel::print("Mutex: Locked\n\0"),
		_ => wrcore::kernel::print("Mutex: Unlocked\n\0")
	};
*/

}
