
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::ceil;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_ceil(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_ceil", |b| {
        b.iter(|| unsafe { black_box(ceil(black_box(ARG1))) })
    });
}

fn rs_ceil(c: &mut Criterion) {
    c.bench_function("rs_ceil", |b| {
        b.iter(|| black_box(black_box(ARG1).ceil()))
    });
}

criterion_group!(benches, fdlibm_rs_ceil, rs_ceil);
criterion_main!(benches);
