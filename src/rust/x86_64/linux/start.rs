use crate::libc::string::{strchr, strlen};
use crate::posix::stdlib::{exit, ARGC, ARGV, AUXV, AUX_CNT, ENV};
use crate::types::{char_t, int_t};
use alloc::borrow::ToOwned;
use alloc::string::ToString;
use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;
use cstrptr::{CStr, CString};

extern "C" {
    fn main(argc: int_t, argv: *const *const char_t, envp: *const *const char_t) -> int_t;
}

/// This function is called by _start().
/// It stores the addresses of the stack arguments, invokes main(), and passes
/// the return status to exit().
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn __libc_start_main(argc: usize, argv: *const *const char_t) -> ! {
    ARGC = argc;
    ARGV = argv;
    let envp = ARGV.add(ARGC + 1);

    let mut envc = envp;
    while !(*envc).is_null() {
        let entry = *envc;
        let index = strchr(entry, b'=' as i32);
        if !index.is_null() {
            ENV.insert(
                from_utf8_unchecked(from_raw_parts(entry as _, index.offset_from(entry) as _))
                    .to_string(),
                CStr::from_ptr(index.add(1)).to_owned(),
            );
        } else {
            ENV.insert(
                from_utf8_unchecked(from_raw_parts(entry as _, strlen(entry))).to_string(),
                CStr::from_ptr(cs!("")).to_owned(),
            );
        }
        envc = envc.add(1); // increases by one pointer size
    }

    let mut auxv = envc.add(1) as *mut usize;
    while *auxv != 0 {
        if *auxv < AUX_CNT {
            AUXV[*auxv] = *auxv.add(1);
        }
        auxv = auxv.add(2);
    }

    exit(main(ARGC as int_t, ARGV, envp))
}

/// Prevent `__libc_start_main` from being optimized away when using lto.
#[used]
static LIBC_START_MAIN: Option<unsafe extern "C" fn(usize, *const *const char_t) -> !> =
    Some(__libc_start_main);

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "xor rbp,rbp
        pop rdi
        mov rsi,rsp
        and rsp,-16
        call __libc_start_main
        hlt",
        options(noreturn)
    );
}
