use crate::consts::errno::*;
use crate::libc::errno::errno;
use crate::posix::mm::*;
use alloc::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ptr::{null_mut, slice_from_raw_parts_mut, NonNull};

/// https://github.com/ezrosent/allocators-rs/blob/master/mmap-alloc/src/lib.rs

#[derive(Clone)]
pub struct MapAlloc {
    pagesize: usize,
    perms: i32,
    commit: bool,
}

unsafe impl Allocator for MapAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            debug_assert!(layout.size() > 0, "alloc: size of layout must be non-zero");

            let size = next_multiple(layout.size(), self.pagesize);

            // alignment less than a page is fine because page-aligned objects are also aligned to
            // any alignment less than a page
            if layout.align() <= self.pagesize {
                map(size, self.perms, self.commit).ok_or(AllocError)
            } else {
                Err(AllocError)
            }
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        debug_assert!(
            layout.size() > 0,
            "dealloc: size of layout must be non-zero"
        );
        unmap(ptr.as_ptr() as *mut u8, layout.size());
    }
}

unsafe impl GlobalAlloc for MapAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
            .map(|mut p| p.as_mut().as_mut_ptr())
            .unwrap_or(null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if !ptr.is_null() {
            self.deallocate(NonNull::new_unchecked(ptr), layout);
        }
    }
}

const fn next_multiple(size: usize, unit: usize) -> usize {
    let remainder = size % unit;
    if remainder == 0 {
        size
    } else {
        size + (unit - remainder)
    }
}

// NOTE on mapping at the NULL address: A previous version of this code explicitly checked for NULL
// being returned from mmap (on both Linux and Mac). However, it was discovered that the POSIX
// standard and the Linux manpage both guarantee that NULL will never be returned so long as the
// MAP_FIXED flag is not passed. If this ever becomes a problem in the future as we support new
// platforms, it may be helpful to see how this was dealt with in the past. The last version of the
// code that explicitly handled this condition was at commit 2caa95624b3d, and the logic was in the
// alloc_helper method. A similar check was performed for Linux's mremap call in the realloc_helper
// method.

unsafe fn map(size: usize, perms: i32, commit: bool) -> Option<NonNull<[u8]>> {
    // TODO: Figure out when it's safe to pass MAP_UNINITIALIZED (it's not defined in all
    // versions of libc). Be careful about not invalidating alloc_zeroed.

    let flags = if commit { MAP_POPULATE } else { 0 };

    let ptr = mmap(
        null_mut(),
        size,
        perms,
        MAP_ANONYMOUS | MAP_PRIVATE | flags,
        -1,
        0,
    );

    if ptr == MAP_FAILED {
        if errno == ENOMEM {
            None
        } else {
            panic!("mmap failed: {}", errno)
        }
    } else {
        // On Linux, if the MAP_FIXED flag is not supplied, mmap will never return NULL. From the
        // Linux manpage: "The portable way to create a mapping is to specify addr as 0 (NULL), and
        // omit MAP_FIXED from flags. In this case, the system chooses the address for the mapping;
        // the address is chosen so as not to conflict with any existing mapping, and will not be
        // 0."
        assert_ne!(ptr, null_mut(), "mmap returned NULL");
        Some(NonNull::new_unchecked(slice_from_raw_parts_mut(
            ptr as _, size,
        )))
    }
}

unsafe fn unmap(ptr: *mut u8, size: usize) {
    // NOTE: Don't inline the call to munmap; then errno might be called before munmap.
    let ret = munmap(ptr as *mut _, size);
    assert_eq!(ret, 0, "munmap failed: {}", errno);
}

pub const fn get_perm(read: bool, write: bool, exec: bool) -> i32 {
    match (read, write, exec) {
        (false, false, false) => PROT_NONE,
        (true, false, false) => PROT_READ,
        (false, true, false) => PROT_WRITE,
        (false, false, true) => PROT_EXEC,
        (true, true, false) => PROT_READ | PROT_WRITE,
        (true, false, true) => PROT_READ | PROT_EXEC,
        (false, true, true) => PROT_WRITE | PROT_EXEC,
        (true, true, true) => PROT_READ | PROT_WRITE | PROT_EXEC,
    }
}

#[global_allocator]
pub static ALLOCATOR: MapAlloc = MapAlloc {
    pagesize: 4096,
    perms: get_perm(true, true, false),
    commit: false,
};
