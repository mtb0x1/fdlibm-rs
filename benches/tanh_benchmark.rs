
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::tanh;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_tanh(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_tanh", |b| {
        b.iter(|| unsafe { black_box(tanh(black_box(ARG1))) })
    });
}

fn rs_tanh(c: &mut Criterion) {
    c.bench_function("rs_tanh", |b| {
        b.iter(|| black_box(black_box(ARG1).tanh()))
    });
}

criterion_group!(benches, fdlibm_rs_tanh, rs_tanh);
criterion_main!(benches);
