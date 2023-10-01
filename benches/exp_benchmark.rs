
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::exp;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_exp(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_exp", |b| {
        b.iter(|| unsafe { black_box(exp(black_box(ARG1))) })
    });
}

fn rs_exp(c: &mut Criterion) {
    c.bench_function("rs_exp", |b| {
        b.iter(|| black_box(black_box(ARG1).exp()))
    });
}

criterion_group!(benches, fdlibm_rs_exp, rs_exp);
criterion_main!(benches);
