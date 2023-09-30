const FLAGS_NAMES: &[&str; 5] = &[
    "_IEEE_LIBM",
    "_IEEE_MODE",
    "_XOPEN_MODE",
    "_POSIX_MODE",
    "_SVID3_MODE",
];

fn bindgen_symbols(path: &str, _flags: &std::collections::HashSet<String>) {
    //@todo
    //can we pass the flags to bindgen ?
    let bindings = bindgen::Builder::default()
        .header("fdlibm/fdlibm.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .no_convert_floats()
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(path)
        .expect("Couldn't write bindings!");
}

fn build_bundled(flags: &std::collections::HashSet<String>) {
    let mut cfg = cc::Build::new();
    cfg.include("fdlibm")
        .file(std::path::Path::new("fdlibm/e_acos.c"))
        .file(std::path::Path::new("fdlibm/e_acosh.c"))
        .file(std::path::Path::new("fdlibm/e_asin.c"))
        .file(std::path::Path::new("fdlibm/e_atan2.c"))
        .file(std::path::Path::new("fdlibm/e_atanh.c"))
        .file(std::path::Path::new("fdlibm/e_cosh.c"))
        .file(std::path::Path::new("fdlibm/e_exp.c"))
        .file(std::path::Path::new("fdlibm/e_fmod.c"))
        .file(std::path::Path::new("fdlibm/e_gamma_r.c"))
        .file(std::path::Path::new("fdlibm/e_gamma.c"))
        .file(std::path::Path::new("fdlibm/e_hypot.c"))
        .file(std::path::Path::new("fdlibm/e_j0.c"))
        .file(std::path::Path::new("fdlibm/e_j1.c"))
        .file(std::path::Path::new("fdlibm/e_jn.c"))
        .file(std::path::Path::new("fdlibm/e_lgamma_r.c"))
        .file(std::path::Path::new("fdlibm/e_lgamma.c"))
        .file(std::path::Path::new("fdlibm/e_log.c"))
        .file(std::path::Path::new("fdlibm/e_log10.c"))
        .file(std::path::Path::new("fdlibm/e_pow.c"))
        .file(std::path::Path::new("fdlibm/e_rem_pio2.c"))
        .file(std::path::Path::new("fdlibm/e_remainder.c"))
        .file(std::path::Path::new("fdlibm/e_scalb.c"))
        .file(std::path::Path::new("fdlibm/e_sinh.c"))
        .file(std::path::Path::new("fdlibm/e_sqrt.c"))
        .file(std::path::Path::new("fdlibm/k_cos.c"))
        .file(std::path::Path::new("fdlibm/k_rem_pio2.c"))
        .file(std::path::Path::new("fdlibm/k_sin.c"))
        .file(std::path::Path::new("fdlibm/k_standard.c"))
        .file(std::path::Path::new("fdlibm/k_tan.c"))
        .file(std::path::Path::new("fdlibm/s_asinh.c"))
        .file(std::path::Path::new("fdlibm/s_atan.c"))
        .file(std::path::Path::new("fdlibm/s_cbrt.c"))
        .file(std::path::Path::new("fdlibm/s_ceil.c"))
        .file(std::path::Path::new("fdlibm/s_copysign.c"))
        .file(std::path::Path::new("fdlibm/s_cos.c"))
        .file(std::path::Path::new("fdlibm/s_erf.c"))
        .file(std::path::Path::new("fdlibm/s_expm1.c"))
        .file(std::path::Path::new("fdlibm/s_fabs.c"))
        .file(std::path::Path::new("fdlibm/s_finite.c"))
        .file(std::path::Path::new("fdlibm/s_floor.c"))
        .file(std::path::Path::new("fdlibm/s_frexp.c"))
        .file(std::path::Path::new("fdlibm/s_ilogb.c"))
        .file(std::path::Path::new("fdlibm/s_isnan.c"))
        .file(std::path::Path::new("fdlibm/s_ldexp.c"))
        .file(std::path::Path::new("fdlibm/s_lib_version.c"))
        .file(std::path::Path::new("fdlibm/s_log1p.c"))
        .file(std::path::Path::new("fdlibm/s_logb.c"))
        .file(std::path::Path::new("fdlibm/s_matherr.c"))
        .file(std::path::Path::new("fdlibm/s_modf.c"))
        .file(std::path::Path::new("fdlibm/s_nextafter.c"))
        .file(std::path::Path::new("fdlibm/s_rint.c"))
        .file(std::path::Path::new("fdlibm/s_scalbn.c"))
        .file(std::path::Path::new("fdlibm/s_signgam.c"))
        .file(std::path::Path::new("fdlibm/s_significand.c"))
        .file(std::path::Path::new("fdlibm/s_sin.c"))
        .file(std::path::Path::new("fdlibm/s_tan.c"))
        .file(std::path::Path::new("fdlibm/s_tanh.c"))
        .file(std::path::Path::new("fdlibm/w_acos.c"))
        .file(std::path::Path::new("fdlibm/w_acosh.c"))
        .file(std::path::Path::new("fdlibm/w_asin.c"))
        .file(std::path::Path::new("fdlibm/w_atan2.c"))
        .file(std::path::Path::new("fdlibm/w_atanh.c"))
        .file(std::path::Path::new("fdlibm/w_cosh.c"))
        .file(std::path::Path::new("fdlibm/w_exp.c"))
        .file(std::path::Path::new("fdlibm/w_fmod.c"))
        .file(std::path::Path::new("fdlibm/w_gamma_r.c"))
        .file(std::path::Path::new("fdlibm/w_gamma.c"))
        .file(std::path::Path::new("fdlibm/w_hypot.c"))
        .file(std::path::Path::new("fdlibm/w_j0.c"))
        .file(std::path::Path::new("fdlibm/w_j1.c"))
        .file(std::path::Path::new("fdlibm/w_jn.c"))
        .file(std::path::Path::new("fdlibm/w_lgamma_r.c"))
        .file(std::path::Path::new("fdlibm/w_lgamma.c"))
        .file(std::path::Path::new("fdlibm/w_log.c"))
        .file(std::path::Path::new("fdlibm/w_log10.c"))
        .file(std::path::Path::new("fdlibm/w_pow.c"))
        .file(std::path::Path::new("fdlibm/w_remainder.c"))
        .file(std::path::Path::new("fdlibm/w_scalb.c"))
        .file(std::path::Path::new("fdlibm/w_sinh.c"))
        .file(std::path::Path::new("fdlibm/w_sqrt.c"));
    #[cfg(target_endian = "little")]
    {
        cfg.flag("-D__LITTLE_ENDIAN");
    }
    if !flags.is_empty() {
        for flag in flags.iter() {
            cfg.flag(&format!("-D{}", flag));
        }
    } else {
        cfg.flag("-D_IEEE_LIBM");
    }
    cfg.warnings(false);
    cfg.compile("fdlibm");
}

pub fn main() {
    if cfg!(target_endian = "big") {
        unimplemented!("");
    }

    println!("cargo:rerun-if-changed=fdlibm");
    println!("cargo:rerun-if-env-changed=_IEEE_LIBM");
    println!("cargo:rerun-if-env-changed=_IEEE_MODE");
    println!("cargo:rerun-if-env-changed=_XOPEN_MODE");
    println!("cargo:rerun-if-env-changed=_POSIX_MODE");
    println!("cargo:rerun-if-env-changed=_SVID3_MODE");

    let flags: std::collections::HashSet<String> = FLAGS_NAMES
        .iter()
        .filter_map(|flag| std::env::var(flag).ok().map(|_| flag.to_string()))
        .collect();

    bindgen_symbols("src/fdlibm.rs", &flags);
    /*
    # There are two options in making libm at fdlibm compile time:
    # 	_IEEE_LIBM 	--- IEEE libm; smaller, and somewhat faster
    #	_MULTI_LIBM	--- Support multi-standard at runtime by
    #			    imposing wrapper functions defined in
    #			    fdlibm.h:
    #				_IEEE_MODE 	-- IEEE
    #				_XOPEN_MODE 	-- X/OPEN
    #				_POSIX_MODE 	-- POSIX/ANSI
    #				_SVID3_MODE 	-- SVID
    # for more info check fdlibm/readme
    */
    build_bundled(&flags);
}
