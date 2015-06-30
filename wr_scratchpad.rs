#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]
extern crate wrcore;

use wrcore::types::wr_char;


pub fn changer(v: &mut u32) {
    *v = b'*' as u32;
}

#[no_mangle]
pub fn wrcore_scratchpad() {
    let mut out: &[u8] = b"WrongCore Scratchpad\n\0";
    wrcore::kernel::print(out);
    

    let out = b"Wrongcore: Done\n\0";
    wrcore::kernel::print(out);

}
