TARGET=target/thumbv7em-none-eabi/debug/examples
OBJCOPY=arm-none-eabi-objcopy
JLINK=JLinkExe
JLINK_OPTIONS+=-device nrf52 -if swd -speed 1200 -AutoConnect 1
JLINK_SCRIPTS_DIR=jtag/

.PHONY: build
build:
	xargo build --examples

.PHONY: flash
flash: 	
	xargo build --example $(app)
	$(OBJCOPY) -Oihex $(TARGET)/$(app) $(TARGET)/application.hex
	$(JLINK) $(JLINK_OPTIONS) $(JLINK_SCRIPTS_DIR)/flash.jlink

.PHONY: clean
clean: 	
	xargo clean
