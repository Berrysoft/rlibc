use crate::libc::errno::errno;
use crate::libc::string::{strlen, strncmp, strncpy, strnlen};
use crate::types::{char_t, int_t, size_t};
// use consts::NULL;
use crate::consts::errno::{EEXIST, EINVAL};
pub use crate::posix::pm::{_Exit, _exit, abort, atexit, exit};
use core::slice::from_raw_parts;

pub static mut ARGV: *const *const char_t = 0 as *const *const char_t;
pub static mut ARGC: usize = 0;
pub static mut ENVP: *const *const char_t = 0 as *const *const char_t;
pub static mut ENVC: usize = 0;

const K_ENV_MAXKEYLEN: size_t = 512;

pub unsafe fn get_argv() -> &'static [*const char_t] {
    from_raw_parts(ARGV, ARGC)
}

pub unsafe fn get_envp() -> &'static [*const char_t] {
    from_raw_parts(ENVP, ENVC)
}

#[no_mangle]
pub unsafe extern "C" fn getenv(key: *const char_t) -> *const char_t {
    let len = strnlen(key, K_ENV_MAXKEYLEN);
    for &env in get_envp().iter() {
        if strncmp(key, env, len) == 0 && *env.offset(len as isize) == '=' as i8 {
            return env.offset((len as isize) + 1);
        }
    }
    0 as *const char_t
}

#[no_mangle]
pub unsafe extern "C" fn setenv(key: *const char_t, val: *const char_t, overwrite: int_t) -> int_t {
    _exit(1); // TODO implement mutable environment
}

#[no_mangle]
pub unsafe extern "C" fn unsetenv(key: *const char_t) -> int_t {
    _exit(1); // TODO implement mutable environment
}

/*
#[no_mangle]
pub unsafe extern fn mkstemp(tplt: *mut char_t) -> int_t {
    let slc = tplt.to_mut_slice(strlen(tplt as *_) as usize);
    if slc.len() < 6 || slc.lastn(6).iter().any(|c| *c != cc!('X')) {
        errno = EINVAL;
        return -1;
    }
    let rand = os_rand();
    let mut buf: [char_t, ..6] = uninit();
    loop {
        rand.fill(buf);
        for (i, c) in slc.lastn(6).mut_iter().enumerate() {
            match buf[i] & 15 {
                0..9   => *c = buf[i] + cc!('0'),
                10..15 => *c = buf[i] + cc!('a') - 10,
            }
        }
        match open(tplt, ) {
            i if i >= 0          => return i,
            _ if errno != EEXIST => return -1,
            _ => { }
        }
    }
}
*/
