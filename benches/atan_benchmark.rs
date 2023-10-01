
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::atan;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_atan(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_atan", |b| {
        b.iter(|| unsafe { black_box(atan(black_box(ARG1))) })
    });
}

fn rs_atan(c: &mut Criterion) {
    c.bench_function("rs_atan", |b| {
        b.iter(|| black_box(black_box(ARG1).atan()))
    });
}

criterion_group!(benches, fdlibm_rs_atan, rs_atan);
criterion_main!(benches);
