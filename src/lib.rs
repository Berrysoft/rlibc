#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(core_intrinsics, lang_items, llvm_asm, option_result_unwrap_unchecked)]
#![no_std]

extern crate core;

#[cfg(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "android", target_arch = "x86_64")
))]
pub use rust::x86_64::linux::start::__libc_start_main;

#[cfg(any(
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "ios", target_arch = "x86_64")
))]
pub use rust::x86_64::macos::start::_libc_start_main;

#[macro_use]
mod rust;

mod consts;
mod types;

pub mod libc;
pub mod math;
pub mod posix;
pub mod syscalls;
