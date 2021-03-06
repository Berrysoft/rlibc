use crate::types::errno_t;

#[thread_local]
pub static mut errno: errno_t = 0;

#[no_mangle]
pub unsafe extern "C" fn __p_errno() -> *mut errno_t {
    &mut errno
}
