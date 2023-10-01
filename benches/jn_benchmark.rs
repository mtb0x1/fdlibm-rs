
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::jn;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_jn(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_jn", |b| {
        b.iter(|| unsafe { black_box(jn(black_box(5),black_box(ARG1))) })
    });
}

//@todo
/*
fn rs_jn(c: &mut Criterion) {
    c.bench_function("rs_jn", |b| {
        b.iter(|| black_box(black_box(ARG1).jn()))
    });
}*/

criterion_group!(benches, fdlibm_rs_jn/*, rs_jn*/);
criterion_main!(benches);
