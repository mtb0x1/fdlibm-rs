
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::gamma;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_gamma(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_gamma", |b| {
        b.iter(|| unsafe { black_box(gamma(black_box(ARG1))) })
    });
}

//@todo
/*
fn rs_gamma(c: &mut Criterion) {
    c.bench_function("rs_gamma", |b| {
        b.iter(|| black_box(black_box(ARG1).gamma()))
    });
}*/

criterion_group!(benches, fdlibm_rs_gamma/*, rs_gamma*/);
criterion_main!(benches);
