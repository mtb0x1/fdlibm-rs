#![feature(core_intrinsics)]
#![feature(float_gamma)]

use fdlibm_rs::*;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-6,
            "{} is not approximately equal to {}",
            *a,
            *b
        );
    }};
}

#[test]
pub fn acos_test() {
    assert_eq!(unsafe { acos(ARG1) }, ARG1.acos());
}
#[test]
pub fn asin_test() {
    assert_eq!(unsafe { asin(ARG1) }, ARG1.asin());
}
#[test]
pub fn atan_test() {
    assert_eq!(unsafe { atan(ARG1) }, ARG1.atan());
}
#[test]
pub fn atan2_test() {
    assert_eq!(unsafe { atan2(3.0_f64, -3.0_f64) }, 3.0_f64.atan2(-3.0_f64));
}
#[test]
pub fn cosh_test() {
    assert_eq!(unsafe { cosh(ARG1) }, ARG1.cosh());
}
#[test]
pub fn expm1_test() {
    assert_eq!(unsafe { expm1(ARG1) }, ARG1.exp_m1());
}
#[test]
pub fn hypot_test() {
    assert_eq!(unsafe { hypot(2.0_f64, 3.0_f64) }, 2.0_f64.hypot(3.0_f64));
}
#[test]
pub fn log1p_test() {
    assert_eq!(unsafe { log1p(ARG1) }, ARG1.ln_1p());
}
#[test]
pub fn sinh_test() {
    assert_eq!(unsafe { sinh(ARG1) }, ARG1.sinh());
}
#[test]
pub fn tan_test() {
    assert_eq!(unsafe { tan(ARG1) }, ARG1.tan());
}
#[test]
pub fn tanh_test() {
    assert_eq!(unsafe { tanh(ARG1) }, ARG1.tanh());
}
#[test]
pub fn cbrt_test() {
    assert_approx_eq!(unsafe { cbrt(-ARG1) }, -ARG1.cbrt());
}
#[test]
//https://raw.githubusercontent.com/rust-lang/rust/c4ce33cfbc800ed027f4ebd7d2033fcb4d6c6665/src/tools/miri/tests/pass/intrinsics-math.rs
pub fn intrinsics_inspired_test() {
    assert_approx_eq!(64f64.sqrt(), unsafe { sqrt(64f64) });

    assert_approx_eq!(2f64.sqrt(), unsafe { sqrt(2f64) });

    assert_approx_eq!(23.2f64.powi(2), unsafe { pow(23.2f64, 2.0) });

    assert_approx_eq!(400f64.powf(0.5f64), unsafe { pow(400f64, 0.5f64) });

    assert_approx_eq!(unsafe { exp(1f64) }, core::f64::consts::E);

    assert_approx_eq!(unsafe { expm1(1f64) }, core::f64::consts::E - 1.0);

    assert_approx_eq!(unsafe { exp(50f64) }, 50f64.exp());

    assert_approx_eq!(unsafe { log(1f64) }, 0f64);

    assert_approx_eq!(unsafe { log1p(0f64) }, 0f64);

    assert_approx_eq!(
        unsafe { log10(core::f64::consts::E) },
        core::f64::consts::LOG10_E
    );

    assert_approx_eq!(unsafe { floor(-1.1f64) }, -2.0f64);

    assert_approx_eq!(unsafe { ceil(3.8f64) }, 4.0f64);

    assert_approx_eq!(unsafe { cbrt(27.0f64) }, 3.0f64);

    assert_approx_eq!(unsafe { hypot(3.0f64, 4.0f64) }, 5.0f64);  
   
    assert_eq!(unsafe {ldexp(0.65f64, 3i32)}, 5.2f64);

    assert_eq!(unsafe {ldexp(1.42, 0xFFFF)}, core::f64::INFINITY);
    
    assert_eq!(unsafe {ldexp(1.42, -0xFFFF)}, 0f64);

    assert_approx_eq!(unsafe{sin(0f64)}, 0f64);
    
    assert_approx_eq!(unsafe {sin(core::f64::consts::PI / 2f64)}, 1f64);

    assert_approx_eq!(unsafe {sin(core::f64::consts::FRAC_PI_6)}, 0.5);
    
    assert_approx_eq!(unsafe{ asin(sin(core::f64::consts::FRAC_PI_4))} , core::f64::consts::FRAC_PI_4);
    
    assert_approx_eq!(unsafe {sinh(1.0f64)}, 1.1752012f64);

    assert_approx_eq!(unsafe{asinh(-2.0f64)}, -1.443635475178810342493276740273105f64);

    assert_approx_eq!(unsafe {cos(core::f64::consts::PI * 2f64)}, 1f64);
 
    assert_approx_eq!(unsafe { cos(core::f64::consts::PI * 2f64)}, 1f64);
 
    assert_approx_eq!(unsafe {cos(core::f64::consts::FRAC_PI_3)}, 0.5);
 
    assert_approx_eq!(unsafe {acos(cos(core::f64::consts::FRAC_PI_4))}, core::f64::consts::FRAC_PI_4);
 
    assert_approx_eq!(unsafe {cosh(1.0f64)}, 1.54308f64);
 
    assert_approx_eq!(unsafe {acosh(3.0f64)}, 1.76274717403908605046521864995958461f64);
 
    assert_approx_eq!(unsafe {tan(1.0f64)}, 1.557408f64);
 
    assert_approx_eq!(unsafe {atan(tan(1.0_f64))},1.0_f64);
 
    assert_approx_eq!(unsafe {atan2(1.0f64,2.0f64)}, 0.46364761f64);

    assert_approx_eq!(
        unsafe { tanh(1.0f64)},
        (1.0 - core::f64::consts::E.powi(-2)) / (1.0 + core::f64::consts::E.powi(-2))
    );
 
    assert_approx_eq!(unsafe {atanh(0.5f64)}, 0.54930614433405484569762261846126285f64);

    //@todo ?
    //assert_approx_eq!(unsafe {gamma(5.0f64)}, 5.0f64.gamma());
}
