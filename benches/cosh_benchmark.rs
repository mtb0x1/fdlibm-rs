
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::cosh;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_cosh(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_cosh", |b| {
        b.iter(|| unsafe { black_box(cosh(black_box(ARG1))) })
    });
}

fn rs_cosh(c: &mut Criterion) {
    c.bench_function("rs_cosh", |b| {
        b.iter(|| black_box(black_box(ARG1).cosh()))
    });
}

criterion_group!(benches, fdlibm_rs_cosh, rs_cosh);
criterion_main!(benches);
