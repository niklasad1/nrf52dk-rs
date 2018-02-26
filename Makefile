TARGET=target/thumbv7em-none-eabi/debug/examples
JLINK=JLinkExe
JLINK_OPTIONS+=-device nrf52 -if swd -speed 1200 -AutoConnect 1
JLINK_SCRIPTS_DIR=jtag/

.PHONY: build
build:
	xargo build --example hello

.PHONY: flash
flash: 	
	xargo build --example hello
	arm-none-eabi-objcopy -Oihex $(TARGET)/hello $(TARGET)/hello.hex
	$(JLINK) $(JLINK_OPTIONS) $(JLINK_SCRIPTS_DIR)/flash.jlink

.PHONY: clean
clean: 	
	xargo clean
