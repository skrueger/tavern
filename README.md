# Tavern

## About
Tavern is a [kernel](https://en.wikipedia.org/wiki/Kernel_\(operating_system\)) for the [Raspberry Pi Model 3B+](https://www.raspberrypi.com/products/raspberry-pi-3-model-b-plus/).

It is a simple unix-like monolith
that runs AArch64 user-mode processes with virtual memory addressing.
It is my (Simon Krueger) hobby project and its primary goal is education in kernel development.
I started this project because I wanted to learn the low-level details of how a kernel works by implementing one from scratch.
I started this project from [Stanford's 2018 CS140e course](https://cs140e.sergio.bz/).
The course provided the initial skeleton and I've been fleshing out the implementation.

Tavern is in its early stages of development and is incomplete.
It runs multiple user-mode processes in EL0 with physical memory addressing but lacks virtual memory addressing.
Virtual memory addressing is the target for the next development effort.

You will benefit from this project if you're interested
in learning about kernel-implementations, rust, ARMv8/AArch64, or Raspberry Pi 3 hardware.

## Development

Tavern is written in rust and developed on Ubuntu 22.04.3 LTS (Jammy Jellyfish) Desktop amd64.
Other development environments (e.g., other linux distributions, macOS, or Windows) are not supported by me,
but they would likely work if you're someone familiar with rust and toolchains in those environments.

Tavern requires:
- [Rust](https://rustup.rs/)
    - Tavern uses the release channel `nightly` because it is needed for `lang_items` and `ptr_internals`. `nightly` is installed with `rustup default nightly`.
    - Tavern runs bare metal aarch64 with a floating point unit and thus targets `aarch64-unknown-none`. That target is installed with `rustup target add aarch64-unknown-none`.
    - As of October 2023, Tavern built successfully with Rust `v1.74.0-nightly`.
- [Make](https://packages.ubuntu.com/jammy/make)
- [aarch64 toolchain](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain)
    - The aarch64 toolchain is needed for a cross platform aarch64 assembler and linker. Download the aarch64 toolchain and add its `bin` directory to your `PATH`.
- [QEMU aarch64](https://wiki.ubuntu.com/ARM64/QEMU)
    - Tavern also runs on QEMU. QEMU conveniently emulates the Raspberry Pi 3B with `-machine raspi3b`. Running Tavern on QEMU is a quick and easy way to develop Tavern.

Build Tavern with `make all`. Run Tavern on QEMU with `make run`.

## Boot Process
The Raspberry Pi's firmware loads Tavern at memory address `0x80000` and its ARM Cortex-A53 starts executing instructions from there.
Tavern's entry point is in `kernel.S` and labeled with `__start`.
From there Tavern switches over to ARM Exception Level EL1 (Kernel-mode), sets ups the stack pointer `sp`, sets up the exception handler, and then jumps into the function
`kmain` which is in `lib.rs`.
In `kmain`, Tavern sets up the global heap memory allocator.
The allocator's memory range is determined by the Atags values that were loaded in by the firmware.
After the memory allocator is initialized, it initializes a hardware timer and a cooresponding component interrupt controller.
The timer interrupt drives round robbin process scheduling.
Finally, Tavern creates 2 user-mode processes that continuously output to the UART0 serial port.


## Appendix: Raspberry Pi Model 3B+

The Raspberry Pi Model 3B+ runs a [Broadcom BCM2837 SoC](https://www.raspberrypi.com/documentation/computers/processors.html#bcm2837).
BCM2837 is largely the same as the BCM2835 used in the Raspberry Pi 1.
BCM2837 contains many [peripherals](https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf) and
an ARM Cortex-A53.
Peripherals are accessed by reading and writing to specific memory addresses.
The ARM Cortex-A53 is a mid-range, low-power processor that implements the Armv8-A architecture.
The `-A` means it is an "Application" processor that contains an MMU and is designed for general purpose computing and running an operating system like linux.

## Appendix: Misc. Resources
Here is a heap of miscellaneous resources that I've referenced during development.
There are pointers to information about hardware, ARM, qemu and rust.

- https://www.qemu.org/docs/master/system/target-arm.html
- https://www.qemu.org/docs/master/system/arm/raspi.html
- https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain
- Arm Architecture Reference Manual for A-profile architecture
- ARM Cortex-A Series Programmer's Guide for ARMv8-A
- Arm Cortex-A53 MPCore Processor Technical Reference Manual
- Broadcom VideoCore IV https://docs.broadcom.com/doc/12358545
- https://learn.sparkfun.com/tutorials/serial-communication
- https://learn.sparkfun.com/tutorials/serial-peripheral-interface-spi
- https://learn.sparkfun.com/tutorials/i2c
- https://doc.rust-lang.org/core/index.html
- https://doc.rust-lang.org/alloc/index.html
