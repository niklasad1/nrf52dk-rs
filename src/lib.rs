//! nrf52dk-rs
//!
//! A crate that provides bare-metal support for nrf52dk without any depenendecies to Nordic
//! SoftDevice or similar
//!
//! Still very WIP and only a crate for playing around so far and many drivers are based 
//! on drivers from [TockOS](https://github.com/helena-project/tock)
//!
//! For more stable support for nrf52dk: 
//! Checkout [TockOS](https://github.com/helena-project/tock)

#![deny(missing_docs)]
#![deny(warnings)]
#![crate_type="staticlib"]
#![feature(asm, compiler_builtins_lib, lang_items, naked_functions, const_fn)]

#![no_std]

extern crate compiler_builtins;
extern crate cortex_m;
extern crate vcell;

mod lang_items;

/// Drivers for peripherals 
pub mod peripherals;
/// Board specific definitions
pub mod board;

use core::ptr;

/// Symbols that are exported from the linker script
extern "C" {
    /// Start of the text section to copy from
    static mut __etext: u32;

    /// Start of the RAM section to copy data from flash to
    static mut __data_start__: u32;

    /// End of the RAM section to copy data to
    /// and start of the BSS section
    static mut __bss_start__: u32;

    /// End of BSS section
    static mut __bss_end__: u32;

    /// Stack pointer, i.e., not a ordinary function
    fn __stack();
}

/// Interrupt Vector that is specified by ARM
#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[no_mangle]
pub static BASE_VECTORS: [unsafe extern "C" fn(); 16] = [
    __stack,                // Stack pointer
    reset_handler,          // Reset handler
    unhandled_interrupt,    // NMI
    hardfault_handler,      // Hard Fault
    unhandled_interrupt,    // CM3 Memory Management Fault
    unhandled_interrupt,    // CM3 Bus Fault
    unhandled_interrupt,    // CM3 Usage Fault
    unhandled_interrupt,    // Reserved 
    unhandled_interrupt,    // Reserved
    unhandled_interrupt,    // Reserved
    unhandled_interrupt,    // Reserved
    svc_handler,            // SVCall
    unhandled_interrupt,    // Reserved for debug
    unhandled_interrupt,    // Reserved
    unhandled_interrupt,    // PendSV
    systick_handler,        // SysTick
];

/// Interrupt Vector that is chip specific
// TODO: Specify NRF52 specific here
#[link_section = ".irqs"]
#[no_mangle] // Ensures that the symbol is kept until the final binary
pub static ISR: [unsafe extern "C" fn(); 80] = [generic_isr; 80];

/// First entry point
#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn reset_handler() {
    extern "C" {
        // This function is created internally by`rustc`. See `src/lang_items.rs` for more details.
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    init();

    // FIXME: Here we should enable clocks and similar things

    cortex_m::interrupt::enable();

    main(0, ptr::null());
}

/// Initilization of processor that copies data from Flash to RAM
/// and zeros out the BSS region
pub unsafe fn init() {
    let mut src: *mut u32 = &mut __etext;
    let mut dest: *mut u32 = &mut __data_start__;

    // Copy Flash to RAM
    while dest < &mut __bss_start__ as *mut u32 {
        *dest = *src;
        dest = ((dest as u32) + 4) as *mut u32;
        src = ((src as u32) + 4) as *mut u32;
    }

    dest = &mut __bss_start__ as *mut u32;

    // Clear BSS region of RAM
    while dest < &mut __bss_end__ as *mut u32 {
        *dest = 0;
        dest = ((dest as u32) + 4) as *mut u32;
    }
}

/// ARM Hard-Fault Handler
pub unsafe extern "C" fn hardfault_handler() {
    loop {}
}

/// ARM Hard-Fault Handler
pub unsafe extern "C" fn systick_handler() {
    loop {}
}

/// ARM SysTick Handler
pub unsafe extern "C" fn svc_handler() {
    loop {}
}

/// Unhandeled Interrupts
pub unsafe extern "C" fn unhandled_interrupt() {
    loop {}
}

/// Generic Interrupt Handler
pub unsafe extern "C" fn generic_isr() {
    loop {}
}
