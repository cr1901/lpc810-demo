TARGET=target/thumbv7em-none-eabi/release/lpc810-demo
PORT=/dev/ttyUSB2

lpc810-demo:
	xargo build --release --target=thumbv7em-none-eabi
	arm-none-eabi-objdump -Cd $(TARGET) > $(TARGET).lst
	arm-none-eabi-readelf -s $(TARGET) > $(TARGET).sym
	arm-none-eabi-size $(TARGET)
	arm-none-eabi-objcopy -O binary $(TARGET) $(TARGET).bin
	
clean:
	cargo clean
	
prog:
	lpc21isp -bin $(TARGET).bin $(PORT) 115200 0
