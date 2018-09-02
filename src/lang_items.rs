/// Lang item required to make the normal `main` work in applications
#[lang = "start"]
extern "C" fn start<T>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> i32
where
    T: Termination,
{
    main();
    0
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

/// Required by Rust
#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Panic Handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use board::LEDS;
    use peripherals::gpio::GPIO;

    unsafe {
        let gpio = &GPIO;
        for led in LEDS.iter() {
            gpio.make_output(*led);
        }

        loop {
            for _ in 0..100000 {}

            for led in LEDS.iter() {
                gpio.toggle(*led);
            }

            for _ in 0..10000 {}
        }
    }
}
