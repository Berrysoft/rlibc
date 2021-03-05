use crate::consts::errno::{EEXIST, EISDIR};
use crate::libc::errno::errno;
use crate::libc::string::strlen;
use crate::posix::unistd::{rmdir, unlink};
use crate::syscalls::{sys_rename, sys_write};
use crate::types::{char_t, int_t, size_t};
use core::fmt::Write;
use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;

static _IOFBF: int_t = 0;
static _IOLBF: int_t = 1;
static _IONBF: int_t = 2;

static BUFSIZ: int_t = 8192;

static EOF: int_t = -1;

static FOPEN_MAX: int_t = 16;
static FILENAME_MAX: int_t = 4096;
static L_tmpnam: int_t = 20;

static SEEK_SET: int_t = 0;
static SEEK_CUR: int_t = 1;
static SEEK_END: int_t = 2;

static TMP_MAX: int_t = 238328;

#[no_mangle]
pub unsafe extern "C" fn remove(file: *const char_t) -> int_t {
    if unlink(file) == -1 {
        match errno {
            EISDIR => rmdir(file),
            _ => -1,
        }
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn rename(old: *const char_t, new: *const char_t) -> int_t {
    match sys_rename(old, new) {
        n if n < 0 => {
            errno = -n;
            -1
        }
        _ => 0,
    }
}

pub struct FILE {
    fd: int_t,
}

#[no_mangle]
pub static mut __stdin: FILE = FILE { fd: 0 };
#[no_mangle]
pub static mut __stdout: FILE = FILE { fd: 1 };
#[no_mangle]
pub static mut __stderr: FILE = FILE { fd: 2 };

impl Write for FILE {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            if sys_write(self.fd as _, s.as_ptr() as _, s.len()) as usize == s.len() {
                Ok(())
            } else {
                Err(core::fmt::Error)
            }
        }
    }
}

unsafe fn puts_impl(s: *const char_t) -> core::fmt::Result {
    let s = from_utf8_unchecked(from_raw_parts(s as _, strlen(s)));
    __stdout.write_str(s)?;
    __stdout.write_char('\n')
}

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const char_t) -> int_t {
    match puts_impl(s) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[macro_export]
macro_rules! libc_print {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            $crate::libc::stdio::__stdout.write_fmt(format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! libc_println {
    () => {
        #[allow(unused_must_use)]
        {
            $crate::libc::stdio::__stdout.write_char('\n');
        }
    };
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            $crate::libc::stdio::__stdout.write_fmt(format_args!($($arg)*));
            $crate::libc::stdio::__stdout.write_char('\n');
        }
    };
}

#[macro_export]
macro_rules! libc_write {
    ($arg:expr) => {
        #[allow(unused_must_use)]
        {
            $crate::libc::stdio::__stdout.write_str($arg);
        }
    };
}

#[macro_export]
macro_rules! libc_writeln {
    () => {
        #[allow(unused_must_use)]
        {
            $crate::libc::stdio::__stdout.write_char('\n');
        }
    };
    ($arg:expr) => {
        #[allow(unused_must_use)]
        {
            $crate::libc::stdio::__stdout.write_str($arg);
            $crate::libc::stdio::__stdout.write_char('\n');
        }
    };
}
