use crate::posix::pm::_exit;
use crate::posix::unistd::getpid;
use crate::syscalls::sys_kill;
use crate::types::{int_t, pid_t, void_t};

pub use self::Signals::*;

pub type sighandler_t = Option<extern "C" fn(int_t)>;

type __sa_handler_t = Option<extern "C" fn(int_t)>;
type __sa_sigaction = Option<extern "C" fn(int_t, *mut void_t, *mut void_t)>;
type __sigaction_u_t = *mut void_t;
type sigset_t = u32;
#[repr(C)]
struct sigaction_s {
    __sigaction_u: __sigaction_u_t,
    sa_mask: sigset_t,
    sa_flags: int_t,
}

pub mod Signals {
    use crate::types::int_t;

    /// Signal Interrupt: Interactive attention signal.
    pub const SIGINT: int_t = 2;
    /// Signal Illegal Instruction: Invalid function image.
    pub const SIGILL: int_t = 4;
    /// Signal Abort: Abnormal termination.
    pub const SIGABRT: int_t = 6;
    /// Signal Floating-Point Exception: Erroneous arithmetic operation.
    pub const SIGFPE: int_t = 8;
    /// Signal Segmentation Violation: Invalid access to memory storage.
    pub const SIGSEGV: int_t = 11;
    /// Signal Terminate: Termination request sent to program.
    pub const SIGTERM: int_t = 15;
}

/// Generates a signal
/// Sends signal `sig` to the current executing program.
/// The signal is handled as specified by function `signal`.
#[no_mangle]
pub unsafe extern "C" fn raise(sig: int_t) -> int_t {
    kill(getpid(), sig)
}

/// Set function to handle signal.
/// Specifies a way to handle the signals with the signal number specified by sig.
#[no_mangle]
pub unsafe extern "C" fn signal(_sig: int_t, _func: sighandler_t) -> sighandler_t {
    _exit(1); // TODO implement signal attachment
}

/// Send a signal to a process or a group of processes.
#[no_mangle]
pub unsafe extern "C" fn kill(pid: pid_t, sig: int_t) -> int_t {
    forward!(sys_kill, pid, sig) as _
}
