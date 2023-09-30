use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::acos;

fn fdlibm_rs_acos(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_acos(20)", |b| b.iter(|| unsafe{black_box(acos(black_box(20f64)))}));
}

fn rs_acos(c: &mut Criterion) {
    c.bench_function("rs_acos(20)", |b| b.iter(|| black_box(black_box(20f64).acos())));
}

criterion_group!(benches, fdlibm_rs_acos, rs_acos);
criterion_main!(benches);