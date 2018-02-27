/// Lang item required to make the normal `main` work in applications
#[lang = "start"]
extern "C" fn start(main: fn(), _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

/// Required by Rust
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Panic Handler
#[lang = "panic_fmt"]
unsafe extern "C" fn rust_begin_unwind() {
    use board::LEDS;
    use peripherals::gpio::GPIO;

    let gpio = &GPIO;

    for led in LEDS.iter() {
        gpio.make_output(*led);
        gpio.clear(*led);
    }

    loop {}
}
