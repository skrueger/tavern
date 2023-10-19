
.PHONY: all run clean

all: build/tavern.elf build/tavern.bin build/tavern.hex

build:
	mkdir -p $@

build/kernel.o: src/kernel.S | build
	aarch64-none-elf-gcc -c $< -o $@

# libtavern.a is phony so cargo build is always ran.
# cargo owns building libtavern.a not this Makefile.
.PHONY: build/aarch64-unknown-none/debug/libtavern.a
build/aarch64-unknown-none/debug/libtavern.a:
	cargo build --target aarch64-unknown-none

build/tavern.elf: build/kernel.o build/aarch64-unknown-none/debug/libtavern.a src/layout.ld
	aarch64-none-elf-ld -o $@ build/kernel.o build/aarch64-unknown-none/debug/libtavern.a -T src/layout.ld

build/tavern.bin: build/tavern.elf | build
	aarch64-none-elf-objcopy $< -O binary $@

build/tavern.hex: build/tavern.elf | build
	aarch64-none-elf-objcopy $< -O ihex $@

clean:
	rm -f build/kernel.o
	rm -f build/tavern.elf
	rm -f build/tavern.bin
	rm -f build/tavern.hex
	rm -f build/aarch64-unknown-none/debug/libtavern.a

run:
	qemu-system-aarch64 -machine raspi3b -cpu cortex-a53 -serial stdio -serial /dev/null -display none -kernel build/tavern.bin -append "cmdline args" -d int
