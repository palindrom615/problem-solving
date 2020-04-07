use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::VecDeque;
use rand::Rng;

pub fn bench_vecdeque(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut queue = VecDeque::with_capacity(100000);
    for i in 0..100000 {
        queue.push_back(rng.gen::<i32>());
    }
 
    c.bench_function("vecdeque pop_front", |b| {
        b.iter(|| {
            queue.pop_front();
        })
    });
}

pub fn bench_vec(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(100000);
    for i in 0..100000 {
        vec.push(rng.gen::<i32>());
    }
    c.bench_function("vec pop", |b| {
        b.iter(|| {
            vec.pop();
        })
    });
}
criterion_group!(benches, bench_vecdeque, bench_vec);
criterion_main!(benches);
