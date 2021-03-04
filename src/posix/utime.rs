//! File access and modification

use crate::syscalls::sys_utimes;
use crate::types::{char_t, int_t, timeval, utimbuf};

/// Change file last access and modification times.
#[no_mangle]
pub unsafe extern "C" fn utime(path: *const char_t, times: *const utimbuf) -> int_t {
    if times.is_null() {
        let mut tv = [
            timeval {
                tv_sec: (*times).actime,
                tv_usec: 0,
            },
            timeval {
                tv_sec: (*times).modtime,
                tv_usec: 0,
            },
        ];
        forward!(sys_utimes, path, tv.as_mut_ptr())
    } else {
        forward!(sys_utimes, path, 0 as *mut timeval)
    }
}
