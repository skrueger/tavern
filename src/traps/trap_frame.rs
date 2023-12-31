#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct TrapFrame {
    /// EL1 Link Register
    pub(crate) elr: u64,
    /// EL1 Saved Program Status Register
    pub(crate) spsr: u64,
    /// EL0 Stack Pointer Register
    pub(crate) sp: u64,
    /// EL0 Thread ID Register
    pub(crate) tpidr: u64,
    /// 128-bit floating point registers
    pub(crate) q0: u128,
    pub(crate) q1: u128,
    pub(crate) q2: u128,
    pub(crate) q3: u128,
    pub(crate) q4: u128,
    pub(crate) q5: u128,
    pub(crate) q6: u128,
    pub(crate) q7: u128,
    pub(crate) q8: u128,
    pub(crate) q9: u128,
    pub(crate) q10: u128,
    pub(crate) q11: u128,
    pub(crate) q12: u128,
    pub(crate) q13: u128,
    pub(crate) q14: u128,
    pub(crate) q15: u128,
    pub(crate) q16: u128,
    pub(crate) q17: u128,
    pub(crate) q18: u128,
    pub(crate) q19: u128,
    pub(crate) q20: u128,
    pub(crate) q21: u128,
    pub(crate) q22: u128,
    pub(crate) q23: u128,
    pub(crate) q24: u128,
    pub(crate) q25: u128,
    pub(crate) q26: u128,
    pub(crate) q27: u128,
    pub(crate) q28: u128,
    pub(crate) q29: u128,
    pub(crate) q30: u128,
    pub(crate) q31: u128,
    /// 64-bit registers
    pub(crate) x1: u64,
    pub(crate) x2: u64,
    pub(crate) x3: u64,
    pub(crate) x4: u64,
    pub(crate) x5: u64,
    pub(crate) x6: u64,
    pub(crate) x7: u64,
    pub(crate) x8: u64,
    pub(crate) x9: u64,
    pub(crate) x10: u64,
    pub(crate) x11: u64,
    pub(crate) x12: u64,
    pub(crate) x13: u64,
    pub(crate) x14: u64,
    pub(crate) x15: u64,
    pub(crate) x16: u64,
    pub(crate) x17: u64,
    pub(crate) x18: u64,
    pub(crate) x19: u64,
    pub(crate) x20: u64,
    pub(crate) x21: u64,
    pub(crate) x22: u64,
    pub(crate) x23: u64,
    pub(crate) x24: u64,
    pub(crate) x25: u64,
    pub(crate) x26: u64,
    pub(crate) x27: u64,
    pub(crate) x28: u64,
    pub(crate) x29: u64,
    pub(crate) reserved: u64,
    pub(crate) x30: u64,
    pub(crate) x0: u64,
}

impl TrapFrame {
    pub(crate) fn zeroed() -> Self {
        Self {
            elr: 0,
            spsr: 0,
            sp: 0,
            tpidr: 0,
            q0: 0,
            q1: 0,
            q2: 0,
            q3: 0,
            q4: 0,
            q5: 0,
            q6: 0,
            q7: 0,
            q8: 0,
            q9: 0,
            q10: 0,
            q11: 0,
            q12: 0,
            q13: 0,
            q14: 0,
            q15: 0,
            q16: 0,
            q17: 0,
            q18: 0,
            q19: 0,
            q20: 0,
            q21: 0,
            q22: 0,
            q23: 0,
            q24: 0,
            q25: 0,
            q26: 0,
            q27: 0,
            q28: 0,
            q29: 0,
            q30: 0,
            q31: 0,
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
            x9: 0,
            x10: 0,
            x11: 0,
            x12: 0,
            x13: 0,
            x14: 0,
            x15: 0,
            x16: 0,
            x17: 0,
            x18: 0,
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            reserved: 0,
            x30: 0,
            x0: 0,
        }
    }
}
