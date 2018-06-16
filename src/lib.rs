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

// #![deny(missing_docs)]
// #![deny(warnings)]
#![crate_type = "staticlib"]
#![feature(asm, extern_prelude, lang_items, start, naked_functions, const_fn, panic_implementation)]
#![no_std]

extern crate cortex_m;
// extern crate vcell;
#[macro_use]
extern crate tock_regs;

mod lang_items;

/// Board specific definitions
pub mod board;
/// Drivers for peripherals
pub mod peripherals;

use cortex_m::interrupt::Nr;

#[allow(non_camel_case_types)]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK,
    #[doc = "1 - RADIO"]
    RADIO,
    #[doc = "2 - UARTE0_UART0"]
    UARTE0_UART0,
    #[doc = "3 - SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0"]
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
    #[doc = "4 - SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1"]
    SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1,
    #[doc = "5 - NFCT"]
    NFCT,
    #[doc = "6 - GPIOTE"]
    GPIOTE,
    #[doc = "7 - SAADC"]
    SAADC,
    #[doc = "8 - TIMER0"]
    TIMER0,
    #[doc = "9 - TIMER1"]
    TIMER1,
    #[doc = "10 - TIMER2"]
    TIMER2,
    #[doc = "11 - RTC0"]
    RTC0,
    #[doc = "12 - TEMP"]
    TEMP,
    #[doc = "13 - RNG"]
    RNG,
    #[doc = "14 - ECB"]
    ECB,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR,
    #[doc = "16 - WDT"]
    WDT,
    #[doc = "17 - RTC1"]
    RTC1,
    #[doc = "18 - QDEC"]
    QDEC,
    #[doc = "19 - COMP_LPCOMP"]
    COMP_LPCOMP,
    #[doc = "20 - SWI0_EGU0"]
    SWI0_EGU0,
    #[doc = "21 - SWI1_EGU1"]
    SWI1_EGU1,
    #[doc = "22 - SWI2_EGU2"]
    SWI2_EGU2,
    #[doc = "23 - SWI3_EGU3"]
    SWI3_EGU3,
    #[doc = "24 - SWI4_EGU4"]
    SWI4_EGU4,
    #[doc = "25 - SWI5_EGU5"]
    SWI5_EGU5,
    #[doc = "26 - TIMER3"]
    TIMER3,
    #[doc = "27 - TIMER4"]
    TIMER4,
    #[doc = "28 - PWM0"]
    PWM0,
    #[doc = "29 - PDM"]
    PDM,
    #[doc = "32 - MWU"]
    MWU,
    #[doc = "33 - PWM1"]
    PWM1,
    #[doc = "34 - PWM2"]
    PWM2,
    #[doc = "35 - SPIM2_SPIS2_SPI2"]
    SPIM2_SPIS2_SPI2,
    #[doc = "36 - RTC2"]
    RTC2,
    #[doc = "37 - I2S"]
    I2S,
    #[doc = "38 - FPU"]
    FPU,
}

unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::POWER_CLOCK => 0,
            Interrupt::RADIO => 1,
            Interrupt::UARTE0_UART0 => 2,
            Interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => 3,
            Interrupt::SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 => 4,
            Interrupt::NFCT => 5,
            Interrupt::GPIOTE => 6,
            Interrupt::SAADC => 7,
            Interrupt::TIMER0 => 8,
            Interrupt::TIMER1 => 9,
            Interrupt::TIMER2 => 10,
            Interrupt::RTC0 => 11,
            Interrupt::TEMP => 12,
            Interrupt::RNG => 13,
            Interrupt::ECB => 14,
            Interrupt::CCM_AAR => 15,
            Interrupt::WDT => 16,
            Interrupt::RTC1 => 17,
            Interrupt::QDEC => 18,
            Interrupt::COMP_LPCOMP => 19,
            Interrupt::SWI0_EGU0 => 20,
            Interrupt::SWI1_EGU1 => 21,
            Interrupt::SWI2_EGU2 => 22,
            Interrupt::SWI3_EGU3 => 23,
            Interrupt::SWI4_EGU4 => 24,
            Interrupt::SWI5_EGU5 => 25,
            Interrupt::TIMER3 => 26,
            Interrupt::TIMER4 => 27,
            Interrupt::PWM0 => 28,
            Interrupt::PDM => 29,
            Interrupt::MWU => 32,
            Interrupt::PWM1 => 33,
            Interrupt::PWM2 => 34,
            Interrupt::SPIM2_SPIS2_SPI2 => 35,
            Interrupt::RTC2 => 36,
            Interrupt::I2S => 37,
            Interrupt::FPU => 38,
        }
    }
}

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

    // Start all clocks
    let clock = &peripherals::clock::CLOCK;

    clock.low_stop();
    clock.high_stop();

    clock.low_set_source(peripherals::clock::LowClockSource::XTAL);
    clock.low_start();
    clock.high_set_source(peripherals::clock::HighClockSource::XTAL);
    clock.high_start();
    while !clock.low_started() {}
    while !clock.high_started() {}

    let mut peripherals = cortex_m::Peripherals::take().unwrap();
    peripherals.NVIC.enable(Interrupt::UARTE0_UART0);
    cortex_m::interrupt::enable();

    main(0, core::ptr::null());
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
