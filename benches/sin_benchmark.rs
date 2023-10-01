
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::sin;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_sin(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_sin", |b| {
        b.iter(|| unsafe { black_box(sin(black_box(ARG1))) })
    });
}

fn rs_sin(c: &mut Criterion) {
    c.bench_function("rs_sin", |b| {
        b.iter(|| black_box(black_box(ARG1).sin()))
    });
}

criterion_group!(benches, fdlibm_rs_sin, rs_sin);
criterion_main!(benches);
