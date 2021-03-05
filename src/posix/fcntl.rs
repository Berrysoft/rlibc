use crate::syscalls::sys_open;
use crate::types::{char_t, int_t, mode_t};

#[no_mangle]
pub unsafe extern "C" fn open(path: *const char_t, flags: int_t, mode: mode_t) -> int_t {
    forward!(sys_open, path, flags, mode as _) as _
}
