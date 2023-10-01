
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::cos;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_cos(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_cos", |b| {
        b.iter(|| unsafe { black_box(cos(black_box(ARG1))) })
    });
}

fn rs_cos(c: &mut Criterion) {
    c.bench_function("rs_cos", |b| {
        b.iter(|| black_box(black_box(ARG1).cos()))
    });
}

criterion_group!(benches, fdlibm_rs_cos, rs_cos);
criterion_main!(benches);
