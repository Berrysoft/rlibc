use crate::types::{int_t, wint_t};
use core::char::from_u32_unchecked;

#[inline]
const fn bool_to_int(b: bool) -> int_t {
    if b {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn isalnum(c: int_t) -> int_t {
    iswalnum(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswalnum(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_alphanumeric())
}

#[no_mangle]
pub unsafe extern "C" fn isalpha(c: int_t) -> int_t {
    iswalpha(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswalpha(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_alphabetic())
}

#[no_mangle]
pub unsafe extern "C" fn isblank(c: int_t) -> int_t {
    iswblank(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswblank(c: wint_t) -> int_t {
    match from_u32_unchecked(c) {
        ' ' | '\t' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn iscntrl(c: int_t) -> int_t {
    iswcntrl(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswcntrl(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_control())
}

#[no_mangle]
pub unsafe extern "C" fn isdigit(c: int_t) -> int_t {
    iswdigit(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswdigit(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_ascii_digit())
}

#[no_mangle]
pub unsafe extern "C" fn isgraph(c: int_t) -> int_t {
    iswgraph(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswgraph(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_ascii_graphic())
}

#[no_mangle]
pub unsafe extern "C" fn islower(c: int_t) -> int_t {
    iswlower(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswlower(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_lowercase())
}

#[no_mangle]
pub unsafe extern "C" fn isprint(c: int_t) -> int_t {
    iswprint(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswprint(c: wint_t) -> int_t {
    let c = from_u32_unchecked(c);
    bool_to_int(c == ' ' || c.is_ascii_graphic())
}

#[no_mangle]
pub unsafe extern "C" fn ispunct(c: int_t) -> int_t {
    iswpunct(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswpunct(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_ascii_punctuation())
}

#[no_mangle]
pub unsafe extern "C" fn isspace(c: int_t) -> int_t {
    iswspace(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswspace(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_whitespace())
}

#[no_mangle]
pub unsafe extern "C" fn isupper(c: int_t) -> int_t {
    iswupper(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswupper(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_uppercase())
}

#[no_mangle]
pub unsafe extern "C" fn isxdigit(c: int_t) -> int_t {
    iswxdigit(c as u32)
}

#[no_mangle]
pub unsafe extern "C" fn iswxdigit(c: wint_t) -> int_t {
    bool_to_int(from_u32_unchecked(c).is_ascii_hexdigit())
}

#[no_mangle]
pub unsafe extern "C" fn tolower(c: int_t) -> int_t {
    towlower(c as u32) as _
}

#[no_mangle]
pub unsafe extern "C" fn towlower(c: wint_t) -> wint_t {
    let mut lower = from_u32_unchecked(c).to_lowercase();
    match lower.next() {
        Some(c) => c as _,
        None => c,
    }
}

#[no_mangle]
pub unsafe extern "C" fn toupper(c: int_t) -> int_t {
    towupper(c as u32) as _
}

#[no_mangle]
pub unsafe extern "C" fn towupper(c: wint_t) -> wint_t {
    let mut upper = from_u32_unchecked(c).to_uppercase();
    match upper.next() {
        Some(c) => c as _,
        None => c,
    }
}
