//! The Rust core prelude.

#[macro_use]
pub mod macros;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod alloc;
pub mod rand;

use crate::libc::stdio::*;
use crate::posix::stdlib::abort;
use core::alloc::Layout;
use core::panic::PanicInfo;
use core2::io::Write;

#[lang = "eh_personality"]
unsafe extern "C" fn rust_eh_personality() {
    crate::syscalls::sys_exit(1);
}

#[lang = "panic_impl"]
unsafe extern "C" fn rust_begin_panic(info: &PanicInfo) -> ! {
    if let Some(msg) = info.message() {
        __stderr.write_fmt(*msg).unwrap_or_default();
        write!(&mut __stderr, "\n").unwrap_or_default();
    }
    abort()
}

#[lang = "oom"]
unsafe fn oom_impl(layout: Layout) -> ! {
    panic!("Out of memory: {:?}", layout)
}
