#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]

extern crate wrcore;
#[repr(C)]
struct KlpReloc {
    loc: u64,
    val: u64,
    ptype: u64,
    name: *const str;
    addend: i32,
    external: i32,
}

#[repr(C)]
struct KlpFunc {
    old_name: *const str,
    new_func: *const w_void,
    old_addr: u64,

};

#[repr(C)]
struct KlpObject {
    name: *const str,
    relocs: *const KlpReloc,
    funcs: *const KlpFunc,
}

#[repr(C)]
struct KlpPatch {
    
}


extern "C" {
    il_create_patch_func(old_name: *const str, new_func: *const w_void, old_addr: *const w_void);
}

enum WrKlpState {
    Disabled,
    Enabled
}

#![no_mangle]
wr_create_patch_func() {

}
