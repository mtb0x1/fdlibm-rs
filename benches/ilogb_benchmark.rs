
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::ilogb;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_ilogb(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_ilogb", |b| {
        b.iter(|| unsafe { black_box(ilogb(black_box(ARG1))) })
    });
}

fn rs_ilogb(c: &mut Criterion) {
    c.bench_function("rs_log2", |b| {
        b.iter(|| black_box(black_box(ARG1).log2()))
    });
}

criterion_group!(benches, fdlibm_rs_ilogb, rs_ilogb);
criterion_main!(benches);
