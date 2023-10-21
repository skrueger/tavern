use crate::hw::interrupt::Interrupt;

use crate::traps::TrapFrame;

#[allow(dead_code)] // not used yet.
pub(crate) fn handle_irq(_interrupt: Interrupt, _tf: &mut TrapFrame) {
    unimplemented!("handle_irq()")
}
