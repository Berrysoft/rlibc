use crate::consts::errno::{EINTR, EINVAL, EISDIR};
use crate::libc::errno::errno;
use crate::libc::string::{memchr, strlen};
use crate::posix::unistd::{read, rmdir, unlink, write};
use crate::syscalls::sys_rename;
use crate::types::*;
use alloc::vec;
use alloc::vec::Vec;
use core::intrinsics::{copy, copy_nonoverlapping};
use core::ptr::null_mut;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use core2::io::*;

const _IOFBF: int_t = 0;
const _IOLBF: int_t = 1;
const _IONBF: int_t = 2;

const BUFSIZ: usize = 8192;

const EOF: int_t = -1;

const FOPEN_MAX: int_t = 16;
const FILENAME_MAX: int_t = 4096;
const L_tmpnam: int_t = 20;

const SEEK_SET: int_t = 0;
const SEEK_CUR: int_t = 1;
const SEEK_END: int_t = 2;

const TMP_MAX: int_t = 238328;

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

#[derive(Debug)]
enum FileBufferType {
    Full,
    Line,
    None,
}

#[derive(Debug)]
enum FileBuffer {
    Internal([char_t; BUFSIZ]),
    Extended(Vec<char_t>),
    External(StrBuffer),
}

#[derive(Debug)]
pub struct FILE {
    fd: int_t,
    buffer: FileBuffer,
    buffer_len: usize,
    buffer_rlen: usize,
    buffer_ty: FileBufferType,
}

impl FILE {
    pub const fn from_fd(fd: int_t) -> Self {
        Self {
            fd,
            buffer: FileBuffer::Internal([0; BUFSIZ]),
            buffer_len: 0,
            buffer_rlen: 0,
            buffer_ty: FileBufferType::Line,
        }
    }

    fn buffer_size(&self) -> usize {
        match &self.buffer {
            FileBuffer::Internal(_) => BUFSIZ,
            FileBuffer::Extended(buf) => buf.len(),
            FileBuffer::External(buf) => buf.len.unwrap_or(BUFSIZ),
        }
    }

    fn capacity(&self) -> usize {
        self.buffer_size() - self.buffer_len
    }

    unsafe fn buffer_ptr(&mut self) -> *mut char_t {
        match &mut self.buffer {
            FileBuffer::Internal(buf) => buf.as_mut_ptr(),
            FileBuffer::Extended(buf) => buf.as_mut_ptr(),
            FileBuffer::External(buf) => buf.data,
        }
        .add(self.buffer_len)
    }

    unsafe fn write_directly(&mut self, buf: &[u8]) -> Result<usize> {
        let len = write(self.fd, buf.as_ptr() as _, buf.len());
        if len >= 0 {
            Ok(len as usize)
        } else {
            Err(match_errno_io(errno).into())
        }
    }

    unsafe fn write_buffered(&mut self, buf: &[u8]) -> Result<usize> {
        if buf.len() > self.buffer_size() {
            self.write_directly(buf)
        } else {
            copy_nonoverlapping(buf.as_ptr(), self.buffer_ptr() as *mut u8, buf.len());
            self.buffer_len += buf.len();
            self.buffer_rlen -= self.buffer_rlen.min(buf.len());
            Ok(buf.len())
        }
    }

    unsafe fn read_directly(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = read(self.fd, buf.as_mut_ptr() as _, buf.len());
        if len >= 0 {
            Ok(len as usize)
        } else {
            Err(match_errno_io(errno).into())
        }
    }
}

#[no_mangle]
pub static mut __stdin: FILE = FILE::from_fd(0);
#[no_mangle]
pub static mut __stdout: FILE = FILE::from_fd(1);
#[no_mangle]
pub static mut __stderr: FILE = FILE::from_fd(2);

const fn match_errno_io(err: errno_t) -> ErrorKind {
    match err {
        EINTR => ErrorKind::Interrupted,
        _ => ErrorKind::Other,
    }
}

impl Write for FILE {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let cap = self.capacity();
        if buf.len() > cap {
            self.flush()?;
        }
        unsafe {
            match self.buffer_ty {
                FileBufferType::None => self.write_directly(buf),
                FileBufferType::Full => self.write_buffered(buf),
                FileBufferType::Line => {
                    let mut len = 0;
                    let mut buf = buf;
                    loop {
                        let index = memchr(buf.as_ptr() as _, b'\n' as _, buf.len());
                        if index.is_null() {
                            len += self.write_buffered(buf)?;
                            break;
                        } else {
                            let index = index.offset_from(buf.as_ptr() as _) as usize;
                            len += self.write_buffered(buf.get_unchecked(..=index))?;
                            self.flush()?;
                            buf = buf.get_unchecked((index + 1)..);
                        }
                    }
                    Ok(len)
                }
            }
        }
    }

    fn flush(&mut self) -> Result<()> {
        let len = self.buffer_len;
        self.buffer_len = 0;
        self.buffer_rlen = 0;
        unsafe {
            let ptr = self.buffer_ptr() as _;
            self.write_directly(from_raw_parts(ptr, len))?;
        }
        Ok(())
    }
}

impl core2::io::Read for FILE {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unsafe {
            if self.buffer_rlen > 0 {
                let len = self.buffer_rlen.min(buf.len());
                copy_nonoverlapping(self.buffer_ptr(), buf.as_mut_ptr() as _, len);
                Ok(len)
            } else {
                self.read_directly(buf)
            }
        }
    }
}

impl core2::io::BufRead for FILE {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        unsafe {
            if self.capacity() - self.buffer_rlen == 0 {
                self.flush()?;
            }
            let ptr = self.buffer_ptr();
            let buf = from_raw_parts_mut(
                ptr.add(self.buffer_rlen) as _,
                self.capacity() - self.buffer_rlen,
            );
            self.buffer_rlen += self.read_directly(buf)?;
            Ok(from_raw_parts(ptr as _, self.buffer_rlen))
        }
    }

    fn consume(&mut self, amt: usize) {
        unsafe {
            let ptr = self.buffer_ptr();
            copy(ptr.add(amt), ptr, self.buffer_size() - amt);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fileno(f: *mut FILE) -> int_t {
    f.as_ref().unwrap_unchecked().fd
}

#[no_mangle]
pub unsafe extern "C" fn setvbuf(f: *mut FILE, buf: *mut char_t, mode: int_t, n: size_t) -> int_t {
    let f = f.as_mut().unwrap_unchecked();
    f.buffer_ty = match mode {
        _IOFBF => FileBufferType::Full,
        _IOLBF => FileBufferType::Line,
        _IONBF => FileBufferType::None,
        _ => {
            errno = EINVAL;
            return -1;
        }
    };
    if buf.is_null() {
        if n == BUFSIZ {
            f.buffer = FileBuffer::Internal([0; BUFSIZ]);
        } else {
            f.buffer = FileBuffer::Extended(vec![0; n]);
        }
    } else {
        f.buffer = FileBuffer::External(StrBuffer::from_ptr_len(buf, n));
    }
    f.buffer_len = 0;
    f.buffer_rlen = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn setbuf(f: *mut FILE, buf: *mut char_t) -> int_t {
    if buf.is_null() {
        setvbuf(f, buf, _IOFBF, BUFSIZ)
    } else {
        setvbuf(f, null_mut(), _IONBF, 0)
    }
}

fn read_until<R: BufRead + ?Sized>(r: &mut R, delim: u8, buf: &mut impl Write) -> Result<usize> {
    let mut read = 0;
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            match unsafe {
                memchr(available.as_ptr() as _, delim as _, available.len())
                    .as_ref()
                    .map(|p| (p as *const i8 as *const u8).offset_from(available.as_ptr()) as usize)
            } {
                Some(i) => {
                    buf.write(&available[..i])?;
                    (true, i)
                }
                None => {
                    buf.write(available)?;
                    (false, available.len())
                }
            }
        };
        r.consume(used);
        read += used;
        if done || used == 0 {
            return Ok(read);
        }
    }
}
#[derive(Debug)]
struct StrBuffer {
    data: *mut char_t,
    len: Option<size_t>,
}

impl StrBuffer {
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

impl core2::io::Write for StrBuffer {
    fn write(&mut self, s: &[u8]) -> core2::io::Result<usize> {
        if !self.data.is_null() {
            let slen = s.len();
            unsafe {
                match self.len {
                    Some(len) => {
                        if slen + 1 > len {
                            self.data = null_mut()
                        } else {
                            copy_nonoverlapping(s.as_ptr(), self.data as _, slen);
                            self.data = self.data.add(slen);
                            *self.data = 0;
                            self.len = Some(len - slen);
                        }
                    }
                    None => {
                        copy_nonoverlapping(s.as_ptr(), self.data as _, slen);
                        self.data = self.data.add(slen);
                        *self.data = 0;
                    }
                }
            }
            Ok(slen)
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

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
        let s = from_raw_parts(s as _, len);
        f.write(s)?;
        if endl {
            write!(f, "\n")?;
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
    write!(f, "{}", core::char::from_u32_unchecked(c as u32))?;
    Ok(0)
}

#[no_mangle]
pub unsafe extern "C" fn fputc(c: int_t, f: *mut FILE) -> int_t {
    unwrap_result(fputc_impl(c, f.as_mut().unwrap_unchecked()))
}

#[no_mangle]
pub unsafe extern "C" fn putchar(c: int_t) -> int_t {
    fputc(c, &mut __stdout)
}

unsafe fn fgets_impl(s: *mut char_t, len: Option<size_t>, f: &mut FILE) -> Result<()> {
    let mut buf = match len {
        Some(len) => StrBuffer::from_ptr_len(s, len),
        None => StrBuffer::from_ptr(s),
    };
    read_until(f, b'\n', &mut buf)?;
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn gets(s: *mut char_t) -> *mut char_t {
    match fgets_impl(s, None, &mut __stdin) {
        Ok(_) => s,
        Err(_) => null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn fgets(s: *mut char_t, n: int_t, f: *mut FILE) -> *mut char_t {
    match fgets_impl(s, Some(n as size_t), f.as_mut().unwrap_unchecked()) {
        Ok(_) => s,
        Err(_) => null_mut(),
    }
}

unsafe fn fgetc_impl(f: &mut FILE) -> Result<int_t> {
    let mut buf = [0];
    f.read(&mut buf)?;
    Ok(buf[0] as _)
}

#[no_mangle]
pub unsafe extern "C" fn fgetc(f: *mut FILE) -> int_t {
    unwrap_result(fgetc_impl(f.as_mut().unwrap_unchecked()))
}

#[no_mangle]
pub unsafe extern "C" fn getchar() -> int_t {
    fgetc(&mut __stdin)
}

unsafe fn fflush_impl(f: &mut FILE) -> Result<int_t> {
    f.flush()?;
    Ok(0)
}

#[no_mangle]
pub unsafe extern "C" fn fflush(f: *mut FILE) -> int_t {
    unwrap_result(fflush_impl(f.as_mut().unwrap_unchecked()))
}

pub mod printf;
