use crate::consts::errno::{EEXIST, EINVAL};
use crate::consts::fcntl::{O_CREAT, O_EXCL};
use crate::libc::errno::errno;
use crate::libc::string::{strlen, strncmp, strnlen};
use crate::posix::fcntl::open;
pub use crate::posix::pm::{_Exit, _exit, abort, atexit, exit};
use crate::rust::alloc::ALLOCATOR;
use crate::rust::rand::{os_rand, Rand};
use crate::types::{char_t, int_t, size_t, ulong_t, void_t};
use alloc::alloc::{GlobalAlloc, Layout};
use core::mem::{align_of, size_of, MaybeUninit};
use core::ptr::{null, null_mut};
use core::slice::{from_raw_parts, from_raw_parts_mut};
use core::str::from_utf8_unchecked_mut;

pub static mut ARGV: *const *const char_t = null();
pub static mut ARGC: usize = 0;
pub static mut ENVP: *const *const char_t = null();
pub static mut ENVC: usize = 0;
pub static mut AUXV: [usize; AUX_CNT] = [0; AUX_CNT];

pub const AUX_CNT: usize = 38;
pub const AT_PAGESZ: ulong_t = 6;

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

#[no_mangle]
pub unsafe extern "C" fn getauxval(t: ulong_t) -> ulong_t {
    AUXV[t as usize] as _
}

#[no_mangle]
pub unsafe extern "C" fn getpagesize() -> int_t {
    getauxval(AT_PAGESZ) as _
}

#[repr(C)]
struct MallocHeader {
    size: usize,
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: size_t) -> *mut void_t {
    let res = ALLOCATOR.alloc(Layout::from_size_align_unchecked(
        size + size_of::<MallocHeader>(),
        align_of::<MallocHeader>(),
    )) as *mut MallocHeader;
    if let Some(h) = res.as_mut() {
        h.size = size;
        res.add(1) as _
    } else {
        null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut void_t) {
    if !ptr.is_null() {
        let ptr = (ptr as *mut MallocHeader).sub(1);
        let size = ptr.as_ref().unwrap_unchecked().size;
        ALLOCATOR.dealloc(
            ptr as _,
            Layout::from_size_align_unchecked(
                size + size_of::<MallocHeader>(),
                align_of::<MallocHeader>(),
            ),
        );
    }
}
