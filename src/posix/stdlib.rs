use crate::consts::errno::{EEXIST, EINVAL};
use crate::consts::fcntl::{O_CREAT, O_EXCL};
use crate::libc::errno::errno;
use crate::libc::string::{strlen, strncmp, strnlen};
use crate::posix::fcntl::open;
pub use crate::posix::pm::{_Exit, _exit, abort, atexit, exit};
use crate::rust::rand::{os_rand, Rand};
use crate::types::{char_t, int_t, size_t};
use core::mem::MaybeUninit;
use core::ptr::null;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use core::str::from_utf8_unchecked_mut;

pub static mut ARGV: *const *const char_t = null();
pub static mut ARGC: usize = 0;
pub static mut ENVP: *const *const char_t = null();
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
        if strncmp(key, env, len) == 0 && *env.add(len) == b'=' as _ {
            return env.add(len + 1);
        }
    }
    null()
}

#[no_mangle]
pub unsafe extern "C" fn setenv(key: *const char_t, val: *const char_t, overwrite: int_t) -> int_t {
    unimplemented!(); // TODO implement mutable environment
}

#[no_mangle]
pub unsafe extern "C" fn unsetenv(key: *const char_t) -> int_t {
    unimplemented!(); // TODO implement mutable environment
}

#[no_mangle]
pub unsafe extern "C" fn mkstemp(tplt: *mut char_t) -> int_t {
    let slc = from_utf8_unchecked_mut(from_raw_parts_mut(tplt as *mut _, strlen(tplt)));
    let slc_len = slc.len();
    if slc_len < 6 || !slc.ends_with("XXXXXX") {
        errno = EINVAL;
        return -1;
    }
    let rand = os_rand();
    let mut buf: [u8; 6] = MaybeUninit::uninit().assume_init();
    loop {
        rand.fill(&mut buf);
        for (i, c) in slc
            .as_bytes_mut()
            .get_unchecked_mut((slc_len - 6)..)
            .iter_mut()
            .enumerate()
        {
            match buf[i] & 15 {
                0..=9 => *c = buf[i] + b'0',
                10..=15 => *c = buf[i] + b'a' - 10,
                _ => unreachable!(),
            }
        }
        match open(tplt, O_CREAT | O_EXCL, 0o600) {
            i if i >= 0 => return i,
            _ if errno != EEXIST => return -1,
            _ => {}
        }
    }
}
