
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::asin;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_asin(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_asin", |b| {
        b.iter(|| unsafe { black_box(asin(black_box(ARG1))) })
    });
}

fn rs_asin(c: &mut Criterion) {
    c.bench_function("rs_asin", |b| {
        b.iter(|| black_box(black_box(ARG1).asin()))
    });
}

criterion_group!(benches, fdlibm_rs_asin, rs_asin);
criterion_main!(benches);
