#[macro_export]
macro_rules! cs {
    ($e:expr) => {
        (concat!($e, "\0")).as_ptr() as *const char_t
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! libc_print {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            use core::fmt::Write;
            core::write!(&mut $crate::libc::stdio::__stdout, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! libc_println {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            use core::fmt::Write;
            core::writeln!(&mut $crate::libc::stdio::__stdout, $($arg)*);
        }
    };
}
