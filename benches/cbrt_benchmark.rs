
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::cbrt;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_cbrt(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_cbrt", |b| {
        b.iter(|| unsafe { black_box(cbrt(black_box(ARG1))) })
    });
}

fn rs_cbrt(c: &mut Criterion) {
    c.bench_function("rs_cbrt", |b| {
        b.iter(|| black_box(black_box(ARG1).cbrt()))
    });
}

criterion_group!(benches, fdlibm_rs_cbrt, rs_cbrt);
criterion_main!(benches);
