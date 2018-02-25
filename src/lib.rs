#![feature(asm,compiler_builtins_lib,const_fn,global_allocator,lang_items,naked_functions)]
#![no_std]
#![crate_type="staticlib"]

extern crate compiler_builtins;

mod lang_items;

extern "C" {

    // start of the section to copy from
    static mut __etext: u32;

    // start of the section to copy to
    static mut __data_start__: u32;

    // end of the section to copy to
    // start of the BSS section
    static mut __bss_start__: u32;

    // end of BSS section
    static mut __bss_end__: u32;

    // stack pointer
    fn __StackTop();
}

#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 16] = [Some(__StackTop), // Stack pointer
                                                               Some(Reset_Handler), // Reset
                                                               Some(isr_nmi), // NMI
                                                               Some(isr_hardfault), // Hard Fault
                                                               Some(isr_mmfault), /* CM3 Memory Management Fault */
                                                               Some(isr_busfault), /* CM3 Bus Fault */
                                                               Some(isr_usagefault), /* CM3 Usage Fault */
                                                               Some(isr_reserved_1), /* Reserved - Used as NXP Checksum */
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               Some(isr_svcall), // SVCall
                                                               Some(isr_debugmon), /* Reserved for debug */
                                                               None, // Reserved
                                                               Some(isr_pendsv), // PendSV
                                                               Some(isr_systick) /* SysTick */];


#[link_section=".flashconfig"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static flashconfigbytes: [usize; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE];

#[no_mangle]
#[naked]
#[inline(never)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Reset_Handler() {
    panic!("oo")
}


#[no_mangle]
#[naked]
#[inline(never)]
pub unsafe extern "C" fn _start() {
    let mut src: *mut u32 = &mut __etext;
    let mut dest: *mut u32 = &mut __data_start__;

    // Copy flash to RAM
    while dest < &mut __bss_start__ as *mut u32 {
        *dest = *src;
        dest = ((dest as u32) + 4) as *mut u32;
        src = ((src as u32) + 4) as *mut u32;
    }

    dest = &mut __bss_start__ as *mut u32;

    // Clear bss region of RAM
    while dest < &mut __bss_end__ as *mut u32 {
        *dest = 0;
        dest = ((dest as u32) + 4) as *mut u32;
    }
}


pub fn rust_loop() {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> () {
    loop {}
}

pub unsafe extern "C" fn isr_nmi() {
    loop {}
}
pub unsafe extern "C" fn isr_hardfault() {
    loop {}
}
pub unsafe extern "C" fn isr_mmfault() {
    loop {}
}
pub unsafe extern "C" fn isr_busfault() {
    loop {}
}
pub unsafe extern "C" fn isr_usagefault() {
    loop {}
}
pub unsafe extern "C" fn isr_reserved_1() {
    loop {}
}
pub unsafe extern "C" fn isr_svcall() {
    loop {}
}
pub unsafe extern "C" fn isr_debugmon() {
    loop {}
}
pub unsafe extern "C" fn isr_pendsv() {
    loop {}
}
pub unsafe extern "C" fn isr_systick() {
    loop {}
}
