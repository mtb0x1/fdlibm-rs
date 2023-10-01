
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::log1p;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_log1p(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_log1p", |b| {
        b.iter(|| unsafe { black_box(log1p(black_box(ARG1))) })
    });
}

//@todo
/*
fn rs_log1p(c: &mut Criterion) {
    c.bench_function("rs_log1p", |b| {
        b.iter(|| black_box(black_box(ARG1).log1p()))
    });
}*/

criterion_group!(benches, fdlibm_rs_log1p/*, rs_log1p*/);
criterion_main!(benches);
