use crate::types::int_t;

pub const O_RDONLY: int_t = 0o0;
pub const O_WRONLY: int_t = 0o1;
pub const O_RDWR: int_t = 0o2;
pub const O_CREAT: int_t = 0o100;
pub const O_EXCL: int_t = 0o200;
pub const O_NOCTTY: int_t = 0o400;
pub const O_TRUNC: int_t = 0o1000;
pub const O_APPEND: int_t = 0o2000;
pub const O_NONBLOCK: int_t = 0o4000;
pub const O_NDELAY: int_t = O_NONBLOCK;
pub const O_SYNC: int_t = 0o4010000;
pub const O_FSYNC: int_t = O_SYNC;
pub const O_ASYNC: int_t = 0o20000;
