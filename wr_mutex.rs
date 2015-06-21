#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]

extern crate wrcore;

#[repr(C)]
struct Mutex;

extern "C" {

	fn printk(fmt: &'static str, ...) -> i32;

	fn mutex_lock(mutex: *const Mutex);
	fn mutex_trylock(mutex: *const Mutex) -> i32;
	fn mutex_unlock(mutex: *const Mutex);

	fn kalloc(size: u32, flags: i32);


	fn il_mutex_init(mutex: *const Mutex);
	fn il_mutex_allocate() -> *const Mutex;
	fn il_mutex_is_locked(mutex: *const Mutex) -> i32;
}


#[no_mangle]
pub fn wrcore_test_mutex() {

	// Have to settle for dynamic allocation
	let mutex: *const Mutex = unsafe{ il_mutex_allocate() };

	// runtime init
	unsafe { 
		il_mutex_init(mutex); 
		printk("WrongCore:1 - \n\0");
		mutex_lock(mutex);
	}	

	let rcode: i32 = unsafe { il_mutex_is_locked(mutex) };
	match rcode {
		1 => unsafe { printk("Mutex: Locked\n\0"); },
		_ => unsafe { printk("Mutex: Unlocked\n\0"); }
	};


	unsafe{ mutex_unlock(mutex) };

	unsafe { printk("WrongCore:2 - \n\0") };
	let scode = unsafe { il_mutex_is_locked(mutex) };
	match scode {
		1 => unsafe { printk("Mutex: Locked\n\0"); },
		_ => unsafe { printk("Mutex: Unlocked\n\0"); }
	};

}
