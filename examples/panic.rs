#![no_std]

//! Panic example

extern crate nrf52dk_rs;

fn main() {
    panic!("You should enter `rust_begin_unwind`");
}
