#![feature(asm,compiler_builtins_lib,lang_items,naked_functions)]
#![no_std]
#![crate_type="staticlib"]

extern crate compiler_builtins;

mod lang_items;

use core::ptr;

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
    fn __stack();
}

#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static BASE_VECTORS: [Option<unsafe extern "C" fn()>; 16] = [
    Some(__stack), // Stack pointer
    Some(reset_handler), // Reset
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
    Some(isr_systick) /* SysTick */
];

// Placeholder
#[link_section = ".irqs"]
#[no_mangle] // Ensures that the symbol is kept until the final binary
pub static ISR: [unsafe extern "C" fn(); 80] = [generic_isr; 80];

#[allow(non_snake_case)]
#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn reset_handler() {
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
    
    start_program();
}

#[no_mangle]
#[inline(never)]
pub fn init() {
}

// Call main program/example last entry point
pub unsafe fn start_program() -> ! {
    extern "C" {
        // This function is created internally by`rustc`. See `src/lang_items.rs` for more details.
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    main(0, ptr::null());
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
pub unsafe extern "C" fn generic_isr() {
    loop {}
}
