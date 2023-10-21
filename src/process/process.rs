use super::{Stack, State};
use crate::traps::TrapFrame;
use alloc::boxed::Box;

/// Type alias for the type of a process ID.
pub type Id = u64;

/// A structure that represents the complete state of a process.
#[derive(Debug)]
pub struct Process {
    /// The saved trap frame of a process.
    pub trap_frame: Box<TrapFrame>,
    /// The memory allocation used for the process's stack.
    pub stack: Stack,
    /// The scheduling state of the process.
    pub state: State,
}

impl Process {
    /// Creates a new process with a zeroed `TrapFrame` (the default), a zeroed
    /// stack of the default size, and a state of `Ready`.
    ///
    /// If enough memory could not be allocated to start the process, returns
    /// `None`. Otherwise returns `Some` of the new `Process`.
    pub fn new() -> Self {
        Self {
            trap_frame: Box::new(TrapFrame::zeroed()),
            stack: Stack::new(),
            state: State::Ready,
        }
    }

    /// Returns `true` if this process is ready to be scheduled.
    ///
    /// This functions returns `true` only if one of the following holds:
    ///
    ///   * The state is currently `Ready`.
    ///
    ///   * An event being waited for has arrived.
    ///
    ///     If the process is currently waiting, the corresponding event
    ///     function is polled to determine if the event being waiting for has
    ///     occured. If it has, the state is switched to `Ready` and this
    ///     function returns `true`.
    ///
    /// Returns `false` in all other cases.
    pub fn is_ready(&mut self) -> bool {
        let current_state = core::mem::replace(&mut self.state, State::Ready);
        let (is_ready, next_state) = match current_state {
            State::Ready => (true, State::Ready),
            State::Running => (false, State::Running),
            State::Waiting(mut poll_fn) => {
                let is_ready = poll_fn(self);
                (is_ready, State::Waiting(poll_fn))
            }
        };

        let _ = core::mem::replace(&mut self.state, next_state);
        is_ready
    }
}
