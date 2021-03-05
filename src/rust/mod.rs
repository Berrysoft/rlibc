//! The Rust core prelude.

#[macro_use]
pub mod macros;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod alloc;
pub mod rand;

use core::panic::PanicInfo;
use core::{self, fmt::Write};

#[lang = "eh_personality"]
unsafe extern "C" fn rust_eh_personality() {
    crate::syscalls::sys_exit(1);
}

#[lang = "panic_impl"]
unsafe extern "C" fn rust_begin_panic(info: &PanicInfo) -> ! {
    if let Some(msg) = info.message() {
        crate::libc::stdio::__stderr
            .write_fmt(*msg)
            .unwrap_or_default();
    }
    crate::posix::stdlib::abort()
}
