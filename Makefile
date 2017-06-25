.PHONY: all test clean

TARGET_XTENSA=xtensa-lx106-elf
TARGET_ARM=arm-none-eabi

all: test

test:
	# build and simulate xtensa
	cd test && premake5 gmake2 --target=$(TARGET_XTENSA)
	make -C test all
	# build and simulate ARM

clean:
	make -C test clean
