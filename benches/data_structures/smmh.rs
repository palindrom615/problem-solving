use criterion::{black_box, criterion_group, criterion_main, Criterion};
use problem_solving::data_structures::smmh::*;
use rand::Rng;

pub fn bench_smmh(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut s = SMMH::<i32>::with_capacity(100000);
    for i in 0..100000 {
        s.push(rng.gen());
    }
    c.bench_function("smmh push", |b| {
        b.iter(|| {
            s.pop_max();
            s.push(rng.gen());
        })
    });
}

criterion_group!(benches, bench_smmh);
criterion_main!(benches);
