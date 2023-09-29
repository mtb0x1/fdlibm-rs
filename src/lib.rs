#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod fdlibm;
pub use fdlibm::*;

pub const FDLIBMRS_VERSION: core::ffi::c_float = 0.1;

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn acos(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    racos(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn asin(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rasin(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn atan(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    ratan(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn atan2(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    ratan2(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn cos(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rcos(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn exp(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rexp(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn log(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rlog(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn log10(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rlog10(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn pow(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    rpow(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn sin(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rsin(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn sqrt(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rsqrt(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn cbrt(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rcbrt(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn tan(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rtan(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn floor(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rfloor(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn ceil(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rceil(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn cosh(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rcosh(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn r#mod(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    rmod(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn sinh(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rsinh(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn fabs(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rfabs(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn tanh(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rtanh(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn remainder(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    rremainder(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn hypot(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    rhypot(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn log1p(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rlog1p(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn expm1(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    rexpm1(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_sqrt(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_sqrt(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_acos(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_acos(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_log(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_log(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_atanh(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_atanh(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_asin(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_asin(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_atan2(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    __r__ieee754_atan2(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_exp(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_exp(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_cosh(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_cosh(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_fmod(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    __r__ieee754_fmod(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_pow(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    __r__ieee754_pow(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_log10(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_log10(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_sinh(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    __r__ieee754_sinh(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_hypot(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    __r__ieee754_hypot(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_remainder(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    __r__ieee754_remainder(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_rem_pio2(
    arg1: ::core::ffi::c_double,
    arg2: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    __r__ieee754_rem_pio2(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_scalb(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    __r__ieee754_scalb(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __kernel_standard(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
    arg3: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    __r__kernel_standard(arg1, arg2, arg3)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __kernel_sin(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
    arg3: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    __r__kernel_sin(arg1, arg2, arg3)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __kernel_cos(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    __r__kernel_cos(arg1, arg2)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __kernel_tan(
    arg1: ::core::ffi::c_double,
    arg2: ::core::ffi::c_double,
    arg3: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    __r__kernel_tan(arg1, arg2, arg3)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __kernel_rem_pio2(
    arg1: *mut ::core::ffi::c_double,
    arg2: *mut ::core::ffi::c_double,
    arg3: ::core::ffi::c_int,
    arg4: ::core::ffi::c_int,
    arg5: ::core::ffi::c_int,
    arg6: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    __r__kernel_rem_pio2(arg1, arg2, arg3, arg4, arg5, arg6)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_log1p(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    log1p(arg1)
}

/// # Safety
///
/// only works in _IEEE_LIBM mode
pub unsafe fn __ieee754_expm1(arg1: ::core::ffi::c_double) -> ::core::ffi::c_double {
    expm1(arg1)
}
