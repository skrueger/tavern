mod irq;
mod syndrome;
mod syscall;
mod trap_frame;

use crate::{process::State, SCHEDULER};

pub use self::trap_frame::TrapFrame;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)] // not used yet.
pub enum Kind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)] // not used yet.
pub enum Source {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}

/// This function is called when an exception occurs. The `info` parameter
/// specifies the source and kind of exception that has occurred. The `esr` is
/// the value of the exception syndrome register. Finally, `tf` is a pointer to
/// the trap frame for the exception.
#[no_mangle]
pub extern "C" fn handle_exception(_info: Info, _esr: u32, tf: &mut TrapFrame) {
    crate::kprintln!("handle_exception enter");
    let controller = crate::hw::interrupt::Controller::new();
    if controller.is_pending(crate::hw::interrupt::Interrupt::Timer1) {
        crate::kprintln!("Timer1 interrupt pending. Setting new tick.");
        let mut timer = crate::hw::timer::Timer::new();
        timer.tick_in(crate::TICK);
        let _scheduled_pid = SCHEDULER.switch(State::Ready, tf);
    }
    crate::kprintln!("handle_exception exit");
}
