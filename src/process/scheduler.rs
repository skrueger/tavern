use alloc::collections::VecDeque;

use super::{Id, Process, State};
use crate::mutex::Mutex;
use crate::traps::TrapFrame;

/// Process scheduler for the entire machine.
#[derive(Debug)]
pub struct GlobalScheduler(Mutex<Option<Scheduler>>);

fn thread_id() -> u64 {
    let thread_id: u64;
    unsafe {
        core::arch::asm!(
        "mrs {0}, TPIDR_EL0",
        out(reg) thread_id
        );
    }
    thread_id
}

#[no_mangle]
extern "C" fn init() {
    let thread_id = self::thread_id();
    crate::kprintln!("init enter tid {thread_id}");

    loop {
        let thread_id = self::thread_id(); // keep getting thread_id in case it changed.
        crate::kprintln!("init loop tid {thread_id}. Sleeping for 1 sec.");
        crate::hw::timer::spin_sleep_ms(1_000);
    }
}

impl GlobalScheduler {
    /// Returns an uninitialized wrapper around a local scheduler.
    pub const fn uninitialized() -> GlobalScheduler {
        GlobalScheduler(Mutex::new(None))
    }

    /// Adds a process to the scheduler's queue and returns that process's ID.
    /// For more details, see the documentation on `Scheduler::add()`.
    #[allow(dead_code)] // not used yet.
    pub fn add(&self, process: Process) -> Option<Id> {
        self.0
            .lock()
            .as_mut()
            .expect("scheduler uninitialized")
            .add(process)
    }

    /// Performs a context switch using `tf` by setting the state of the current
    /// process to `new_state`, saving `tf` into the current process, and
    /// restoring the next process's trap frame into `tf`. For more details, see
    /// the documentation on `Scheduler::switch()`.
    #[must_use]
    pub fn switch(&self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        self.0
            .lock()
            .as_mut()
            .expect("scheduler uninitialized")
            .switch(new_state, tf)
    }

    /// Initializes the scheduler and starts executing processes in user space
    /// using timer interrupt based preemptive scheduling. This method should
    /// not return under normal conditions.
    pub fn start(&self) {
        use core::ops::DerefMut;
        let mut guard = self.0.lock();
        let _old = core::mem::replace(guard.deref_mut(), Some(Scheduler::new()));
        let mut process1 = Process::new();
        unsafe {
            // set the process trap frame's stack pointer
            let process1_stack_pointer = process1.stack.top().as_mut_ptr() as *mut TrapFrame;
            process1.trap_frame.sp = process1_stack_pointer as u64;

            // set the link return to the init1 function
            process1.trap_frame.elr = init as usize as u64;

            // push the trap frame onto the processes stack
            let process1_tf_dst = process1_stack_pointer.sub(1);
            process1.trap_frame.tpidr = 1;
            let process1_tf_src = (&mut *process1.trap_frame) as *mut TrapFrame;
            core::ptr::copy(process1_tf_src, process1_tf_dst, 1);

            process1.state = State::Running;
            guard.as_mut().unwrap().add(process1);
            guard.as_mut().unwrap().current = Some(1);

            let mut process2 = Process::new();
            process2.trap_frame.sp = process2.stack.top().as_ptr() as u64;
            process2.trap_frame.elr = init as usize as u64;
            guard.as_mut().unwrap().add(process2);

            drop(guard);

            // set the current executing SP to the new process's stack
            // which contains the trap frame
            core::arch::asm!(
            "mov x0, {0}",
            "mov sp, x0",
            "bl context_restore",
            "ldp     lr, x0, [SP], #0x10",
            // reset SP back to the beginning
            "adrp	x2, __cpu0_stack_end",
            "add	x2, x2, #:lo12:__cpu0_stack_end",
            "mov	sp, x2",
            "eret",
            in(reg) process1_tf_dst
            );
        }
    }
}

#[derive(Debug)]
struct Scheduler {
    processes: VecDeque<Process>,
    current: Option<Id>,
    last_id: Option<Id>,
}

impl Scheduler {
    /// Returns a new `Scheduler` with an empty queue.
    fn new() -> Scheduler {
        Self {
            processes: alloc::collections::VecDeque::new(),
            current: None,
            last_id: None,
        }
    }

    /// Adds a process to the scheduler's queue and returns that process's ID if
    /// a new process can be scheduled. The process ID is newly allocated for
    /// the process and saved in its `trap_frame`. If no further processes can
    /// be scheduled, returns `None`.
    ///
    /// If this is the first process added, it is marked as the current process.
    /// It is the caller's responsibility to ensure that the first time `switch`
    /// is called, that process is executing on the CPU.
    fn add(&mut self, mut process: Process) -> Option<Id> {
        let next_id = self.last_id.unwrap_or(0) + 1;
        process.trap_frame.tpidr = next_id;
        self.processes.push_back(process);
        self.last_id = Some(next_id);
        self.last_id
    }

    /// Sets the current process's state to `new_state`, finds the next process
    /// to switch to, and performs the context switch on `tf` by saving `tf`
    /// into the current process and restoring the next process's trap frame
    /// into `tf`. If there is no current process, returns `None`. Otherwise,
    /// returns `Some` of the process ID that was context switched into `tf`.
    ///
    /// This method blocks until there is a process to switch to, conserving
    /// energy as much as possible in the interim.
    fn switch(&mut self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        if self.processes.is_empty() {
            return None;
        }

        let mut current = self.processes.pop_front().unwrap();
        current.state = new_state;
        *current.trap_frame = *tf;

        self.processes.push_back(current);

        loop {
            for _ in 0..self.processes.len() {
                let next = self.processes.front_mut().unwrap();
                if next.is_ready() {
                    next.state = State::Running;
                    *tf = *next.trap_frame;
                    let next_pid = Some(next.trap_frame.tpidr);
                    self.current = next_pid;
                    return next_pid;
                } else {
                    let next = self.processes.pop_front().unwrap();
                    self.processes.push_back(next);
                }
            }
            unsafe {
                core::arch::asm!("wfi");
            }
        }
    }
}
