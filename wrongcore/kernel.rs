extern "C" {
	fn printk(fmt: &'static str) -> i32;
}

pub fn print(fmt: &'static str) -> i32 {
    unsafe{ printk(fmt) }
}
