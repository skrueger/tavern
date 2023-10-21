use super::IO_BASE;
use crate::volatile::prelude::*;
use crate::volatile::{ReadVolatile, Volatile};

const INT_BASE: usize = IO_BASE + 0xB000 + 0x200;

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)] // not all variants are used yet.
pub enum Interrupt {
    Timer1 = 1,
    Timer3 = 3,
    Usb = 9,
    Gpio0 = 49,
    Gpio1 = 50,
    Gpio2 = 51,
    Gpio3 = 52,
    Uart = 57,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    /// Shows which interrupt are pending.
    /// Bit 0 is ARM Timer IRQ Pending.
    IRQ_BASIC_PENDING: ReadVolatile<u32>,
    /// GPU Pending 1 Register
    /// Holds all the interrupts 0..31 from the GPU.
    IRQ_PENDING_1: ReadVolatile<u32>,
    /// GPU Pending 2 Register.
    /// Holds all the interrupts 32..63 from the GPU.
    IRQ_PENDING_2: ReadVolatile<u32>,
    /// The FIG Register controls which
    /// interrupt source can generate a FIQ to the ARM.
    /// Only a single inerrupt can be selected.
    FIQ_CONTROL: Volatile<u32>,
    /// Write a 1 to a bit will set the correspodning IRQ enable bit.
    /// IRQ 0..31
    ENABLE_IRQS_1: Volatile<u32>,
    /// Write a 1 to a bit will set the correspodning IRQ enable bit.
    /// IRQ 32..63
    ENABLE_IRQS_2: Volatile<u32>,
    /// Basic interrupt enable register.
    /// Write a 1 to a bit will set the corresponding IRQ enable bit.
    /// Bit 0 Set to enable ARM Timer IRQ.
    ENABLE_BASIC_IRQS: Volatile<u32>,
    /// Write a 1 to a bit will clear the correspoding IRQ enable bit.
    /// IRQs 0..31
    DISABLE_IRQS_1: Volatile<u32>,
    /// Write a 1 to a bit will clear the correspoding IRQ enable bit.
    /// IRQs 32..63
    DISABLE_IRQS_2: Volatile<u32>,
    /// Write a 1 to a bit will clear the correspoding IRQ enable bit.
    /// Bit 0 - Set to disable ARM Timer IRQ.
    DISABLE_BASIC_IRQS: Volatile<u32>,
}

/// An interrupt controller. Used to enable and disable interrupts as well as to
/// check if an interrupt is pending.
pub struct Controller {
    registers: &'static mut Registers,
}

impl Controller {
    /// Returns a new handle to the interrupt controller.
    pub fn new() -> Controller {
        Controller {
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    /// Enables the interrupt `int`.
    pub fn enable(&mut self, int: Interrupt) {
        self.registers.ENABLE_IRQS_1.or_mask(1 << (int as u8))
    }

    /// Disables the interrupt `int`.
    #[allow(dead_code)] // not currently used.
    pub fn disable(&mut self, int: Interrupt) {
        self.registers.DISABLE_IRQS_1.and_mask(1 << (int as u32))
    }

    /// Returns `true` if `int` is pending. Otherwise, returns `false`.
    #[allow(dead_code)] // not currently used.
    pub fn is_pending(&self, int: Interrupt) -> bool {
        self.registers.IRQ_PENDING_1.has_mask(1 << (int as u8))
    }
}
