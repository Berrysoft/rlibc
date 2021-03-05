#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(
    core_intrinsics,
    c_variadic,
    lang_items,
    llvm_asm,
    option_result_unwrap_unchecked
)]
#![no_std]

extern crate core;

pub use rust::x86_64::linux::start::__libc_start_main;

#[macro_use]
mod rust;

mod consts;
mod types;

#[macro_use]
pub mod libc;
pub mod math;
pub mod posix;
pub mod syscalls;
