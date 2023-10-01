
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::sinh;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_sinh(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_sinh", |b| {
        b.iter(|| unsafe { black_box(sinh(black_box(ARG1))) })
    });
}

fn rs_sinh(c: &mut Criterion) {
    c.bench_function("rs_sinh", |b| {
        b.iter(|| black_box(black_box(ARG1).sinh()))
    });
}

criterion_group!(benches, fdlibm_rs_sinh, rs_sinh);
criterion_main!(benches);
