
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::fabs;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_fabs(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_fabs", |b| {
        b.iter(|| unsafe { black_box(fabs(black_box(ARG1))) })
    });
}

fn rs_fabs(c: &mut Criterion) {
    c.bench_function("rs_fabs", |b| {
        b.iter(|| black_box(black_box(ARG1).abs()))
    });
}

criterion_group!(benches, fdlibm_rs_fabs, rs_fabs);
criterion_main!(benches);
