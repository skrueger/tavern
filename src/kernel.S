// kernel.S
// This is the kernel's entry point.
// It changes the machine into EL1, setups the stack and jumps to kmain.

.text
.global __start
__start:
    // Run with a single core.
    // Read the core number from Multiprocessor Affinity Register (MPIDR).
    // The last 3 bits indicate the core number of Cortex-A53.
    mrs     x7, mpidr_el1
    and     x7, x7, #0b11
    cbz     x7, setup
    b       __hang

setup:
    // read the current exception level into x0 (ref: C5.2.1)
    mrs     x0, CurrentEL       // 32-bit reg. EL Bits are 3:2.
    and     x0, x0, #0b1100
    lsr     x0, x0, #2

switch_to_el2:
    // switch to EL2 if we're in EL3. otherwise switch to EL1
    cmp     x0, #3
    bne     switch_to_el1

    // set-up EL3's Secure Configuration Register.
    // SCR_EL3 (bits 0, 4, 5, 7, 8, 10) (A53: 4.3.42)
    // 0 - NS - Non-Secure bit EL0 and EL1 are in Non-secure state
    // 4,5 - RES1 -
    // 7 - SMD - SMC (Secure Monitor Call) instruction disable. SMC causes an exception to EL3.
    // 8 - HCE - Hyp Call enabled. HVC causes an exception to EL2.
    // 10 - RW - Register width. 1 means the next lower level is AArch64.
    mov     x2, #0x5b1
    msr     SCR_EL3, x2

    // set-up Saved Program Status Register (SPSR) and Exception Link Register.
    // (bits 0, 3, 6, 7, 8, 9) (ref: C5.2.20)
    // 0:3 - M - Exception level and SP that an exception was taken from
    // 0 - exception SP is determined by EL (bits 3:2)
    // 3:2 - holds the Exception Level (EL).
    // 6 - F - FIQ mask - 1 means the exception is masked.
    // 7 - I - IRQ mask - 1 means the exception is masked.
    // 8 - A - SError interrupt mask - 1 means the exception is masked.
    // 9 - D - Debug mask - 1 means the Watchpoint, breakpoint and software step at target exception is masked.
    mov     x2, #0x3c9
    msr     SPSR_EL3, x2
    adr     x2, switch_to_el1
    msr     ELR_EL3, x2
    eret

switch_to_el1:
    // switch to EL1 if we're not already in EL1. otherwise continue with start
    cmp     x0, #1
    beq     set_stack

    // set the stack-pointer for EL1
    msr     SP_EL1, x1

    // enable CNTP for EL1/EL0 (ref: D7.5.2, D7.5.13)
    // NOTE: This doesn't actually enable the counter stream.
    // CNTHCTL_EL2 is the Counter-timer Hypervisor Control register.
    // 0 - EL1PCTEN - Traps Non-secure EL0 and EL1 accesses to the physical **counter** register to EL2. No interrupts traps.
    // 1 - EL1PCEN - Traps Non-secure EL0 and EL1 accesses to the physical **timer** registers to EL2. - No interrupts traps.
    mrs     x0, CNTHCTL_EL2
    orr     x0, x0, #0b11
    msr     CNTHCTL_EL2, x0
    // CNTVOFF_EL2 is the Counter-timer Virtual Offset register.
    // Holds the 64-bit virtual offset. This is the offset between the physical count value visible in CNTPCT_EL0 and the virtual count value visible in CNTVCT_EL0
    msr     CNTVOFF_EL2, xzr

    // enable AArch64 in EL1 (A53: 4.3.36)
    mov     x0, #(1 << 31)      // Enable AArch64 for EL1
    orr     x0, x0, #(1 << 1)   // RES1 on A-53
    // HCR_EL2 is Hypervisor Configuration Register
    // 31 - RW - Register width. 1 means that EL1 is AArch64.
    // 1 - SWIO - Set/Way Invalidation Override.
    msr     HCR_EL2, x0
    mrs     x0, HCR_EL2

    // enable floating point and SVE (SIMD) (A53: 4.3.38, 4.3.34)
    // CPTR_EL2 is Architectural Feature Trap Register in EL2
    msr     CPTR_EL2, xzr     // don't trap accessing SVE registers
    // CPACR_EL is Architectural Feature Access Control Register
    mrs     x0, CPACR_EL1
    orr     x0, x0, #(0b11 << 20)
    msr     CPACR_EL1, x0

    // Set SCTLR to known state (RES1: 11, 20, 22, 23, 28, 29) (A53: 4.3.30)
    // SCTLR is System Control Register for EL1
    mov     x2, #0x0800
    movk    x2, #0x30d0, lsl #16
    msr     SCTLR_EL1, x2

    // set up exception handlers (guide: 10.4).
    adr     x3, _vectors
    // Vector Base Address Register for EL1
    // Holds the exception base address for any exception that is taken to EL1
    msr     VBAR_EL1, x3

    // change execution level to EL1 (ref: C5.2.19)
    mov     x2, #0x3c5
    msr     SPSR_EL2, x2
    adr     x2, set_stack
    msr     ELR_EL2, x2
    eret

set_stack:
    adrp    x2, __cpu0_stack_end
    add     x2, x2, #:lo12:__cpu0_stack_end
    mov     sp, x2

__clear_bss:
    ldr     w0, _bss_segment + 0
    ldr     w1, _bss_segment + 4
__clear:
    cbz     x1, __go_main
    str     xzr, [x0], #8
    sub     x1, x1, #1
    cbnz    x1, __clear

__go_main:
    stp     x29, lr, [SP, #-0x10]!
    mov     x29, SP
    bl      kmain

__hang:
    // hang will wait for an event forever.
    wfe
    b       __hang


#define HANDLER(source, kind) \
    .align 7; \
    stp     lr, x0, [SP, #-16]!; \
    mov     x0, ##source; \
    movk    x0, ##kind, LSL #16; \
    bl      context_save; \
    ldp     lr, x0, [SP], #16; \
    eret

.align 11
_vectors:
    HANDLER(0, 0)
    HANDLER(0, 1)
    HANDLER(0, 2)
    HANDLER(0, 3)
    HANDLER(1, 0)
    HANDLER(1, 1)
    HANDLER(1, 2)
    HANDLER(1, 3)
    HANDLER(2, 0)
    HANDLER(2, 1)
    HANDLER(2, 2)
    HANDLER(2, 3)
    HANDLER(3, 0)
    HANDLER(3, 1)
    HANDLER(3, 2)
    HANDLER(3, 3)

context_save:
context_restore:
    ret

_bss_segment:
    .word	__bss_start
    .word	__bss_dwords
