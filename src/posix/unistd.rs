use crate::libc::errno::errno;
use crate::syscalls::{sys_close, sys_lseek, sys_read, sys_rmdir, sys_unlink, sys_write};
use crate::syscalls::{sys_geteuid, sys_getpid, sys_getuid, sys_setgid, sys_setsid, sys_setuid};
use crate::syscalls::{sys_pread64, sys_pwrite64};
use crate::types::off_t;
use crate::types::{char_t, int_t, size_t, ssize_t, uint_t, ulong_t, void_t};
use crate::types::{gid_t, pid_t, uid_t};

macro_rules! forward {
    ($sys:ident, $($p:expr),*) => {
        match $sys($($p),*) {
            n if n < 0 => {
                errno = -n;
                -1
            },
            n => n,
        }
    };
}

// File and filesystem manipulation

#[no_mangle]
pub unsafe extern "C" fn unlink(file: *const char_t) -> int_t {
    forward!(sys_unlink, file)
}

#[no_mangle]
pub unsafe extern "C" fn rmdir(file: *const char_t) -> int_t {
    forward!(sys_rmdir, file)
}

#[no_mangle]
pub unsafe extern "C" fn close(fd: int_t) -> int_t {
    forward!(sys_close, fd as uint_t)
}

#[no_mangle]
pub unsafe extern "C" fn read(fd: int_t, buf: *mut void_t, count: size_t) -> ssize_t {
    (forward!(sys_read, fd as uint_t, buf as *mut char_t, count)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn write(fd: int_t, buf: *const void_t, count: size_t) -> ssize_t {
    (forward!(sys_write, fd as uint_t, buf as *const char_t, count)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn pread(
    fd: int_t,
    buf: *mut void_t,
    count: size_t,
    offset: off_t,
) -> ssize_t {
    forward!(
        sys_pread64,
        fd as ulong_t,
        buf as *mut char_t,
        count,
        offset
    ) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn pwrite(
    fd: int_t,
    buf: *const void_t,
    count: size_t,
    offset: off_t,
) -> ssize_t {
    forward!(
        sys_pwrite64,
        fd as uint_t,
        buf as *const char_t,
        count,
        offset
    ) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn lseek(fd: int_t, offset: off_t, whence: int_t) -> off_t {
    forward!(sys_lseek, fd as uint_t, offset, whence as uint_t) as off_t
}

// Environment

#[no_mangle]
pub unsafe extern "C" fn getpid() -> pid_t {
    forward!(sys_getpid,) as pid_t
}

#[no_mangle]
pub unsafe extern "C" fn getuid() -> uid_t {
    forward!(sys_getuid,) as uid_t
}

#[no_mangle]
pub unsafe extern "C" fn geteuid() -> uid_t {
    forward!(sys_geteuid,) as uid_t
}

#[no_mangle]
pub unsafe extern "C" fn setuid(uid: uid_t) -> int_t {
    forward!(sys_setuid, uid)
}

#[no_mangle]
pub unsafe extern "C" fn setgid(gid: gid_t) -> int_t {
    forward!(sys_setgid, gid)
}

#[no_mangle]
pub unsafe extern "C" fn setsid() -> pid_t {
    (forward!(sys_setsid,) as pid_t)
}
