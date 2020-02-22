#![allow(unused_variables)]

// Benchmark field of geometry calculations

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    use bracket_random::prelude::*;
    c.bench_function("next_u64", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            let n = rng.next_u64();
            black_box(n);
        })
    });

    c.bench_function("next_f64", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            let n = rng.rand::<f64>();
            black_box(n);
        })
    });

    c.bench_function("next_f32", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            let n = rng.rand::<f32>();
            black_box(n);
        })
    });

    c.bench_function("roll_3d6", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            let n = rng.roll_dice(3, 6);
            black_box(n);
        })
    });

    c.bench_function("roll_str_3d6+12", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            let n = rng.roll_str("3d6+12").unwrap();
            black_box(n);
        })
    });

    let options = [ "Cat", "Dog", "Gerbil", "Hamster", "Dragon" ];
    c.bench_function("random_slice_entry", |b| {
        let mut rng = RandomNumberGenerator::new();
        b.iter(|| {
            let n = rng.random_slice_entry(&options);
            black_box(n);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
