TARGET=target/thumbv7em-none-eabi/debug/examples
# assumption: `llvm-tools installed`
OBJCOPY=$(shell rustc +nightly --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-objcopy

JLINK=JLinkExe
JLINK_OPTIONS+=-device nrf52 -if swd -speed 1200 -AutoConnect 1
JLINK_SCRIPTS_DIR=jtag/

.PHONY: build
build:
	cargo build --examples

.PHONY: flash
flash: 	
	cargo build --example $(app)
	$(OBJCOPY) --output-target=binary $(TARGET)/$(app) $(TARGET)/app.bin
	$(JLINK) $(JLINK_OPTIONS) $(JLINK_SCRIPTS_DIR)/flash.jlink

.PHONY: clean
clean: 	
	cargo clean
