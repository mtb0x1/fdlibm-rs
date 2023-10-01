
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::floor;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_floor(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_floor", |b| {
        b.iter(|| unsafe { black_box(floor(black_box(ARG1))) })
    });
}

fn rs_floor(c: &mut Criterion) {
    c.bench_function("rs_floor", |b| {
        b.iter(|| black_box(black_box(ARG1).floor()))
    });
}

criterion_group!(benches, fdlibm_rs_floor, rs_floor);
criterion_main!(benches);
