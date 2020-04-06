use criterion::{black_box, criterion_group, criterion_main, Criterion};
use problem_solving::data_structures::smmh::*;
use rand::Rng;

pub fn bench_smmh(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut s = SMMH::with_capacity(10000);
    for i in 0..10000 {
        s.push(rng.gen_range(0, 10000));
    }
    c.bench_function("smmh push", |b| b.iter(|| s.pop_max()));
}

criterion_group!(benches, bench_smmh);
criterion_main!(benches);
