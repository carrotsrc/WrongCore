#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate wrcore;

use wrcore::types::wr_char;

extern "C" {
	fn il_cpuid(id: u32, a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32);
    fn il_cpu_detect(info: *const CpuInfoX86);
}

// This stucture lines up with header for 32bit builds
#[cfg(wr_cpu_detect_kbuild)]
#[repr(C)]
pub struct XCpuInfoX86;

pub struct CpuInfoX86 {

    // originally these are __u8 which is an unsigned char
    x86:                    u8,
    x86_vendor:             u8,
    x86_model:              u8,
    x86_mask:               u8,


    // Testing rig is a 32bit vm, so CONFIG_X86_32 is defined
    // but just in case
    #[cfg(x86_32_kbuild)]
    wp_works_ok:            wr_char,
    #[cfg(x86_32_kbuild)]
    rfu:                    wr_char,
    #[cfg(x86_32_kbuild)]
    pad0:                   wr_char,
    #[cfg(x86_32_kbuild)]
    pad1:                   wr_char,

    #[cfg(not(x86_32_kbuild))]
    x86_tlbsize:            i32,            // int
    
    x86_virt_bits:          u8,
    x86_phys_bits:          u8,
    x86_coreid_bits:        u8,
    extended_cpuid_level:   u32,

    cpuid_level:            u32,            // int
    x86_capability:         [u32;12],       // NCAPINTS+NBUGINTS
    x86_vendor_id:          [wr_char;16],
    x86_model_id:           [wr_char;64],

    x86_cache_size:         i32,
    x86_cache_alignment:    i32,
    x86_power:              i32,

    // is a long, so check for 64bit builds
    loops_per_jiffy:        u32,

    x86_max_cores:          u16,
    apicid:                 u16,
    initial_apicid:         u16,
    x86_clflush_size:       u16,

    booted_cores:           u16,

    phys_proc_id:           u16,

    cpu_core_id:            u16,

    compute_unit_id:        u8,

    cpu_index:              u16,
    microcode:              u32
}

#[cfg(wr_cpu_detect_kbuild)]
#[no_mangle]
pub extern "C" fn cpu_detect(c: &mut CpuInfoX86) {

    // Working
    // unsafe { il_cpu_detect(c); }


    // Failing
    let mut vid_0: u32 = 0;
    let mut vid_8: u32 = 0;
    let mut vid_4: u32 = 0;

    unsafe{ 
        il_cpuid(0x00000000, 
        &mut c.cpuid_level,
        &mut vid_0,
        &mut vid_8,
        &mut vid_4);
    }

    c.x86_vendor_id[0] = vid_0 as u8;
    c.x86_vendor_id[8] = vid_8 as u8;
    c.x86_vendor_id[4] = vid_4 as u8;
    c.x86 = 4;

    if c.cpuid_level >= 0x00000001 {
        let mut junk: u32 = 0;
        let mut tfms: u32 = 0;
        let mut cap0: u32 = 0;
        let mut misc: u32 = 0;

        unsafe{ 
            il_cpuid(0x00000001,
            &mut tfms,
            &mut misc,
            &mut junk,
            &mut cap0);
        }

        c.x86 = ((tfms >> 8) & 0xf) as u8;
        c.x86_model = ((tfms >> 4) & 0xf) as u8;
        c.x86_mask = (tfms & 0xf) as u8;

        if c.x86 == 0xf {
            c.x86 += ((tfms >> 20) & 0xff) as u8;
        }
        
        if c.x86 >= 0x6 {
            c.x86_model += (((tfms >> 16) & 0xf) << 4) as u8;
        }

        if (cap0 & (1<<19)) == 1 {
            c.x86_clflush_size = (((misc >> 8) & 0xff)<<3 as i32) as u16;
            c.x86_cache_alignment = c.x86_clflush_size as i32;
        }
    }
    
}

