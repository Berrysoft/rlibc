//! Process lifetime management

use crate::libc::stdio::*;
use crate::posix::signal::{raise, SIGABRT};
use crate::syscalls::*;
use crate::types::int_t;
use core2::io::Write;

static mut ATEXIT_FNS: [Option<unsafe extern "C" fn()>; 32] = [None; 32];

/// Terminates the process normally, performing the regular cleanup.
/// All C streams are closed, and all files created with tmpfile are removed.
/// Status can be zero or EXIT_SUCCESS, or EXIT_FAILURE.
#[no_mangle]
pub unsafe extern "C" fn exit(x: int_t) -> ! {
    for func in ATEXIT_FNS.iter().rev() {
        if let &Some(func) = func {
            func();
        }
    }
    __stdin.flush().unwrap();
    __stdout.flush().unwrap();
    __stderr.flush().unwrap();
    _exit(x);
}

/// _Exit is a synonym for _exit
#[no_mangle]
pub extern "C" fn _Exit(x: int_t) -> ! {
    _exit(x);
}

#[no_mangle]
pub extern "C" fn _exit(x: int_t) -> ! {
    unsafe {
        sys_exit(x);
    }
    unreachable!()
}

#[no_mangle]
pub unsafe extern "C" fn abort() -> ! {
    raise(SIGABRT);
    core::intrinsics::abort()
}

#[no_mangle]
/// Note: this doesn't check for a null argument, sparing a branch.
pub unsafe extern "C" fn atexit(func: Option<unsafe extern "C" fn()>) -> int_t {
    for i in &mut ATEXIT_FNS {
        if i.is_none() {
            *i = func;
            return 0;
        }
    }
    return 1;
}
