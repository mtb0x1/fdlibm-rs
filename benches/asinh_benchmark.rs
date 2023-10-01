
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::asinh;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_asinh(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_asinh", |b| {
        b.iter(|| unsafe { black_box(asinh(black_box(ARG1))) })
    });
}

fn rs_asinh(c: &mut Criterion) {
    c.bench_function("rs_asinh", |b| {
        b.iter(|| black_box(black_box(ARG1).asinh()))
    });
}

criterion_group!(benches, fdlibm_rs_asinh, rs_asinh);
criterion_main!(benches);
