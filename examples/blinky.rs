#![no_std]

extern crate nrf52dk_rs;

use nrf52dk_rs::peripherals::gpio::GPIO;
use nrf52dk_rs::board::LEDS;

fn main() {
    let gpio = &GPIO;

    // configure LEDs
    for led in LEDS.iter() {
        unsafe {
            gpio.make_output(*led);
        }
    }

    loop {
        for led in LEDS.iter() {
            unsafe { gpio.toggle(*led) }
            dummy_sleep();
        }
    }
}

fn dummy_sleep() {
    for _ in 0..10000 {}
}
