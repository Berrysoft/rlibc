macro_rules! no_mangle {
    ($(fn $fun:ident($($iid:ident : $ity:ty),+) -> $oty:ty;)+) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $fun($($iid: $ity),+) -> $oty {
                libm::$fun($($iid),+)
            }
        )+
    }
}

no_mangle! {
    fn acos(x: f64) -> f64;
    fn acosf(n: f32) -> f32;
    fn acosh(x: f64) -> f64;
    fn acoshf(n: f32) -> f32;

    fn asin(x: f64) -> f64;
    fn asinf(n: f32) -> f32;
    fn asinh(x: f64) -> f64;
    fn asinhf(n: f32) -> f32;

    fn atan(x: f64) -> f64;
    fn atan2(x: f64, y: f64) -> f64;
    fn atan2f(a: f32, b: f32) -> f32;
    fn atanf(n: f32) -> f32;
    fn atanh(x: f64) -> f64;
    fn atanhf(a: f32) -> f32;

    fn cbrt(x: f64) -> f64;
    fn cbrtf(n: f32) -> f32;

    fn ceil(x: f64) -> f64;
    fn ceilf(n: f32) -> f32;

    fn copysign(x: f64, y: f64) -> f64;
    fn copysignf(x: f32, y: f32) -> f32;

    fn cos(x: f64) -> f64;
    fn cosf(x: f32) -> f32;
    fn cosh(x: f64) -> f64;
    fn coshf(n: f32) -> f32;

    fn erf(x: f64) -> f64;
    fn erfc(x: f64) -> f64;
    fn erfcf(x: f32) -> f32;
    fn erff(x: f32) -> f32;

    fn exp(x: f64) -> f64;
    fn exp2(x: f64) -> f64;
    fn exp2f(x: f32) -> f32;
    fn exp10(x: f64) -> f64;
    fn exp10f(x: f32) -> f32;
    fn expf(x: f32) -> f32;
    fn expm1(x: f64) -> f64;
    fn expm1f(n: f32) -> f32;

    fn fabs(x: f64) -> f64;
    fn fabsf(x: f32) -> f32;

    fn fdim(a: f64, b: f64) -> f64;
    fn fdimf(a: f32, b: f32) -> f32;

    fn floor(x: f64) -> f64;
    fn floorf(x: f32) -> f32;

    fn fma(x: f64, y: f64, z: f64) -> f64;
    fn fmaf(x: f32, y: f32, z: f32) -> f32;

    fn fmax(x: f64, y: f64) -> f64;
    fn fmaxf(x: f32, y: f32) -> f32;

    fn fmin(x: f64, y: f64) -> f64;
    fn fminf(x: f32, y: f32) -> f32;

    fn fmod(x: f64, y: f64) -> f64;
    fn fmodf(x: f32, y: f32) -> f32;

    fn hypot(x: f64, y: f64) -> f64;
    fn hypotf(x: f32, y: f32) -> f32;

    fn ilogb(x: f64) -> i32;
    fn ilogbf(x: f32) -> i32;

    fn j0(x: f64) -> f64;
    fn j0f(x: f32) -> f32;
    fn j1(x: f64) -> f64;
    fn j1f(x: f32) -> f32;
    fn jn(n: i32, x: f64) -> f64;
    fn jnf(n: i32, x: f32) -> f32;

    fn ldexp(f: f64, n: i32) -> f64;
    fn ldexpf(f: f32, n: i32) -> f32;

    fn lgamma(x: f64) -> f64;
    fn lgammaf(x: f32) -> f32;

    fn log(x: f64) -> f64;
    fn log1p(x: f64) -> f64;
    fn log1pf(n: f32) -> f32;
    fn log2(x: f64) -> f64;
    fn log2f(x: f32) -> f32;
    fn log10(x: f64) -> f64;
    fn log10f(x: f32) -> f32;
    fn logf(x: f32) -> f32;

    fn nextafter(x: f64, y: f64) -> f64;
    fn nextafterf(x: f32, y: f32) -> f32;

    fn pow(x: f64, y: f64) -> f64;
    fn powf(x: f32, y: f32) -> f32;

    fn remainder(x: f64, y: f64) -> f64;
    fn remainderf(x: f32, y: f32) -> f32;

    fn round(x: f64) -> f64;
    fn roundf(x: f32) -> f32;

    fn scalbn(x: f64, n: i32) -> f64;
    fn scalbnf(x: f32, n: i32) -> f32;

    fn sin(x: f64) -> f64;
    fn sinf(x: f32) -> f32;
    fn sinh(x: f64) -> f64;
    fn sinhf(n: f32) -> f32;

    fn sqrt(x: f64) -> f64;
    fn sqrtf(x: f32) -> f32;

    fn tan(x: f64) -> f64;
    fn tanf(n: f32) -> f32;
    fn tanh(x: f64) -> f64;
    fn tanhf(n: f32) -> f32;

    fn tgamma(x: f64) -> f64;
    fn tgammaf(x: f32) -> f32;

    fn trunc(x: f64) -> f64;
    fn truncf(x: f32) -> f32;

    fn y0(x: f64) -> f64;
    fn y0f(x: f32) -> f32;
    fn y1(x: f64) -> f64;
    fn y1f(x: f32) -> f32;
    fn yn(n: i32, x: f64) -> f64;
    fn ynf(n: i32, x: f32) -> f32;
}

macro_rules! no_mangle_r {
    ($(fn $fun:ident($($iid:ident : $ity:ty),+) -> ($oty:ty, $rty:ty);)+) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $fun($($iid: $ity),+,pp: *mut $rty) -> $oty {
                let (r, p) = libm::$fun($($iid),+);
                *pp = p;
                r
            }
        )+
    }
}

no_mangle_r! {
    fn frexp(x: f64) -> (f64, i32);
    fn frexpf(x: f32) -> (f32, i32);

    fn lgamma_r(x: f64) -> (f64, i32);
    fn lgammaf_r(x: f32) -> (f32, i32);

    fn modf(x: f64) -> (f64, f64);
    fn modff(x: f32) -> (f32, f32);

    fn remquo(x: f64, y: f64) -> (f64, i32);
    fn remquof(x: f32, y: f32) -> (f32, i32);
}

#[no_mangle]
pub unsafe extern "C" fn sincos(x: f64, ps: *mut f64, pc: *mut f64) {
    let (s, c) = libm::sincos(x);
    *ps = s;
    *pc = c;
}

#[no_mangle]
pub unsafe extern "C" fn sincosf(x: f32, ps: *mut f32, pc: *mut f32) {
    let (s, c) = libm::sincosf(x);
    *ps = s;
    *pc = c;
}
