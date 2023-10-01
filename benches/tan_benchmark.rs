
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::tan;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_tan(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_tan", |b| {
        b.iter(|| unsafe { black_box(tan(black_box(ARG1))) })
    });
}

fn rs_tan(c: &mut Criterion) {
    c.bench_function("rs_tan", |b| {
        b.iter(|| black_box(black_box(ARG1).tan()))
    });
}

criterion_group!(benches, fdlibm_rs_tan, rs_tan);
criterion_main!(benches);
