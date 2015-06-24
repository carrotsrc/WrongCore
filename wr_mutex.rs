#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]
extern crate wrcore;

use wrcore::types::wr_char;

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
	let mut out: &[wr_char] = b"WrongCore\n\0";

	wrcore::kernel::print(out);
	let mutex = unsafe{ il_mutex_allocate() };
	
	out = b"WrongCore: Allocated OK\n\0";
	wrcore::kernel::print(out);

	// runtime init
	unsafe { il_mutex_init(mutex); }
	out = b"WrongCore: Init OK\n\0";
	wrcore::kernel::print(out);


	
	let mut lchk = unsafe{ il_mutex_is_locked(mutex) };
	let out = match lchk {
		1 => b"WrongCore: Mutex Locked\n\0",
		_ => b"WrongCore: Mutex Unlock\n\0"
	};
	wrcore::kernel::print(out);

	unsafe { mutex_lock(mutex); }
	lchk = unsafe{ il_mutex_is_locked(mutex) };
	let out = match lchk {
		1 => b"WrongCore: Mutex Locked\n\0",
		_ => b"WrongCore: Mutex Unlock\n\0"
	};
	wrcore::kernel::print(out);


	unsafe { mutex_unlock(mutex); }
	lchk = unsafe{ il_mutex_is_locked(mutex) };
	let out = match lchk {
		1 => b"WrongCore: Mutex Locked\n\0",
		_ => b"WrongCore: Mutex Unlock\n\0"
	};
	wrcore::kernel::print(out);

}
