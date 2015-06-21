#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]

extern crate wrcore;
#[repr(C)]
struct KlpFunc;


extern "C" {
    il_create_patch_func();
}

enum WrKlpState {
    Disabled,
    Enabled
}
