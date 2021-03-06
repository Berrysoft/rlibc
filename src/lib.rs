#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(
    allocator_api,
    asm,
    compiler_builtins,
    const_btree_new,
    core_intrinsics,
    c_variadic,
    default_alloc_error_handler,
    lang_items,
    llvm_asm,
    naked_functions,
    option_result_unwrap_unchecked,
    panic_info_message,
    rustc_private,
    thread_local
)]
#![no_std]

extern crate alloc;
extern crate compiler_builtins;
extern crate core;

#[macro_use]
mod rust;

mod consts;
mod types;

#[macro_use]
pub mod libc;
pub mod math;
pub mod posix;
pub mod syscalls;
