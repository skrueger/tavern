use crate::traps::TrapFrame;

/// Sleep for `ms` milliseconds.
///
/// This system call takes one parameter: the number of milliseconds to sleep.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the approximate true elapsed time from when `sleep` was called to
/// when `sleep` returned.
#[allow(dead_code)] // not used yet.
pub(crate) fn sleep(_ms: u32, _tf: &mut TrapFrame) {
    unimplemented!("syscall: sleep()")
}

#[allow(dead_code)] // not used yet.
pub(crate) fn handle_syscall(_num: u16, _tf: &mut TrapFrame) {
    unimplemented!("handle_syscall()")
}
