#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod fdlibm;
pub use fdlibm::*;

pub const FDLIBMRS_VERSION: f32 = 1.5;

pub unsafe fn acos(arg1: f64) -> f64 {
    racos(arg1)
}

pub unsafe fn asin(arg1: f64) -> f64 {
    rasin(arg1)
}

pub unsafe fn atan(arg1: f64) -> f64 {
    ratan(arg1)
}

pub unsafe fn atan2(arg1: f64, arg2: f64) -> f64 {
    ratan2(arg1, arg2)
}

pub unsafe fn cos(arg1: f64) -> f64 {
    rcos(arg1)
}

pub unsafe fn exp(arg1: f64) -> f64 {
    rexp(arg1)
}

pub unsafe fn log(arg1: f64) -> f64 {
    rlog(arg1)
}

pub unsafe fn log10(arg1: f64) -> f64 {
    rlog10(arg1)
}

pub unsafe fn pow(arg1: f64, arg2: f64) -> f64 {
    rpow(arg1, arg2)
}

pub unsafe fn sin(arg1: f64) -> f64 {
    rsin(arg1)
}

pub unsafe fn sqrt(arg1: f64) -> f64 {
    rsqrt(arg1)
}

pub unsafe fn cbrt(arg1: f64) -> f64 {
    rcbrt(arg1)
}

pub unsafe fn tan(arg1: f64) -> f64 {
    rtan(arg1)
}

pub unsafe fn floor(arg1: f64) -> f64 {
    rfloor(arg1)
}
pub unsafe fn ceil(arg1: f64) -> f64 {
    rceil(arg1)
}

pub unsafe fn cosh(arg1: f64) -> f64 {
    rcosh(arg1)
}

pub unsafe fn r#mod(arg1: f64, arg2: f64) -> f64 {
    rmod(arg1, arg2)
}

pub unsafe fn sinh(arg1: f64) -> f64 {
    rsinh(arg1)
}

pub unsafe fn fabs(arg1: f64) -> f64 {
    rfabs(arg1)
}

pub unsafe fn tanh(arg1: f64) -> f64 {
    rtanh(arg1)
}

pub unsafe fn remainder(arg1: f64, arg2: f64) -> f64 {
    rremainder(arg1, arg2)
}

pub unsafe fn hypot(arg1: f64, arg2: f64) -> f64 {
    rhypot(arg1, arg2)
}

pub unsafe fn log1p(arg1: f64) -> f64 {
    rlog1p(arg1)
}

pub unsafe fn expm1(arg1: f64) -> f64 {
    rexpm1(arg1)
}

pub unsafe fn __ieee754_sqrt(arg1: f64) -> f64 {
    __r__ieee754_sqrt(arg1)
}

pub unsafe fn __ieee754_acos(arg1: f64) -> f64 {
    __r__ieee754_acos(arg1)
}

pub unsafe fn __ieee754_log(arg1: f64) -> f64 {
    __r__ieee754_log(arg1)
}

pub unsafe fn __ieee754_atanh(arg1: f64) -> f64 {
    __r__ieee754_atanh(arg1)
}

pub unsafe fn __ieee754_asin(arg1: f64) -> f64 {
    __r__ieee754_asin(arg1)
}

pub unsafe fn __ieee754_atan2(arg1: f64, arg2: f64) -> f64 {
    __r__ieee754_atan2(arg1, arg2)
}

pub unsafe fn __ieee754_exp(arg1: f64) -> f64 {
    __r__ieee754_exp(arg1)
}

pub unsafe fn __ieee754_cosh(arg1: f64) -> f64 {
    __r__ieee754_cosh(arg1)
}

pub unsafe fn __ieee754_fmod(arg1: f64, arg2: f64) -> f64 {
    __r__ieee754_fmod(arg1, arg2)
}

pub unsafe fn __ieee754_pow(arg1: f64, arg2: f64) -> f64 {
    __r__ieee754_pow(arg1, arg2)
}

pub unsafe fn __ieee754_log10(arg1: f64) -> f64 {
    __r__ieee754_log10(arg1)
}

pub unsafe fn __ieee754_sinh(arg1: f64) -> f64 {
    __r__ieee754_sinh(arg1)
}

pub unsafe fn __ieee754_hypot(arg1: f64, arg2: f64) -> f64 {
    __r__ieee754_hypot(arg1, arg2)
}

pub unsafe fn __ieee754_remainder(arg1: f64, arg2: f64) -> f64 {
    __r__ieee754_remainder(arg1, arg2)
}

pub unsafe fn __ieee754_rem_pio2(arg1: f64, arg2: *mut f64) -> ::std::os::raw::c_int {
    __r__ieee754_rem_pio2(arg1, arg2)
}

pub unsafe fn __ieee754_scalb(arg1: f64, arg2: f64) -> f64 {
    __r__ieee754_scalb(arg1, arg2)
}

pub unsafe fn __kernel_standard(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64 {
    __r__kernel_standard(arg1, arg2, arg3)
}

pub unsafe fn __kernel_sin(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64 {
    __r__kernel_sin(arg1, arg2, arg3)
}

pub unsafe fn __kernel_cos(arg1: f64, arg2: f64) -> f64 {
    __r__kernel_cos(arg1, arg2)
}

pub unsafe fn __kernel_tan(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64 {
    __r__kernel_tan(arg1, arg2, arg3)
}

pub unsafe fn __kernel_rem_pio2(
    arg1: *mut f64,
    arg2: *mut f64,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
    arg5: ::std::os::raw::c_int,
    arg6: *const ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    __r__kernel_rem_pio2(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn __ieee754_log1p(arg1: f64) -> f64 {
    rlog1p(arg1)
}
