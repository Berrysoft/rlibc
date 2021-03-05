use crate::consts::errno::EISDIR;
use crate::libc::errno::errno;
use crate::libc::string::{strchr, strlen};
use crate::posix::unistd::{rmdir, unlink};
use crate::syscalls::{sys_rename, sys_write};
use crate::types::*;
use core::ffi::VaList;
use core::fmt::{Error, Write};
use core::intrinsics::copy_nonoverlapping;
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

struct StrBufferFile {
    data: *mut char_t,
    len: Option<size_t>,
}

impl StrBufferFile {
    pub fn from_ptr(data: *mut char_t) -> Self {
        Self { data, len: None }
    }

    pub fn from_ptr_len(data: *mut char_t, len: size_t) -> Self {
        Self {
            data,
            len: Some(len),
        }
    }
}

impl Write for StrBufferFile {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let slen = s.len();
        unsafe {
            match self.len {
                Some(len) => {
                    if slen > len {
                        Err(core::fmt::Error)
                    } else {
                        copy_nonoverlapping(s.as_bytes().as_ptr(), self.data as _, slen);
                        self.data = self.data.add(slen);
                        self.len = Some(len - slen);
                        Ok(())
                    }
                }
                None => {
                    copy_nonoverlapping(s.as_bytes().as_ptr(), self.data as _, slen);
                    self.data = self.data.add(slen);
                    Ok(())
                }
            }
        }
    }
}

struct EmptyBufferFile;

impl Write for EmptyBufferFile {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Ok(())
    }
}

type Result<T> = core::result::Result<T, Error>;

#[inline]
fn unwrap_result(res: Result<int_t>) -> int_t {
    match res {
        Ok(res) => res,
        Err(_) => -1,
    }
}

unsafe fn fputs_impl(s: *const char_t, f: &mut FILE, endl: bool) -> Result<int_t> {
    let len = strlen(s);
    if len != 0 {
        let s = from_utf8_unchecked(from_raw_parts(s as _, len));
        f.write_str(s)?;
        if endl {
            f.write_char('\n')?;
        }
    }
    Ok(0)
}

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const char_t) -> int_t {
    unwrap_result(fputs_impl(s, &mut __stdout, true))
}

#[no_mangle]
pub unsafe extern "C" fn fputs(s: *const char_t, f: *mut FILE) -> int_t {
    unwrap_result(fputs_impl(s, f.as_mut().unwrap_unchecked(), false))
}

unsafe fn fputc_impl(c: int_t, f: &mut FILE) -> Result<int_t> {
    f.write_char(core::char::from_u32_unchecked(c as u32))?;
    Ok(0)
}

#[no_mangle]
pub unsafe extern "C" fn fputc(c: int_t, f: *mut FILE) -> int_t {
    unwrap_result(fputc_impl(c, f.as_mut().unwrap_unchecked()))
}

#[no_mangle]
pub unsafe extern "C" fn vprintf(fmt: *const char_t, vlist: VaList) -> int_t {
    vfprintf(&mut __stdout, fmt, vlist)
}

#[no_mangle]
pub unsafe extern "C" fn printf(fmt: *const char_t, mut args: ...) -> int_t {
    vprintf(fmt, args.as_va_list())
}

#[no_mangle]
pub unsafe extern "C" fn vfprintf(f: *mut FILE, fmt: *const char_t, vlist: VaList) -> int_t {
    unwrap_result(vprintf_impl(f.as_mut().unwrap_unchecked(), fmt, vlist))
}

#[no_mangle]
pub unsafe extern "C" fn vsprintf(buf: *mut char_t, fmt: *const char_t, vlist: VaList) -> int_t {
    unwrap_result(vprintf_impl(&mut StrBufferFile::from_ptr(buf), fmt, vlist))
}

#[no_mangle]
pub unsafe extern "C" fn vsnprintf(
    buf: *mut char_t,
    n: size_t,
    fmt: *const char_t,
    vlist: VaList,
) -> int_t {
    unwrap_result(if n == 0 {
        vprintf_impl(&mut EmptyBufferFile, fmt, vlist)
    } else {
        vprintf_impl(&mut StrBufferFile::from_ptr_len(buf, n), fmt, vlist)
    })
}

#[derive(Debug, PartialEq, Eq)]
enum FormatSpec {
    /// None
    None,
    /// %l
    Long,
    /// %ll
    LongLong,
    /// %h
    Short,
    /// %hh
    ShortShort,
    /// %j
    Max,
    /// %z
    Size,
    /// %t
    Diff,
    /// %L
    LongDouble,
}

unsafe fn vprintf_impl(
    buf: &mut impl Write,
    fmt: *const char_t,
    mut vlist: VaList,
) -> Result<int_t> {
    let mut current = fmt;
    let mut res = 0;
    loop {
        let mut index = strchr(current, b'%' as i32);
        if index.is_null() {
            let len = strlen(current);
            buf.write_str(from_utf8_unchecked(from_raw_parts(current as _, len)))?;
            res += len;
            break;
        } else {
            let len = index.offset_from(current);
            if len != 0 {
                buf.write_str(from_utf8_unchecked(from_raw_parts(current as _, len as _)))?;
            }
            index = index.add(1);
            let (spec, off) = match *index as u8 {
                b'l' => {
                    if *index.add(1) as u8 == b'l' {
                        (FormatSpec::LongLong, 2)
                    } else {
                        (FormatSpec::Long, 1)
                    }
                }
                b'h' => {
                    if *index.add(1) as u8 == b'h' {
                        (FormatSpec::ShortShort, 2)
                    } else {
                        (FormatSpec::Short, 1)
                    }
                }
                b'j' => (FormatSpec::Max, 1),
                b'z' => (FormatSpec::Size, 1),
                b't' => (FormatSpec::Diff, 1),
                b'L' => (FormatSpec::LongDouble, 1),
                _ => (FormatSpec::None, 0),
            };
            index = index.add(off);
            let len = match *index as u8 {
                b'%' => match spec {
                    FormatSpec::None => {
                        buf.write_char('%')?;
                        1
                    }
                    _ => {
                        return Err(Error);
                    }
                },
                b'c' => match spec {
                    FormatSpec::None => {
                        let c: int_t = vlist.arg();
                        buf.write_char(c as u8 as char)?;
                        1
                    }
                    FormatSpec::Long => {
                        let c: wint_t = vlist.arg();
                        let c = core::char::from_u32_unchecked(c);
                        buf.write_char(c)?;
                        c.len_utf8()
                    }
                    _ => {
                        return Err(Error);
                    }
                },
                b's' => match spec {
                    FormatSpec::None => {
                        let s: *mut char_t = vlist.arg();
                        let len = strlen(s);
                        buf.write_str(from_utf8_unchecked(from_raw_parts(s as _, len)))?;
                        len
                    }
                    FormatSpec::Long => {
                        let _s: *mut wchar_t = vlist.arg();
                        // Needs alloc
                        todo!()
                    }
                    _ => {
                        return Err(Error);
                    }
                },
                b'd' | b'i' => {
                    let value: i128 = match spec {
                        FormatSpec::None => vlist.arg::<int_t>() as _,
                        FormatSpec::Short => vlist.arg::<short_t>() as _,
                        FormatSpec::ShortShort => vlist.arg::<char_t>() as _,
                        FormatSpec::Long => vlist.arg::<long_t>() as _,
                        FormatSpec::LongLong => vlist.arg::<longlong_t>() as _,
                        FormatSpec::Max => vlist.arg::<intmax_t>() as _,
                        FormatSpec::Size => vlist.arg::<isize>() as _,
                        FormatSpec::Diff => vlist.arg::<ptrdiff_t>() as _,
                        FormatSpec::LongDouble => {
                            return Err(Error);
                        }
                    };
                    core::write!(buf, "{}", value)?;
                    // Needs alloc to determine length
                    todo!()
                }
                _ => {
                    return Err(Error);
                }
            };
            current = index.add(1);
            res += len;
        }
    }
    Ok(res as _)
}
