nrf52dk-rs

## Requirements
1. [Rust](http://www.rust-lang.org/) (install via `rustup`)
3. [arm-none-eabi toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)

### Installation

#### Rust
```
$ curl https://sh.rustup.rs -sSf | sh
# Install nightly Rust
$ rustup install nightly
# Install Rust source
$ rustup component add rust-src
# Disable incremental compilation
$ export CARGO_INCREMENTAL=0
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
$ make flash app=blinky
```
 
### Debug via gdb
```bash
$ cd jtag
$ ./JLinkGDBServer.sh
$ arm-none-eabi-gdb -x gdbinit.jlink
$ b reset_handler
$ b main
```

### Contributing

Pull requests are welcome and create issues for questions or similar
