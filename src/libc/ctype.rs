use crate::types::int_t;

#[no_mangle]
pub extern "C" fn isalnum(c: int_t) -> int_t {
    match c as u8 as char {
        'a'..='z' => 1,
        'A'..='Z' => 1,
        '0'..='9' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isalpha(c: int_t) -> int_t {
    match c as u8 as char {
        'a'..='z' => 1,
        'A'..='Z' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isblank(c: int_t) -> int_t {
    match c as u8 as char {
        ' ' | '\t' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn iscntrl(c: int_t) -> int_t {
    match c as u8 as char {
        '\x00'..='\x19' => 1,
        '\x7f' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isdigit(c: int_t) -> int_t {
    match c as u8 as char {
        '0'..='9' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isgraph(c: int_t) -> int_t {
    match c {
        0x21..=0x7e => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn islower(c: int_t) -> int_t {
    match c as u8 as char {
        'a'..='z' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isprint(c: int_t) -> int_t {
    match c {
        0x20..=0x7e => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn ispunct(c: int_t) -> int_t {
    ((isgraph(c) != 0) && (isalpha(c) == 0)) as int_t
}

#[no_mangle]
pub extern "C" fn isspace(c: int_t) -> int_t {
    match c as u8 as char {
        ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isupper(c: int_t) -> int_t {
    match c as u8 as char {
        'A'..='Z' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn isxdigit(c: int_t) -> int_t {
    match c as u8 as char {
        '0'..='9' => 1,
        'A'..='F' => 1,
        'a'..='f' => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn tolower(c: int_t) -> int_t {
    match c as u8 as char {
        'A'..='Z' => c + 0x20,
        _ => c,
    }
}

#[no_mangle]
pub extern "C" fn toupper(c: int_t) -> int_t {
    match c as u8 as char {
        'a'..='z' => c - 0x20,
        _ => c,
    }
}
