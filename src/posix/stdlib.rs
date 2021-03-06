use crate::consts::errno::{EEXIST, EINVAL};
use crate::consts::fcntl::{O_CREAT, O_EXCL};
use crate::libc::errno::errno;
use crate::libc::string::{strlen, strnlen};
use crate::posix::fcntl::open;
pub use crate::posix::pm::{_Exit, _exit, abort, atexit, exit};
use crate::rust::alloc::ALLOCATOR;
use crate::rust::rand::{os_rand, Rand};
use crate::types::{char_t, int_t, size_t, ulong_t, void_t};
use alloc::alloc::{GlobalAlloc, Layout};
use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use core::mem::{align_of, size_of, MaybeUninit};
use core::ptr::{null, null_mut};
use core::slice::{from_raw_parts, from_raw_parts_mut};
use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};
use cstrptr::{CStr, CString};

pub static mut ARGV: *const *const char_t = null();
pub static mut ARGC: usize = 0;
pub static mut ENV: BTreeMap<String, CString> = BTreeMap::new();
pub static mut AUXV: [usize; AUX_CNT] = [0; AUX_CNT];

pub const AUX_CNT: usize = 38;
pub const AT_PAGESZ: ulong_t = 6;

const K_ENV_MAXKEYLEN: size_t = 512;

#[no_mangle]
pub unsafe extern "C" fn getenv(key: *const char_t) -> *const char_t {
    let len = strnlen(key, K_ENV_MAXKEYLEN);
    match ENV.get(from_utf8_unchecked(from_raw_parts(key as _, len))) {
        Some(value) => value.as_c_str().as_ptr(),
        None => null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn setenv(key: *const char_t, val: *const char_t, overwrite: int_t) -> int_t {
    let len = strnlen(key, K_ENV_MAXKEYLEN);
    let key = from_utf8_unchecked(from_raw_parts(key as _, len));
    match ENV.get_mut(key) {
        Some(value) => {
            if overwrite != 0 {
                *value = CStr::from_ptr(val).to_owned();
            }
        }
        None => {
            ENV.insert(key.to_string(), CStr::from_ptr(val).to_owned());
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn unsetenv(key: *const char_t) -> int_t {
    let len = strnlen(key, K_ENV_MAXKEYLEN);
    let key = from_utf8_unchecked(from_raw_parts(key as _, len));
    ENV.remove(key);
    0
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

impl MallocHeader {
    pub unsafe fn wrap(ptr: *mut u8, size: usize) -> *mut Self {
        let ptr: *mut Self = ptr.cast();
        if let Some(h) = ptr.as_mut() {
            h.size = size;
            ptr.add(1)
        } else {
            null_mut()
        }
    }

    pub unsafe fn unwrap(ptr: *mut Self) -> (*mut u8, usize) {
        let ptr = ptr.sub(1);
        let size = ptr.as_ref().unwrap_unchecked().size;
        (ptr as _, size)
    }
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: size_t) -> *mut void_t {
    let res = ALLOCATOR.alloc(Layout::from_size_align_unchecked(
        size + size_of::<MallocHeader>(),
        align_of::<MallocHeader>(),
    ));
    MallocHeader::wrap(res, size) as _
}

#[no_mangle]
pub unsafe extern "C" fn calloc(num: size_t, size: size_t) -> *mut void_t {
    let size = num * size;
    let res = ALLOCATOR.alloc_zeroed(Layout::from_size_align_unchecked(
        size + size_of::<MallocHeader>(),
        align_of::<MallocHeader>(),
    ));
    MallocHeader::wrap(res, size) as _
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut void_t, new_size: size_t) -> *mut void_t {
    if ptr.is_null() {
        malloc(new_size)
    } else {
        let (ptr, size) = MallocHeader::unwrap(ptr as _);
        let new_ptr = ALLOCATOR.realloc(
            ptr,
            Layout::from_size_align_unchecked(
                size + size_of::<MallocHeader>(),
                align_of::<MallocHeader>(),
            ),
            new_size,
        );
        MallocHeader::wrap(new_ptr, new_size) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut void_t) {
    if !ptr.is_null() {
        let (ptr, size) = MallocHeader::unwrap(ptr as _);
        ALLOCATOR.dealloc(
            ptr,
            Layout::from_size_align_unchecked(
                size + size_of::<MallocHeader>(),
                align_of::<MallocHeader>(),
            ),
        );
    }
}
