use crate::posix::stdlib::{exit, ARGC, ARGV, AUXV, AUX_CNT, ENVC, ENVP};
use crate::types::{char_t, int_t};
use core::mem::MaybeUninit;

extern "C" {
    fn main(argc: int_t, argv: *const *const char_t, envp: *const *const char_t) -> int_t;
}

/// This function is called by start().
/// It stores the addresses of the stack arguments, invokes main(), and passes
/// the return status to exit().
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn __libc_start_main(argc: usize, argv: *const *const char_t) -> ! {
    ARGC = argc;
    ARGV = argv;
    ENVP = ARGV.add(ARGC + 1);

    let mut envc = ENVP;
    while !(*envc).is_null() {
        envc = envc.add(1); // increases by one pointer size
    }
    ENVC = envc.offset_from(ENVP) as usize - 1;

    let mut auxv = envc.add(1) as *mut usize;
    while *auxv != 0 {
        if *auxv < AUX_CNT {
            AUXV[*auxv] = *auxv.add(1);
        }
        auxv = auxv.add(2);
    }

    exit(main(ARGC as int_t, ARGV, ENVP))
}
