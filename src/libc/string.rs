use crate::types::{char_t, int_t, size_t};
pub use compiler_builtins::mem;
use core::cmp::Ordering;
use core::ptr::{null, null_mut};
use core::slice::from_raw_parts_mut;

struct MemMutIter<'a, B: MemBound> {
    data: &'a mut char_t,
    bound: B,
}

impl<'a, B: MemBound> MemMutIter<'a, B> {
    pub fn from_ptr(data: *mut char_t, bound: B) -> Self {
        Self {
            data: unsafe { data.as_mut().unwrap_unchecked() },
            bound,
        }
    }
}

impl<'a, B: MemBound> Iterator for MemMutIter<'a, B> {
    type Item = &'a mut char_t;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bound.test(*self.data) {
            let res = self.data as *mut char_t;
            self.data = unsafe { res.add(1).as_mut().unwrap_unchecked() };
            Some(unsafe { res.as_mut().unwrap_unchecked() })
        } else {
            None
        }
    }
}

struct MemIter<'a, B: MemBound> {
    data: &'a char_t,
    bound: B,
}

impl<'a, B: MemBound> MemIter<'a, B> {
    pub fn from_ptr(data: *const char_t, bound: B) -> Self {
        let data = unsafe { data.as_ref().unwrap_unchecked() };
        Self { data, bound }
    }

    pub fn into_rev(self) -> MemRevIter<'a, LenBound> {
        let len = self.bound.len(self.data as *const _);
        MemRevIter::from_ptr(
            unsafe { (self.data as *const char_t).add(len - 1) },
            LenBound { len },
        )
    }
}

impl<'a, B: MemBound> Iterator for MemIter<'a, B> {
    type Item = &'a char_t;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bound.test(*self.data) {
            let res = self.data;
            self.data = unsafe { (res as *const char_t).add(1).as_ref().unwrap_unchecked() };
            Some(res)
        } else {
            None
        }
    }
}

struct MemRevIter<'a, B: MemBound> {
    data: &'a char_t,
    bound: B,
}

impl<'a, B: MemBound> MemRevIter<'a, B> {
    pub fn from_ptr(data: *const char_t, bound: B) -> Self {
        Self {
            data: unsafe { data.as_ref().unwrap_unchecked() },
            bound,
        }
    }
}

impl<'a, B: MemBound> Iterator for MemRevIter<'a, B> {
    type Item = &'a char_t;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bound.test(*self.data) {
            let res = self.data as *const char_t;
            self.data = unsafe { res.sub(1).as_ref().unwrap_unchecked() };
            Some(unsafe { res.as_ref().unwrap_unchecked() })
        } else {
            None
        }
    }
}

trait MemBound {
    fn test(&mut self, data: char_t) -> bool;
    fn len(&self, _: *const char_t) -> usize {
        unimplemented!()
    }
    fn and<B2: MemBound>(self, b2: B2) -> AndBound<Self, B2>
    where
        Self: Sized,
    {
        AndBound::new(self, b2)
    }
}

struct NoBound;

impl MemBound for NoBound {
    fn test(&mut self, _: char_t) -> bool {
        true
    }
}

struct LenBound {
    pub len: size_t,
}

impl MemBound for LenBound {
    fn len(&self, _: *const char_t) -> usize {
        self.len
    }
    fn test(&mut self, _: char_t) -> bool {
        let res = self.len != 0;
        if res {
            self.len -= 1;
        }
        res
    }
}

struct CStrBound;

impl MemBound for CStrBound {
    fn len(&self, data: *const char_t) -> usize {
        unsafe { strlen(data) }
    }
    fn test(&mut self, data: char_t) -> bool {
        data != 0
    }
}

struct AndBound<B1: MemBound, B2: MemBound> {
    b1: B1,
    b2: B2,
}

impl<B1: MemBound, B2: MemBound> AndBound<B1, B2> {
    pub fn new(b1: B1, b2: B2) -> Self {
        Self { b1, b2 }
    }
}

impl<B1: MemBound, B2: MemBound> MemBound for AndBound<B1, B2> {
    fn test(&mut self, data: char_t) -> bool {
        self.b1.test(data) && self.b2.test(data)
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut char_t, src: *const char_t, n: size_t) -> *mut char_t {
    mem::memcpy(dst as _, src as _, n) as _
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut char_t, src: *const char_t, n: size_t) -> *mut char_t {
    mem::memmove(dst as _, src as _, n) as _
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(dst: *mut char_t, src: *const char_t) -> *mut char_t {
    for (dst, src) in MemMutIter::from_ptr(dst, NoBound).zip(MemIter::from_ptr(src, NoBound)) {
        *dst = *src;
        if *src == 0 {
            break;
        }
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(dst: *mut char_t, src: *const char_t, n: size_t) -> *mut char_t {
    let mut zeroed = false;
    for (dst, src) in
        MemMutIter::from_ptr(dst, LenBound { len: n }).zip(MemIter::from_ptr(src, NoBound))
    {
        if zeroed {
            *dst = 0;
        } else {
            *dst = *src;
            if *src == 0 {
                zeroed = true;
            }
        }
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn strcat(dst: *mut char_t, src: *const char_t) -> *mut char_t {
    let base = strlen(dst);
    strcpy(dst.add(base), src);
    dst
}

#[no_mangle]
pub unsafe extern "C" fn strncat(dst: *mut char_t, src: *const char_t, n: size_t) -> *mut char_t {
    let base = strlen(dst);
    strncpy(dst.add(base), src, n);
    dst
}

#[inline]
fn iter_cmp<'a>(
    m1: impl Iterator<Item = &'a char_t>,
    m2: impl Iterator<Item = &'a char_t>,
) -> int_t {
    match m1.cmp(m2) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(m1: *const char_t, m2: *const char_t, n: size_t) -> int_t {
    mem::memcmp(m1 as _, m2 as _, n)
}

#[no_mangle]
unsafe extern "C" fn bcmp(m1: *const char_t, m2: *const char_t, n: size_t) -> int_t {
    mem::bcmp(m1 as _, m2 as _, n)
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(m1: *const char_t, m2: *const char_t) -> int_t {
    iter_cmp(
        MemIter::from_ptr(m1, CStrBound),
        MemIter::from_ptr(m2, CStrBound),
    )
}

#[no_mangle]
pub unsafe extern "C" fn strcoll(m1: *const char_t, m2: *const char_t) -> int_t {
    strcmp(m1, m2)
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(m1: *const char_t, m2: *const char_t, n: size_t) -> int_t {
    iter_cmp(
        MemIter::from_ptr(m1, LenBound { len: n }.and(CStrBound)),
        MemIter::from_ptr(m2, LenBound { len: n }.and(CStrBound)),
    )
}

#[no_mangle]
pub unsafe extern "C" fn strxfrm(dst: *mut char_t, src: *const char_t, n: size_t) -> size_t {
    let len = strlen(src);
    if len < n {
        memcpy(dst, src, len + 1);
    }
    len
}

#[inline]
fn iter_chr<'a>(s: impl Iterator<Item = &'a char_t>, c: char_t) -> Option<usize> {
    for (i, s) in s.enumerate() {
        if *s == c {
            return Some(i);
        }
    }
    None
}

#[no_mangle]
pub unsafe extern "C" fn memchr(s: *const char_t, c: int_t, n: size_t) -> *const char_t {
    match iter_chr(MemIter::from_ptr(s, LenBound { len: n }), c as char_t) {
        Some(offset) => s.add(offset),
        None => null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn strchr(s: *const char_t, c: int_t) -> *const char_t {
    if c == 0 {
        s.add(strlen(s))
    } else {
        match iter_chr(MemIter::from_ptr(s, CStrBound), c as char_t) {
            Some(offset) => s.add(offset),
            None => null(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn strcspn(s1: *const char_t, s2: *const char_t) -> size_t {
    let mut i = 0;
    for c in MemIter::from_ptr(s1, CStrBound) {
        if !strchr(s2, *c as int_t).is_null() {
            return i;
        }
        i += 1;
    }
    i
}

#[no_mangle]
pub unsafe extern "C" fn strpbrk(s1: *const char_t, s2: *const char_t) -> *const char_t {
    for (i, c) in MemIter::from_ptr(s1, CStrBound).enumerate() {
        if !strchr(s2, *c as int_t).is_null() {
            return s1.add(i);
        }
    }
    null()
}

#[no_mangle]
pub unsafe extern "C" fn strrchr(s: *const char_t, c: int_t) -> *const char_t {
    let n = strlen(s);
    match iter_chr(
        MemIter::from_ptr(s, LenBound { len: n }).into_rev(),
        c as char_t,
    ) {
        Some(i) => s.add(n - i - 1),
        None => null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn strspn(s1: *const char_t, s2: *const char_t) -> size_t {
    MemIter::from_ptr(s1, CStrBound)
        .filter(|c| !strchr(s2, **c as int_t).is_null())
        .count()
}

#[no_mangle]
pub unsafe extern "C" fn strstr(s1: *const char_t, s2: *const char_t) -> *const char_t {
    let len1 = strlen(s1);
    let len2 = strlen(s2);
    for i in 0..=(len1 - len2) {
        if memcmp(s1.add(i), s2, len2) == 0 {
            return s1.add(i);
        }
    }
    null()
}

static mut STRTOK_STR: Option<&mut [char_t]> = None;

unsafe fn strtok_impl(s2: *const char_t) -> *const char_t {
    if let Some(mut s1) = STRTOK_STR.take() {
        let res = s1.as_ptr() as *mut _;
        let end_index = strcspn(res, s2);
        if end_index != s1.len() {
            *res.add(end_index) = 0;
            s1 = s1.get_unchecked_mut((end_index + 1)..);
        } else {
            s1 = from_raw_parts_mut(null_mut(), 0);
        }
        if s1.is_empty() {
            STRTOK_STR = None;
        } else {
            STRTOK_STR = Some(s1);
        }
        res
    } else {
        null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn strtok(s1: *mut char_t, s2: *const char_t) -> *const char_t {
    if !s1.is_null() {
        STRTOK_STR = Some(from_raw_parts_mut(s1 as *mut _, strlen(s1)));
    }
    loop {
        let res = strtok_impl(s2);
        if res.is_null() || *res != b'\0' as _ {
            return res;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn memset(dst: *mut char_t, c: int_t, n: size_t) -> *mut char_t {
    mem::memset(dst as _, c, n) as _
}

#[no_mangle]
pub unsafe extern "C" fn strerror(_: int_t) -> *const char_t {
    cs!("u w0t m8?")
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const char_t) -> size_t {
    MemIter::from_ptr(s, CStrBound).count()
}

#[no_mangle]
pub unsafe extern "C" fn strnlen(s: *const char_t, n: size_t) -> size_t {
    MemIter::from_ptr(s, LenBound { len: n }.and(CStrBound)).count()
}
