//! Hardware (hw) module.
//!
//! This contains code related to interfacing with the raspberry pi 3 hardware.
pub(crate) mod interrupt;
pub(crate) mod timer;
pub(crate) mod uart;

/// The physical address where I/O peripherals are mapped to.
pub(crate) const IO_BASE: usize = 0x3F000000;
