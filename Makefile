TARGET=target/thumbv7em-none-eabi/debug/examples
# assuming `nightly` and `llvm-tools installed`
OBJCOPY=$(shell rustc --print sysroot)/bin/llvm-objcopy
JLINK=JLinkExe
JLINK_OPTIONS+=-device nrf52 -if swd -speed 1200 -AutoConnect 1
JLINK_SCRIPTS_DIR=jtag/

.PHONY: build
build:
	cargo build --examples

.PHONY: flash
flash: 	
	cargo build --example $(app)
	$(OBJCOPY) -Oihex $(TARGET)/$(app) $(TARGET)/application.hex
	$(JLINK) $(JLINK_OPTIONS) $(JLINK_SCRIPTS_DIR)/flash.jlink

.PHONY: clean
clean: 	
	cargo clean
