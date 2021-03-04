use crate::types::{char_t, int_t, size_t};

use crate::consts::fcntl::O_RDONLY;

use core::mem::size_of;
use core::ops::Drop;

use crate::posix::fcntl::open;
use crate::posix::unistd::{close, read};

pub struct FD {
    fd: int_t,
}

impl FD {
    pub fn raw(&self) -> int_t {
        self.fd
    }
}

impl Drop for FD {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}

pub trait Rand {
    fn fill<T>(&self, dst: &mut [T]);
}

pub struct OSRand {
    fd: FD,
}

impl Rand for OSRand {
    fn fill<T>(&self, dst: &mut [T]) {
        unsafe {
            read(
                self.fd.raw(),
                dst.as_mut_ptr() as *mut _,
                (dst.len() * size_of::<T>()) as size_t,
            );
        }
    }
}

pub fn os_rand() -> OSRand {
    let fd = unsafe { open(cs!("/dev/urandom"), O_RDONLY, 0) };
    OSRand { fd: FD { fd: fd } }
}
