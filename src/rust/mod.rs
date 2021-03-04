//! The Rust core prelude.

#[macro_use]
pub mod macros;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod rand;

use core;
use core::panic::PanicInfo;

#[lang = "eh_personality"]
#[no_mangle]
unsafe extern "C" fn rust_eh_personality() {
    crate::syscalls::sys_exit(1);
}

#[lang = "panic_impl"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_: &PanicInfo) -> ! {
    unsafe { crate::posix::stdlib::abort() }
}
