
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::expm1;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_expm1(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_expm1", |b| {
        b.iter(|| unsafe { black_box(expm1(black_box(ARG1))) })
    });
}

fn rs_expm1(c: &mut Criterion) {
    c.bench_function("rs_expm1", |b| {
        b.iter(|| black_box(black_box(ARG1).exp_m1()))
    });
}

criterion_group!(benches, fdlibm_rs_expm1, rs_expm1);
criterion_main!(benches);
