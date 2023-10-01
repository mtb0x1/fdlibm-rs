
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::pow;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_pow(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_pow", |b| {
        b.iter(|| unsafe { black_box(pow(black_box(ARG1),black_box(5f64))) })
    });
}

fn rs_pow(c: &mut Criterion) {
    c.bench_function("rs_pow", |b| {
        b.iter(|| black_box(black_box(ARG1).powf(black_box(5f64))))
    });
}

criterion_group!(benches, fdlibm_rs_pow, rs_pow);
criterion_main!(benches);
