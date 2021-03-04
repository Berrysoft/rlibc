use crate::libc::errno::errno;
use crate::syscalls::sys_open;
use crate::types::{char_t, int_t, mode_t};

#[no_mangle]
pub unsafe extern "C" fn open(path: *const char_t, flags: int_t, mode: mode_t) -> int_t {
    /*
    match sys_open(path, flags, mode) {
        n if n < 0 => {
            errno = -n;
            -1
        },
        n => n,
    }
    */
    0
}
