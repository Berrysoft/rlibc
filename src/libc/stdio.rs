use crate::consts::errno::EISDIR;
use crate::libc::errno::errno;
use crate::libc::string::{strchr, strlen};
use crate::posix::unistd::{rmdir, unlink};
use crate::syscalls::{sys_rename, sys_write};
use crate::types::*;
use alloc::format;
use alloc::string::String;
use core::ffi::VaList;
use core::fmt::{Error, Write};
use core::intrinsics::copy_nonoverlapping;
use core::ptr::null_mut;
use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;
use widestring::U32CStr;

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
    forward!(sys_rename, old, new) as _
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
        if !self.data.is_null() {
            let slen = s.len();
            unsafe {
                match self.len {
                    Some(len) => {
                        if slen + 1 > len {
                            self.data = null_mut()
                        } else {
                            copy_nonoverlapping(s.as_bytes().as_ptr(), self.data as _, slen);
                            self.data = self.data.add(slen);
                            *self.data = 0;
                            self.len = Some(len - slen);
                        }
                    }
                    None => {
                        copy_nonoverlapping(s.as_bytes().as_ptr(), self.data as _, slen);
                        self.data = self.data.add(slen);
                        *self.data = 0;
                    }
                }
            }
        }
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
pub unsafe extern "C" fn fprintf(f: *mut FILE, fmt: *const char_t, mut args: ...) -> int_t {
    vfprintf(f, fmt, args.as_va_list())
}

#[no_mangle]
pub unsafe extern "C" fn vsprintf(buf: *mut char_t, fmt: *const char_t, vlist: VaList) -> int_t {
    unwrap_result(vprintf_impl(&mut StrBufferFile::from_ptr(buf), fmt, vlist))
}

#[no_mangle]
pub unsafe extern "C" fn sprintf(buf: *mut char_t, fmt: *const char_t, mut args: ...) -> int_t {
    vsprintf(buf, fmt, args.as_va_list())
}

#[no_mangle]
pub unsafe extern "C" fn vsnprintf(
    buf: *mut char_t,
    n: size_t,
    fmt: *const char_t,
    vlist: VaList,
) -> int_t {
    unwrap_result(vprintf_impl(
        &mut StrBufferFile::from_ptr_len(buf, n),
        fmt,
        vlist,
    ))
}

#[no_mangle]
pub unsafe extern "C" fn snprintf(
    buf: *mut char_t,
    n: size_t,
    fmt: *const char_t,
    mut args: ...
) -> int_t {
    vsnprintf(buf, n, fmt, args.as_va_list())
}

#[derive(Debug, PartialEq, Eq)]
enum FormatLength {
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

#[derive(Debug, PartialEq, Eq)]
enum FormatType {
    None,
    Percent,
    Char,
    String,
    SignedInt,
    UnsignedInt,
    UnsignedOct,
    UnsignedHex(bool),
    FixedFloat,
    ExponentFloat(bool),
    ExponentHexFloat(bool),
    GeneralFloat(bool),
    CurrentLen,
    Pointer,
}

#[derive(Debug)]
enum FormatUSize {
    None,
    Some(usize),
    ReadNext,
}

impl FormatUSize {
    pub fn read_next(&self, vlist: &mut VaList) -> Option<usize> {
        match self {
            Self::None => None,
            Self::Some(v) => Some(*v),
            Self::ReadNext => {
                let v: int_t = unsafe { vlist.arg() };
                Some(v as usize)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FormatPadding {
    None,
    Space,
    Zero,
}

#[derive(Debug)]
struct FormatSpec {
    left_align: bool,
    alter: bool,
    positive: bool,
    pad_ty: FormatPadding,
    pad: FormatUSize,
    prc: FormatUSize,
    length: FormatLength,
    ty: FormatType,
}

impl Default for FormatSpec {
    fn default() -> Self {
        Self {
            left_align: false,
            alter: false,
            positive: false,
            pad_ty: FormatPadding::None,
            pad: FormatUSize::None,
            prc: FormatUSize::None,
            length: FormatLength::None,
            ty: FormatType::None,
        }
    }
}

impl FormatSpec {
    pub unsafe fn parse(mut index: *const i8) -> (*const i8, Self) {
        let mut res = Self::default();
        loop {
            match *index as u8 {
                b'-' => res.left_align = true,
                b'+' => res.positive = true,
                b' ' => res.pad_ty = FormatPadding::Space,
                b'#' => res.alter = true,
                b'0' => res.pad_ty = FormatPadding::Zero,
                _ => break,
            }
            index = index.add(1);
        }
        match *index as u8 {
            b'*' | b'1'..=b'9' => {
                let (i, p) = Self::read_usize(index);
                index = i;
                res.pad = p;
            }
            _ => {}
        }
        if *index as u8 == b'.' {
            index = index.add(1);
            match *index as u8 {
                b'*' | b'0'..=b'9' => {
                    let (i, p) = Self::read_usize(index);
                    index = i;
                    res.prc = p;
                }
                _ => res.prc = FormatUSize::Some(0),
            }
        }
        let (spec, off) = match *index as u8 {
            b'l' => {
                if *index.add(1) as u8 == b'l' {
                    (FormatLength::LongLong, 2)
                } else {
                    (FormatLength::Long, 1)
                }
            }
            b'h' => {
                if *index.add(1) as u8 == b'h' {
                    (FormatLength::ShortShort, 2)
                } else {
                    (FormatLength::Short, 1)
                }
            }
            b'j' => (FormatLength::Max, 1),
            b'z' => (FormatLength::Size, 1),
            b't' => (FormatLength::Diff, 1),
            b'L' => (FormatLength::LongDouble, 1),
            _ => (FormatLength::None, 0),
        };
        index = index.add(off);
        res.length = spec;
        res.ty = match *index as u8 {
            b'%' => FormatType::Percent,
            b'c' => FormatType::Char,
            b's' => FormatType::String,
            b'd' | b'i' => FormatType::SignedInt,
            b'o' => FormatType::UnsignedOct,
            b'x' => FormatType::UnsignedHex(false),
            b'X' => FormatType::UnsignedHex(true),
            b'u' => FormatType::UnsignedInt,
            b'f' | b'F' => FormatType::FixedFloat,
            b'e' => FormatType::ExponentFloat(false),
            b'E' => FormatType::ExponentFloat(true),
            b'a' => FormatType::ExponentHexFloat(false),
            b'A' => FormatType::ExponentHexFloat(true),
            b'g' => FormatType::GeneralFloat(false),
            b'G' => FormatType::GeneralFloat(true),
            b'n' => FormatType::CurrentLen,
            b'p' => FormatType::Pointer,
            _ => FormatType::None,
        };
        index = index.add(1);
        (index, res)
    }

    unsafe fn read_usize(mut index: *const i8) -> (*const i8, FormatUSize) {
        if *index as u8 == b'*' {
            index = index.add(1);
            return (index, FormatUSize::ReadNext);
        }
        let mut off = 0;
        let mut pad: usize = 0;
        while (*index.add(off) as u8 as char).is_ascii_digit() {
            pad *= 10;
            pad += (*index.add(off) as u8 - b'0') as usize;
            off += 1;
        }
        if off == 0 {
            (index, FormatUSize::None)
        } else {
            index = index.add(off);
            (index, FormatUSize::Some(pad))
        }
    }

    fn read_pad_prc(&self, vlist: &mut VaList) -> (Option<usize>, Option<usize>) {
        (self.pad.read_next(vlist), self.prc.read_next(vlist))
    }

    fn fmt_signed_int(&self, vlist: &mut VaList) -> Result<String> {
        let value: i128 = unsafe {
            match self.length {
                FormatLength::None => vlist.arg::<int_t>() as _,
                FormatLength::Short => vlist.arg::<short_t>() as _,
                FormatLength::ShortShort => vlist.arg::<char_t>() as _,
                FormatLength::Long => vlist.arg::<long_t>() as _,
                FormatLength::LongLong => vlist.arg::<longlong_t>() as _,
                FormatLength::Max => vlist.arg::<intmax_t>() as _,
                FormatLength::Size => vlist.arg::<isize>() as _,
                FormatLength::Diff => vlist.arg::<ptrdiff_t>() as _,
                FormatLength::LongDouble => {
                    return Err(Error);
                }
            }
        };
        let (pad, prc) = self.read_pad_prc(vlist);
        let s = if self.positive {
            if self.left_align {
                match pad {
                    Some(pad) => match prc {
                        Some(prc) => {
                            format!("{:<1$}", format!("{:+01$}", value, prc), pad)
                        }
                        None => format!("{:+<1$}", value, pad),
                    },
                    None => return Err(Error),
                }
            } else {
                match pad {
                    Some(pad) => match self.pad_ty {
                        FormatPadding::None | FormatPadding::Space => {
                            format!(
                                "{:>1$}",
                                match prc {
                                    Some(prc) => format!("{:+>1$}", value, prc + 1),
                                    None => format!("{:+}", value),
                                },
                                pad
                            )
                        }
                        FormatPadding::Zero => {
                            format!("{:+01$}", value, pad.max(prc.unwrap_or_default() + 1))
                        }
                    },
                    None => match self.pad_ty {
                        FormatPadding::None => match prc {
                            Some(prc) => format!("{:+>1$}", value, prc + 1),
                            None => format!("{:+}", value),
                        },
                        _ => return Err(Error),
                    },
                }
            }
        } else {
            if self.left_align {
                match pad {
                    Some(pad) => match self.pad_ty {
                        FormatPadding::None | FormatPadding::Zero => match prc {
                            Some(prc) => {
                                format!("{:<1$}", format!("{:01$}", value, prc), pad)
                            }
                            None => format!("{:<1$}", value, pad),
                        },
                        FormatPadding::Space => match prc {
                            Some(prc) => format!(
                                "{:<1$}",
                                if value >= 0 {
                                    format!("{:01$}", value, prc)
                                } else {
                                    format!("{:01$}", value, prc + 1)
                                },
                                pad
                            ),
                            None => {
                                if value >= 0 {
                                    format!(" {:<1$}", value, pad - 1)
                                } else {
                                    format!("{:<1$}", value, pad)
                                }
                            }
                        },
                    },
                    None => return Err(Error),
                }
            } else {
                match pad {
                    Some(pad) => match self.pad_ty {
                        FormatPadding::None | FormatPadding::Space => match prc {
                            Some(prc) => {
                                format!("{:>1$}", format!("{:01$}", value, prc), pad)
                            }
                            None => format!("{:>1$}", value, pad),
                        },
                        FormatPadding::Zero => match prc {
                            Some(prc) if prc > pad => {
                                if value >= 0 {
                                    format!("{:01$}", value, prc)
                                } else {
                                    format!("{:01$}", value, prc + 1)
                                }
                            }
                            _ => {
                                if value >= 0 {
                                    format!("{:01$}", value, pad)
                                } else {
                                    format!("{:01$}", value, pad)
                                }
                            }
                        },
                    },
                    None => match prc {
                        Some(prc) => {
                            if value >= 0 {
                                match self.pad_ty {
                                    FormatPadding::Space => {
                                        format!(" {:01$}", value, prc)
                                    }
                                    _ => {
                                        format!("{:01$}", value, prc)
                                    }
                                }
                            } else {
                                format!("{:01$}", value, prc + 1)
                            }
                        }
                        None => format!("{}", value),
                    },
                }
            }
        };
        Ok(s)
    }
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
            let (i, spec) = FormatSpec::parse(index);
            index = i;
            let len = match spec.ty {
                FormatType::Percent => match spec.length {
                    FormatLength::None => {
                        buf.write_char('%')?;
                        1
                    }
                    _ => {
                        return Err(Error);
                    }
                },
                FormatType::Char => match spec.length {
                    FormatLength::None => {
                        let c: int_t = vlist.arg();
                        buf.write_char(c as u8 as char)?;
                        1
                    }
                    FormatLength::Long => {
                        let c: wint_t = vlist.arg();
                        let c = core::char::from_u32_unchecked(c);
                        buf.write_char(c)?;
                        c.len_utf8()
                    }
                    _ => {
                        return Err(Error);
                    }
                },
                FormatType::String => match spec.length {
                    FormatLength::None => {
                        let s: *mut char_t = vlist.arg();
                        let len = strlen(s);
                        buf.write_str(from_utf8_unchecked(from_raw_parts(s as _, len)))?;
                        len
                    }
                    FormatLength::Long => {
                        let s: *mut wchar_t = vlist.arg();
                        let s = U32CStr::from_ptr_str(s);
                        let s = s.to_string_lossy();
                        buf.write_str(&s)?;
                        s.len()
                    }
                    _ => {
                        return Err(Error);
                    }
                },
                FormatType::SignedInt => {
                    let s = spec.fmt_signed_int(&mut vlist)?;
                    buf.write_str(&s)?;
                    s.len()
                }
                _ => {
                    return Err(Error);
                }
            };
            current = index;
            res += len;
        }
    }
    Ok(res as _)
}
