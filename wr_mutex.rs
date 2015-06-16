#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]

#[lang="sized"]
trait Sized {}

#[lang="copy"]
trait Copy {}

#[repr(C)]
struct Mutex;

extern "C" {
	fn printk(fmt: &'static str, ...) -> i32;
	fn il_mutex_is_locked(mutex: *const Mutex) -> i32;
	fn mutex_lock(mutex: *const Mutex);
	fn mutex_unlock(mutex: *const Mutex);
	fn il_create_mutex() -> Mutex;
}


#[no_mangle]
pub fn wrong_test_mutex() {

	let mutex = unsafe { il_create_mutex(); };

//	unsafe { mutex_lock(&mutex) };
	unsafe { printk("WrongCore:1 - "); }
/*
	let mut rcode = unsafe { il_mutex_is_locked(&mutex) };
	match rcode {
		1 => unsafe { printk("Mutex: Locked\n") },
		_ => unsafe { printk("Mutex: Unlocked\n") }
	};

	unsafe{ mutex_unlock(&mutex) };

	unsafe { printk("WrongCore:2 - ") };
	let rcode = unsafe { il_mutex_is_locked(&mutex) };
	match rcode {
		1 => unsafe { printk("Mutex: Locked\n") },
		_ => unsafe { printk("Mutex: Unlocked\n") }
	};
*/
}
