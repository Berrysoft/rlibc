use crate::syscalls::sys_gettimeofday;
use crate::types::{char_t, int_t, long_t, time_t, timeval, timezone, tm};
use core::mem::MaybeUninit;
use core::ptr::null_mut;

#[no_mangle]
pub unsafe extern "C" fn time(t: *mut time_t) -> time_t {
    let mut now: timeval = MaybeUninit::uninit().assume_init();
    if gettimeofday(&mut now, null_mut()) >= 0 {
        if !t.is_null() {
            *t = now.tv_sec;
        }
        now.tv_sec
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> int_t {
    forward!(sys_gettimeofday, tv, tz) as _
}

#[thread_local]
pub static mut GMTIME_TM: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: null_mut(),
};

#[thread_local]
pub static mut LOCALTIME_TM: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: null_mut(),
};

static TM_ZONE_GMT: &[u8] = b"GMT\0";

const EPOCH_YR: int_t = 1970;
const YR_1900: int_t = 1900;
const SECS_DAY: u64 = 86400;

/// Length of the passed gregorian year (e.g. 1970).
const fn yearsize(year: int_t) -> u64 {
    match year {
        // leap years on non-centennials
        y if (y % 4 == 0 && y % 100 != 0) => 366,
        // leap years on centennials not multiples of 400
        y if (y % 4 == 0 && y % 100 == 0 && y % 400 != 0) => 365,
        // leap years on multiples of 400
        y if (y % 4 == 0 && y % 100 == 0 && y % 400 == 0) => 366,
        // normal non-leap years. This doesn't exhaust for some reason:
        // y if (y % 4 != 0) => 365,
        _ => 365,
    }
}

const fn leapyear(year: int_t) -> bool {
    yearsize(year) == 366
}

const fn monthlen(ly: bool, mon: int_t) -> u64 {
    let _ytab = &[
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    ];
    _ytab[ly as usize][mon as usize]
}

#[no_mangle]
pub unsafe extern "C" fn gmtime(timer: *const time_t) -> *mut tm {
    gmtime_r(timer, &mut GMTIME_TM)
}

/// TODO negative times
#[no_mangle]
pub unsafe extern "C" fn gmtime_r(timer: *const time_t, buf: *mut tm) -> *mut tm {
    let time = *timer;
    let dayclock: u64 = time as u64 % SECS_DAY;
    let mut dayno: u64 = time as u64 / SECS_DAY;
    let mut year: int_t = EPOCH_YR;

    *buf = tm {
        tm_sec: (dayclock % 60) as int_t,
        tm_min: ((dayclock % 3600) / 60) as int_t,
        tm_hour: (dayclock / 3600) as int_t,
        tm_wday: ((dayno + 4) % 7) as int_t, // day 0 was a thursday

        tm_year: {
            while dayno >= yearsize(year) {
                dayno -= yearsize(year);
                year += 1;
            }
            year - YR_1900
        },

        tm_yday: dayno as int_t,
        tm_mon: {
            let mut mon = 0;
            while dayno >= monthlen(leapyear(year), mon) {
                dayno -= monthlen(leapyear(year), mon);
                mon += 1;
            }
            mon
        },

        tm_mday: dayno as int_t + 1,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: TM_ZONE_GMT.as_ptr() as *mut char_t,
    };

    buf
}

#[no_mangle]
pub unsafe extern "C" fn localtime(timer: *const time_t) -> *mut tm {
    localtime_r(timer, &mut LOCALTIME_TM)
}

/// TODO time localization
#[no_mangle]
pub unsafe extern "C" fn localtime_r(timer: *const time_t, buf: *mut tm) -> *mut tm {
    gmtime_r(timer, buf)
}

/// Convert a GMT `struct tm` to a time_t.
#[no_mangle]
pub unsafe extern "C" fn timegm(timer_ptr: *const tm) -> time_t {
    let timer = &*timer_ptr;
    let yr = timer.tm_year + EPOCH_YR;
    let mut t = (yr as time_t - 1970) * (yearsize(yr) * SECS_DAY) as time_t;
    t += timer.tm_yday as time_t * SECS_DAY as time_t;
    t += (timer.tm_hour * 3600 + timer.tm_min * 60 + timer.tm_sec) as time_t;
    t
}

/// TODO time localization
#[no_mangle]
pub unsafe extern "C" fn mktime(timer_ptr: *const tm) -> time_t {
    timegm(timer_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn timelocal(timer_ptr: *const tm) -> time_t {
    mktime(timer_ptr)
}

#[no_mangle]
#[thread_local]
pub static mut tzname: [*mut char_t; 2] = [null_mut(); 2];
#[no_mangle]
#[thread_local]
pub static mut timezone: long_t = 0;
#[no_mangle]
#[thread_local]
pub static mut daylight: int_t = 0;

/// TODO time localization
#[no_mangle]
pub unsafe extern "C" fn tzset() {
    tzname[0] = TM_ZONE_GMT.as_ptr() as *mut char_t;
    tzname[1] = TM_ZONE_GMT.as_ptr() as *mut char_t;
}
