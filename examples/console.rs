//! Console

#![no_std]

extern crate nrf52dk_rs;

#[link_section = ".data"]
static BUFFER: [u8; 5] = [0x48, 0x45, 0x4c, 0x4c, 0x4f];

const BAUDRAUTE: u32 = 115200;

fn main() {
    unsafe {
        let uart = &nrf52dk_rs::peripherals::uart::UART;
        uart.initialize(BAUDRAUTE);
        uart.transmit(&BUFFER);
    }
}
