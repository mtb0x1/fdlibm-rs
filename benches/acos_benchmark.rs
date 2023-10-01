
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::acos;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_acos(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_acos", |b| {
        b.iter(|| unsafe { black_box(acos(black_box(ARG1))) })
    });
}

fn rs_acos(c: &mut Criterion) {
    c.bench_function("rs_acos", |b| {
        b.iter(|| black_box(black_box(ARG1).acos()))
    });
}

criterion_group!(benches, fdlibm_rs_acos, rs_acos);
criterion_main!(benches);
