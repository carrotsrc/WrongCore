use ::types::wr_char;
extern "C" {
	fn printk(fmt: &[wr_char]) -> i32;
}

pub fn print(fmt: &[wr_char]) {
    unsafe{ printk(fmt); }
}
