
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::j1;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_j1(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_j1", |b| {
        b.iter(|| unsafe { black_box(j1(black_box(ARG1))) })
    });
}

//@todo
/*
fn rs_j1(c: &mut Criterion) {
    c.bench_function("rs_j1", |b| {
        b.iter(|| black_box(black_box(ARG1).j1()))
    });
}*/

criterion_group!(benches, fdlibm_rs_j1/*, rs_j1*/);
criterion_main!(benches);
