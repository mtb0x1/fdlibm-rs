
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fdlibm_rs::ldexp;

pub const ARG1: ::core::ffi::c_double = 0.055605003447049994_f64;

fn fdlibm_rs_ldexp(c: &mut Criterion) {
    c.bench_function("fdlibm_rs_ldexp", |b| {
        b.iter(|| unsafe { black_box(ldexp(black_box(ARG1),black_box(5))) })
    });
}
//@todo
/*
fn rs_ldexp(c: &mut Criterion) {
    c.bench_function("rs_ldexp", |b| {
        b.iter(|| black_box(ldexp(black_box(ARG1),black_box(4))))
    });
}*/

criterion_group!(benches, fdlibm_rs_ldexp/*, rs_ldexp*/);
criterion_main!(benches);
