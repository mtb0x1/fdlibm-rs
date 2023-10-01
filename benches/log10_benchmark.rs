
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::log10;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_log10(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_log10", |b| {
        b.iter(|| unsafe { black_box(log10(black_box(ARG1))) })
    });
}

fn rs_log10(c: &mut Criterion) {
    c.bench_function("rs_log10", |b| {
        b.iter(|| black_box(black_box(ARG1).log10()))
    });
}

criterion_group!(benches, fdlibm_rs_log10, rs_log10);
criterion_main!(benches);
