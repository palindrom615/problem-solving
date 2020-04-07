use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{VecDeque, HashSet};
use rand::Rng;

pub fn bench_vecdeque(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut queue = VecDeque::with_capacity(500000);
    for i in 0..500000 {
        queue.push_back((rng.gen::<i32>(), rng.gen::<i32>()));
    }
 
    c.bench_function("vecdeque pop_front", |b| {
        b.iter(|| {
            queue.pop_front();
        })
    });
    c.bench_function("vecdeque len", |b| {
        b.iter(|| {
            queue.len();
        })
    });
}

pub fn bench_vec(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(500000);
    for i in 0..500000 {
        vec.push((rng.gen::<i32>(), rng.gen::<i32>()));
    }
    c.bench_function("vec pop", |b| {
        b.iter(|| {
            vec.pop();
        })
    });
}

pub fn bench_hashset(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut set = HashSet::with_capacity(500000);
    let mut vec = Vec::with_capacity(500000);

    for i in 0..500000 {
        let r = rng.gen::<i32>();
        vec.push(r);
        set.insert(r);
    }
    let mut i = 0;
    c.bench_function("set remove", |b| {
        b.iter(|| {
            i += 1;
            set.remove(&vec[i % 500000]);
        })
    });
}

criterion_group!(benches, bench_vecdeque, bench_vec, bench_hashset);
criterion_main!(benches);
