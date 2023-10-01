
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::atanh;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_atanh(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_atanh", |b| {
        b.iter(|| unsafe { black_box(atanh(black_box(ARG1))) })
    });
}

fn rs_atanh(c: &mut Criterion) {
    c.bench_function("rs_atanh", |b| {
        b.iter(|| black_box(black_box(ARG1).atanh()))
    });
}

criterion_group!(benches, fdlibm_rs_atanh, rs_atanh);
criterion_main!(benches);
