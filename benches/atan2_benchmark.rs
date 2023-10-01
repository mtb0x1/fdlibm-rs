
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::atan2;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_atan2(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_atan2", |b| {
        b.iter(|| unsafe { black_box(atan2(black_box(ARG1),black_box(ARG1/2.0))) })
    });
}

fn rs_atan2(c: &mut Criterion) {
    c.bench_function("rs_atan2", |b| {
        b.iter(|| black_box(black_box(ARG1).atan2(black_box(ARG1/2.0))))
    });
}

criterion_group!(benches, fdlibm_rs_atan2, rs_atan2);
criterion_main!(benches);
