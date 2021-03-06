use crate::posix::stdlib::{exit, ARGC, ARGV, AUXV, AUX_CNT, ENVC, ENVP};
use crate::types::{char_t, int_t};

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
