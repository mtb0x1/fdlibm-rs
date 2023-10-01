
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::j0;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_j0(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_j0", |b| {
        b.iter(|| unsafe { black_box(j0(black_box(ARG1))) })
    });
}

//@todo
/*
fn rs_j0(c: &mut Criterion) {
    c.bench_function("rs_j0", |b| {
        b.iter(|| black_box(black_box(ARG1).j0()))
    });
}*/

criterion_group!(benches, fdlibm_rs_j0/*, rs_j0*/);
criterion_main!(benches);
