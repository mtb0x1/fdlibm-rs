
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::acosh;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_acosh(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_acosh", |b| {
        b.iter(|| unsafe { black_box(acosh(black_box(ARG1))) })
    });
}

fn rs_acosh(c: &mut Criterion) {
    c.bench_function("rs_acosh", |b| {
        b.iter(|| black_box(black_box(ARG1).acosh()))
    });
}

criterion_group!(benches, fdlibm_rs_acosh, rs_acosh);
criterion_main!(benches);
