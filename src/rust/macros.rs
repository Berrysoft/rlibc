#[macro_export]
macro_rules! cc {
    ($e:expr) => {
        $e as char_t
    };
}

#[macro_export]
macro_rules! cs {
    ($e:expr) => {
        (concat!($e, "\0")).as_ptr() as *const char_t
    };
}

macro_rules! forward {
    ($sys:ident, $($p:expr),*) => {
        match $sys($($p),*) {
            n if n < 0 => {
                use $crate::libc::errno::{errno};
                errno = -n;
                -1
            },
            n => n,
        }
    };
}
