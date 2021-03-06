use crate::types::int_t;

#[no_mangle]
#[thread_local]
pub static mut errno: int_t = 0;
