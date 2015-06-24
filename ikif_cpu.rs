#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]
extern crate wrcore;

use wrcore::types::wr_char;

#[repr(C)]
struct Mutex;

extern "C" {
	fn il_cpuid(id: u32, a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32);
}


pub struct CpuInfoX86 {
	x86: 			u8,
	x86_vendor: 		u8,
	x86_model: 		u8,
	x86_mask: 		u8,
	
	wp_works_ok: 		wr_char,
	rfu: 			wr_char,
	pad0:			wr_char,
	pad1:			wr_char,

	x86_virt_bits:		u8,
	x86_phys_bits:		u8,
	x86_coreid_bits:	u8,
	extended_cpuid_level:	u32,

	cpuid_level:		i32,
	x86_capability:		[u32;12],
	x86_vendor_id:		[wr_char;16],
	x86_model_id:		[wr_char;64],
	
	x86_cache_size:		i32,
	x86_cache_alignment:	i32,
	x86_power:		i32,
	loops_per_jiffy:	u32, // check this

	x86_max_cores:		u16,
	apicid:			u16,
	initial_apicid:		u16,
	x86_clflush_size:	u16,

	booted_cores:		u16,

	phys_proc_id:		u16,

	cpu_core_id:		u16,

	compute_unit_id:	u8,

	cput_index:		u16,
	microcode:		u32
}

#[no_mangle]
pub fn cpu_detectl(c: &mut CpuInfoX86) {
	unsafe{ 
		il_cpuid(0x00000000, 
		&mut (c.cpuid_level 	 as u32),
		&mut (c.x86_vendor_id[0] as u32),
		&mut (c.x86_vendor_id[8] as u32),
		&mut (c.x86_vendor_id[4] as u32));
	}

	c.x86 = 4;

	if c.cpuid_level >= 0x00000001 {
		let junk: u32; let tfms: u32; let cap0: u32; let misc: u32;
	}
}

