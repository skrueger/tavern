//! Tavern Kernel

#![cfg_attr(not(test), no_std)]
// lang_items is needed for eh_personality.
#![cfg_attr(not(test), feature(lang_items))]
// The lang_items feature creates a build warning for internal_features.
#![cfg_attr(not(test), allow(internal_features))]
// ptr_internals is needed for core::ptr::Unique.
#![cfg_attr(not(test), feature(ptr_internals))]

extern crate alloc;

mod allocator;
mod atags;
mod hw;
mod lang_items;
mod mutex;
#[cfg(not(test))]
mod process;
#[cfg(not(test))]
mod traps;
mod vm;
mod volatile;

use atags::Atags;
use hw::interrupt::{Controller as InterruptController, Interrupt};
#[cfg(not(test))]
use process::GlobalScheduler;

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

/// The `tick` time.
const TICK: u32 = 2 * 1_000 * 1_000;

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator::uninitialized();

#[cfg(not(test))]
static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();

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

    let mut interrupt_controller = InterruptController::new();
    interrupt_controller.enable(Interrupt::Timer1);

    let mut timer = crate::hw::timer::Timer::new();

    timer.tick_in(TICK);

    #[cfg(not(test))]
    SCHEDULER.start();

    kprintln!("kmain exit");
}
