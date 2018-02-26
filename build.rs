extern crate gcc;

use gcc::Build;
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    File::create(out.join("nrf52810_xxaa.ld"))
        .unwrap()
        .write_all(include_bytes!("nrf52810_xxaa.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=nrf52_xxaa.ld");
}
