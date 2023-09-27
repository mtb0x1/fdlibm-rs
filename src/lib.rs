#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod fdlibm;
pub use fdlibm::*;
pub const FDLIBMRS_VERSION: f32 = 1.5;

pub fn acos(arg1: f64) -> f64 {
    unsafe { racos(arg1) }
}
pub fn asin(arg1: f64) -> f64 {
    unsafe { rasin(arg1) }
}
pub fn atan(arg1: f64) -> f64 {
    unsafe { ratan(arg1) }
}
pub fn atan2(arg1: f64,arg2: f64) -> f64 {
    unsafe { ratan2(arg1,arg2) }
}
pub fn cos(arg1: f64) -> f64 {
    unsafe { rcos(arg1) }
}
pub fn exp(arg1: f64) -> f64 {
    unsafe { rexp(arg1) }
}
pub fn log(arg1: f64) -> f64 {
    unsafe { rlog(arg1) }
}
pub fn log10(arg1: f64) -> f64 {
    unsafe { rlog10(arg1) }
}
pub fn pow(arg1: f64, arg2: f64) -> f64 {
    unsafe { rpow(arg1, arg2) }
}
pub fn sin(arg1: f64) -> f64 {
    unsafe { rsin(arg1) }
}
pub fn sqrt(arg1: f64) -> f64 {
    unsafe { rsqrt(arg1) }
}
pub fn cbrt(arg1: f64) -> f64 {
    unsafe { rcbrt(arg1) }
}
pub fn tan(arg1: f64) -> f64 {
    unsafe { rtan(arg1) }
}
pub fn floor(arg1: f64) -> f64 {
    unsafe { rfloor(arg1) }
}
pub fn ceil(arg1: f64) -> f64 {
    unsafe { rceil(arg1) }
}
pub fn cosh(arg1: f64) -> f64 {
    unsafe { rcosh(arg1) }
}
pub fn r#mod(arg1: f64, arg2: f64) -> f64 {
    unsafe { rmod(arg1, arg2) }
}
pub fn sinh(arg1: f64) -> f64 {
    unsafe { rsinh(arg1) }
}
pub fn fabs(arg1: f64) -> f64 {
    unsafe { rfabs(arg1) }
}
pub fn tanh(arg1: f64) -> f64 {
    unsafe { rtanh(arg1) }
}
pub fn remainder(arg1: f64, arg2: f64) -> f64 {
    unsafe { rremainder(arg1, arg2) }
}
pub fn hypot(arg1: f64, arg2: f64) -> f64 {
    unsafe { rhypot(arg1, arg2) }
}
pub fn log1p(arg1: f64) -> f64 {
    unsafe { rlog1p(arg1) }
}
pub fn expm1(arg1: f64) -> f64 {
    unsafe { rexpm1(arg1) }
}

pub fn __ieee754_sqrt(arg1: f64) -> f64 {
    unsafe { __r__ieee754_sqrt(arg1) }
}
pub fn __ieee754_acos(arg1: f64) -> f64 {
    unsafe { __r__ieee754_acos(arg1) }
}
pub fn __ieee754_log(arg1: f64) -> f64 {
    unsafe { __r__ieee754_log(arg1) }
}
pub fn __ieee754_atanh(arg1: f64) -> f64 {
    unsafe { __r__ieee754_atanh(arg1) }
}
pub fn __ieee754_asin(arg1: f64) -> f64 {
    unsafe { __r__ieee754_asin(arg1) }
}
pub fn __ieee754_atan2(arg1: f64, arg2: f64) -> f64 {
    unsafe { __r__ieee754_atan2(arg1, arg2) }
}
pub fn __ieee754_exp(arg1: f64) -> f64 {
    unsafe { __r__ieee754_exp(arg1) }
}
pub fn __ieee754_cosh(arg1: f64) -> f64 {
    unsafe { __r__ieee754_cosh(arg1) }
}
pub fn __ieee754_fmod(arg1: f64, arg2: f64) -> f64 {
    unsafe { __r__ieee754_fmod(arg1, arg2) }
}
pub fn __ieee754_pow(arg1: f64, arg2: f64) -> f64 {
    unsafe { __r__ieee754_pow(arg1, arg2) }
}
pub fn __ieee754_log10(arg1: f64) -> f64 {
    unsafe { __r__ieee754_log10(arg1) }
}
pub fn __ieee754_sinh(arg1: f64) -> f64 {
    unsafe { __r__ieee754_sinh(arg1) }
}
pub fn __ieee754_hypot(arg1: f64, arg2: f64) -> f64 {
    unsafe { __r__ieee754_hypot(arg1, arg2) }
}
pub fn __ieee754_remainder(arg1: f64, arg2: f64) -> f64 {
    unsafe { __r__ieee754_remainder(arg1, arg2) }
}
pub fn __ieee754_rem_pio2(arg1: f64, arg2: *mut f64) -> ::std::os::raw::c_int {
    unsafe { __r__ieee754_rem_pio2(arg1, arg2) }
}
pub fn __ieee754_scalb(arg1: f64, arg2: f64) -> f64 {
    unsafe { __r__ieee754_scalb(arg1, arg2) }
}
pub fn __kernel_standard(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64 {
    unsafe { __r__kernel_standard(arg1, arg2, arg3) }
}
pub fn __kernel_sin(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64 {
    unsafe { __r__kernel_sin(arg1, arg2, arg3) }
}
pub fn __kernel_cos(arg1: f64, arg2: f64) -> f64 {
    unsafe { __r__kernel_cos(arg1, arg2) }
}
pub fn __kernel_tan(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64 {
    unsafe { __r__kernel_tan(arg1, arg2, arg3) }
}
pub fn __kernel_rem_pio2(
    arg1: *mut f64,
    arg2: *mut f64,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
    arg5: ::std::os::raw::c_int,
    arg6: *const ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { __r__kernel_rem_pio2(arg1, arg2, arg3, arg4, arg5, arg6) }
}
pub fn __ieee754_log1p(arg1: f64) -> f64 {
    unsafe { rlog1p(arg1) }
}
pub fn __ieee754_expm1(arg1: f64) -> f64 {
    unsafe { rexpm1(arg1) }
}
