use criterion::{black_box, criterion_group, criterion_main, Criterion};
use problem_solving::data_structures::smmh::*;
use rand::Rng;
fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
pub fn bench_smmh(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut s = SMMH::with_capacity(1000);
    for _ in 1..100 {
        s.push(rng.gen_range(1, 100));
    }

    c.bench_function("smmh push", |b| b.iter(|| s.pop_max()));
}

criterion_group!(benches, bench_smmh);
criterion_main!(benches);
