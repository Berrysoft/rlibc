//! Memory management

use crate::consts::errno::ENOMEM;
use crate::libc::errno::errno;
use crate::syscalls::*;
use crate::types::*;

pub const MAP_FAILED: *mut void_t = -1 as _;

pub const MAP_PRIVATE: int_t = 0x02;
pub const MAP_ANON: int_t = 0x20;
pub const MAP_ANONYMOUS: int_t = MAP_ANON;
pub const MAP_POPULATE: int_t = 0x8000;

pub const MREMAP_MAYMOVE: ulong_t = 1;

pub const MADV_WILLNEED: int_t = 3;
pub const MADV_DONTNEED: int_t = 4;

pub const PROT_NONE: int_t = 0;
pub const PROT_READ: int_t = 1;
pub const PROT_WRITE: int_t = 2;
pub const PROT_EXEC: int_t = 4;

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
    addr: *mut void_t,
    length: size_t,
    prot: int_t,
    flags: int_t,
    fd: int_t,
    offset: off_t,
) -> *mut void_t {
    match sys_mmap(
        addr as ulong_t,
        length as ulong_t,
        prot as ulong_t,
        flags as ulong_t,
        fd as ulong_t,
        offset as ulong_t,
    ) {
        n if n < 0 => {
            errno = -n as i32;
            MAP_FAILED
        }
        n => n as _,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mremap(
    old_addr: *mut void_t,
    old_size: size_t,
    new_size: size_t,
    flags: ulong_t,
) -> *mut void_t {
    forward!(
        sys_mremap,
        old_addr as ulong_t,
        old_size as ulong_t,
        new_size as ulong_t,
        flags,
        0
    ) as *mut void_t
}

#[no_mangle]
pub unsafe extern "C" fn madvise(addr: *mut void_t, length: size_t, advice: int_t) -> int_t {
    forward!(sys_madvise, addr as ulong_t, length, advice)
}

#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *mut void_t, length: size_t) -> int_t {
    forward!(sys_munmap, addr as ulong_t, length)
}
