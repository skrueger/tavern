//! Tavern Kernel

#![cfg_attr(not(test), no_std)]
// lang_items is needed for eh_personality.
#![cfg_attr(not(test), feature(lang_items))]
// The lang_items feature creates a build warning for internal_features.
#![cfg_attr(not(test), allow(internal_features))]

use crate::atags::Atags;

extern crate alloc;

mod allocator;
mod atags;
mod hw;
mod lang_items;
mod mutex;

#[allow(unused_macros)]
#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            core::writeln!(unsafe { &mut $crate::hw::uart::UART0 }, $($arg)*).unwrap();
        }
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            core::write!(unsafe { &mut $crate::hw::uart::UART0 }, $($arg)*).unwrap();
        }
    };
}

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator::uninitialized();

#[no_mangle]
pub extern "C" fn kmain() {
    kprintln!("kmain enter");
    #[cfg(not(test))]
    ALLOCATOR.initialize();

    for atag in Atags::get() {
        if let Some(cmdline) = atag.cmd() {
            kprintln!("Atags cmdline: {cmdline}");
        }

        if let Some(mem) = atag.mem() {
            kprintln!("Atags mem start: {}, size: {}", mem.start, mem.size);
        }
    }

    kprintln!("kmain exit");
}
