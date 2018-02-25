#[lang = "start"]
extern "C" fn start(main: fn(), _argc: isize, _argv: *const *const u8) -> isize {
    main();

    0
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
unsafe extern "C" fn rust_begin_unwind() {
    loop {}
}
