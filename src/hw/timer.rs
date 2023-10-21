use crate::volatile::prelude::*;
use crate::volatile::{ReadVolatile, Volatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = super::IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    /// System Timer Control / Status Register
    /// This register is used to record and clear timer channel comparator matches.
    /// Write a 1 to clear the match detect status bit and the coorespoding interrupt request line.
    /// Bit 0 is for Timer 0. Read 1 means there was a match detected.
    CS: Volatile<u32>,
    /// System Timer Counter Lower 32 bits.
    CLO: ReadVolatile<u32>,
    /// System Timer Counter Higher 32 bits.
    CHI: ReadVolatile<u32>,
    /// Timer compare registers.
    /// C0, C1, C2, C3.
    /// Write 32 bits, when it matches the lower 32 bits of the free running timer
    /// the bit is flipped in the system timer control/status register.
    COMPARE: [Volatile<u32>; 4],
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers,
}

#[cfg_attr(test, allow(dead_code))]
impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        let mut counter: u64 = self.registers.CHI.read() as u64;
        counter <<= 32;
        counter += self.registers.CLO.read() as u64;
        counter
    }

    /// Sets up a match in timer 1 to occur `us` microseconds from now. If
    /// interrupts for timer 1 are enabled and IRQs are unmasked, then a timer
    /// interrupt will be issued in `us` microseconds.
    pub fn tick_in(&mut self, us: u32) {
        self.registers.CS.or_mask(0b10);
        let current_lower = self.registers.CLO.read();
        self.registers.COMPARE[1].write(current_lower + us);
    }
}

/// Returns the current time in microseconds.
#[cfg_attr(test, allow(dead_code))]
pub fn current_time() -> u64 {
    Timer::new().read()
}

/// Spins until `us` microseconds have passed.
#[cfg_attr(test, allow(dead_code))]
pub fn spin_sleep_us(us: u64) {
    let start = current_time();
    loop {
        let now = current_time();
        let duration = now - start;
        if duration > us {
            break;
        } else {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
}

/// Spins until `ms` milliseconds have passed.
#[cfg_attr(test, allow(dead_code))]
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms * 1000);
}
