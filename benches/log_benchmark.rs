
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::log;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_log(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_log", |b| {
        b.iter(|| unsafe { black_box(log(black_box(ARG1))) })
    });
}

fn rs_log(c: &mut Criterion) {
    c.bench_function("rs_log", |b| {
        b.iter(|| black_box(black_box(ARG1).log(2f64)))
    });
}

criterion_group!(benches, fdlibm_rs_log, rs_log);
criterion_main!(benches);
