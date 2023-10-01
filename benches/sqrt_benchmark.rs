
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::sqrt;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_sqrt(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_sqrt", |b| {
        b.iter(|| unsafe { black_box(sqrt(black_box(ARG1))) })
    });
}

fn rs_sqrt(c: &mut Criterion) {
    c.bench_function("rs_sqrt", |b| {
        b.iter(|| black_box(black_box(ARG1).sqrt()))
    });
}

criterion_group!(benches, fdlibm_rs_sqrt, rs_sqrt);
criterion_main!(benches);
