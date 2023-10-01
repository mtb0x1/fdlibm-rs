
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::lgamma;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_lgamma(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_lgamma", |b| {
        b.iter(|| unsafe { black_box(lgamma(black_box(ARG1))) })
    });
}
//@todo
/*
fn rs_lgamma(c: &mut Criterion) {
    c.bench_function("rs_lgamma", |b| {
        b.iter(|| black_box(black_box(ARG1).lgamma()))
    });
}*/

criterion_group!(benches, fdlibm_rs_lgamma/*, rs_lgamma*/);
criterion_main!(benches);
