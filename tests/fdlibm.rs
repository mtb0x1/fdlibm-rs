#![feature(core_intrinsics)]
use std::intrinsics::log2f64;

use fdlibm_rs::*;
pub const ARG1: f64 = 0.055605003447049994_f64;

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
    //eprint!("cbrt {}, vs {}",unsafe{cbrt(-ARG1)},-ARG1.cbrt());
    assert_approx_eq!(unsafe { cbrt(-ARG1) }, -ARG1.cbrt());
}
#[test]
//https://raw.githubusercontent.com/rust-lang/rust/c4ce33cfbc800ed027f4ebd7d2033fcb4d6c6665/src/tools/miri/tests/pass/intrinsics-math.rs
pub fn intrinsics_inspired_test() {
    assert_approx_eq!(64f64.sqrt(), unsafe { sqrt(64f64)});
    assert_approx_eq!(2f64.sqrt(), unsafe { sqrt(2f64)});

    //@todo
    //assert_approx_eq!(23.2f64.powi(2),unsafe {pow(23.2f64, 2.0)});

    //@todo
    //assert_approx_eq!(400f64.powf(0.5f64),unsafe{pow(400f64,0.5f64)} );

    assert_approx_eq!( unsafe{exp(1f64)}, std::f64::consts::E);

    assert_approx_eq!(unsafe{expm1(1f64)}, std::f64::consts::E - 1.0);

    assert_approx_eq!(unsafe{exp(50f64)}, 50f64.exp());
    //@todo
    //assert_approx_eq!(unsafe{log(1f64)}, 0f64);
    assert_approx_eq!(unsafe{log1p(0f64)}, 0f64);

    //@todo
    //assert_approx_eq!(unsafe{log10(std::f64::consts::E)}, std::f64::consts::LOG10_E);

    //@todo, something todo with 
    /*
    assert_approx_eq!(
        unsafe {logb(std::f64::consts::E) },
        std::f64::consts::E.log(2.0) //?LOG2_E ?
    );
    */

    assert_approx_eq!(unsafe { floor(-1.1f64) }, -2.0f64);
/*  
    //@todo
    //assert_approx_eq!(unsafe {ceil(3.8f64)}, 4.0f64);

    assert_approx_eq!(0.1f32.trunc(), 0.0f32);
    assert_approx_eq!((-0.1f64).trunc(), 0.0f64);

    assert_approx_eq!(27.0f32.cbrt(), 3.0f32);
    assert_approx_eq!(27.0f64.cbrt(), 3.0f64);

    assert_approx_eq!(3.0f32.hypot(4.0f32), 5.0f32);
    assert_approx_eq!(3.0f64.hypot(4.0f64), 5.0f64);

    assert_eq!(3.3_f32.round(), 3.0);
    assert_eq!(3.3_f64.round(), 3.0);

    assert_eq!(ldexp(0.65f64, 3i32), 5.2f64);
    assert_eq!(ldexp(1.42, 0xFFFF), f64::INFINITY);
    assert_eq!(ldexp(1.42, -0xFFFF), 0f64);

    // Trigonometric functions.

    assert_approx_eq!(0f32.sin(), 0f32);
    assert_approx_eq!((f64::consts::PI / 2f64).sin(), 1f64);
    assert_approx_eq!(f32::consts::FRAC_PI_6.sin(), 0.5);
    assert_approx_eq!(f64::consts::FRAC_PI_6.sin(), 0.5);
    assert_approx_eq!(f32::consts::FRAC_PI_4.sin().asin(), f32::consts::FRAC_PI_4);
    assert_approx_eq!(f64::consts::FRAC_PI_4.sin().asin(), f64::consts::FRAC_PI_4);

    assert_approx_eq!(1.0f32.sinh(), 1.1752012f32);
    assert_approx_eq!(1.0f64.sinh(), 1.1752012f64);
    assert_approx_eq!(2.0f32.asinh(), 1.443635475178810342493276740273105f32);
    assert_approx_eq!((-2.0f64).asinh(), -1.443635475178810342493276740273105f64);

    assert_approx_eq!(0f32.cos(), 1f32);
    assert_approx_eq!((f64::consts::PI * 2f64).cos(), 1f64);
    assert_approx_eq!(f32::consts::FRAC_PI_3.cos(), 0.5);
    assert_approx_eq!(f64::consts::FRAC_PI_3.cos(), 0.5);
    assert_approx_eq!(f32::consts::FRAC_PI_4.cos().acos(), f32::consts::FRAC_PI_4);
    assert_approx_eq!(f64::consts::FRAC_PI_4.cos().acos(), f64::consts::FRAC_PI_4);

    assert_approx_eq!(1.0f32.cosh(), 1.54308f32);
    assert_approx_eq!(1.0f64.cosh(), 1.54308f64);
    assert_approx_eq!(2.0f32.acosh(), 1.31695789692481670862504634730796844f32);
    assert_approx_eq!(3.0f64.acosh(), 1.76274717403908605046521864995958461f64);

    assert_approx_eq!(1.0f32.tan(), 1.557408f32);
    assert_approx_eq!(1.0f64.tan(), 1.557408f64);
    assert_approx_eq!(1.0_f32, 1.0_f32.tan().atan());
    assert_approx_eq!(1.0_f64, 1.0_f64.tan().atan());
    assert_approx_eq!(1.0f32.atan2(2.0f32), 0.46364761f32);
    assert_approx_eq!(1.0f32.atan2(2.0f32), 0.46364761f32);

    assert_approx_eq!(
        1.0f32.tanh(),
        (1.0 - f32::consts::E.powi(-2)) / (1.0 + f32::consts::E.powi(-2))
    );
    assert_approx_eq!(
        1.0f64.tanh(),
        (1.0 - f64::consts::E.powi(-2)) / (1.0 + f64::consts::E.powi(-2))
    );
    assert_approx_eq!(0.5f32.atanh(), 0.54930614433405484569762261846126285f32);
    assert_approx_eq!(0.5f64.atanh(), 0.54930614433405484569762261846126285f64);

    assert_approx_eq!(5.0f32.gamma(), 24.0);
    assert_approx_eq!(5.0f64.gamma(), 24.0);
    // These fail even on the host, precision seems to be terrible.
    //assert_approx_eq!(-0.5f32.gamma(), -2.0 * f32::consts::PI.sqrt());
    //assert_approx_eq!(-0.5f64.gamma(), -2.0 * f64::consts::PI.sqrt());

    assert_eq!(2.0f32.ln_gamma(), (0.0, 1));
    assert_eq!(2.0f64.ln_gamma(), (0.0, 1));
    // Gamma(-0.5) = -2*sqrt(π)
    let (val, sign) = (-0.5f32).ln_gamma();
    assert_approx_eq!(val, (2.0 * f32::consts::PI.sqrt()).ln());
    assert_eq!(sign, -1);
    let (val, sign) = (-0.5f64).ln_gamma();
    assert_approx_eq!(val, (2.0 * f64::consts::PI.sqrt()).ln());
    assert_eq!(sign, -1);
    */
}
