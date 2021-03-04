//! Memory management

use crate::types::off_t;
use crate::types::rlimit;
use crate::types::{int_t, intptr_t, size_t, uint_t, ulong_t, void_t};

use crate::syscalls::sys_brk;
use crate::syscalls::{sys_getrlimit, sys_mmap, sys_munmap};

use crate::consts::errno::ENOMEM;
use crate::libc::errno::errno;

/// Increases the data break to the given address, returning 0 on success
/// or -1 on failure, setting errno to ENOMEM.
#[no_mangle]
pub unsafe extern "C" fn brk(addr: *const void_t) -> int_t {
    let oldbrk = sys_brk(0) as usize;
    match sys_brk(addr as ulong_t) as usize != oldbrk {
        true => 0,
        false => {
            errno = ENOMEM;
            -1
        }
    }
}

/// Increments the data break by `increment`, returning either the previous
/// break or `((void*)-1)` on failure, setting errno to ENOMEM.
#[no_mangle]
pub unsafe extern "C" fn sbrk(increment: intptr_t) -> *const void_t {
    let oldbrk = sys_brk(0) as *const u8;
    if increment != 0 {
        let newbrk = oldbrk.offset(increment as isize);
        if sys_brk(newbrk as ulong_t) as *const u8 != newbrk {
            errno = ENOMEM;
            -1isize as *const void_t
        } else {
            oldbrk as *const void_t
        }
    } else {
        oldbrk as *const void_t
    }
}

/// Get resource limits. For RLIMIT_DATA, this is the maximum size of the
/// process's data segment. This limit affects calls to brk(2) and sbrk(2).
#[no_mangle]
pub unsafe extern "C" fn getrlimit(resource: int_t, rlim: *mut rlimit) -> int_t {
    forward!(sys_getrlimit, resource as uint_t, rlim)
}

/// Map or unmap files or devices into memory.
#[no_mangle]
pub unsafe extern "C" fn mmap(
    addr: *const void_t,
    length: size_t,
    prot: int_t,
    flags: int_t,
    fd: int_t,
    offset: off_t,
) -> *const void_t {
    forward!(
        sys_mmap,
        addr as ulong_t,
        length as ulong_t,
        prot as ulong_t,
        flags as ulong_t,
        fd as ulong_t,
        offset as ulong_t
    ) as *const void_t
}

#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *const void_t, length: size_t) -> int_t {
    forward!(sys_munmap, addr as ulong_t, length)
}
