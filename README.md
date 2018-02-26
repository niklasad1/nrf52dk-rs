# [WIP] nrf52dk-bare-metal-rs

## Requirements
1. [Rust](http://www.rust-lang.org/) (install via `rustup`)
2. [Xargo](http://www.rust-lang.org/) (Rust `cargo` wrapper that installs core library for embedded targets)
3. [arm-none-eabi toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)

### Installation

#### Rust
```
$ curl https://sh.rustup.rs -sSf | sh
$ rustup install nightly-2017-08-16
$ rustup component add rust-src
```

#### Xargo
```bash
$ cargo install xargo
```

#### arm-none-eabi toolchain
Use your packet manager or download it from [here](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)


## Usage

### Build
```bash
$ make
```
### Build and flash via JLink
```bash
$ make flash
```
 
### Debug via gdb
```bash
$ cd jtag
$ ./JLinkGDBServer.sh
$ arm-none-eabi-gdb -x gdbinit.jlink
$ b reset_handler
$ b main
```
