#![allow(unused_variables)]

// Benchmark field of geometry calculations

#[macro_use]
extern crate criterion;

extern crate rand;
use crate::rand::Rng;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    use rltk::{line2d, LineAlg, Point};
    c.bench_function("bresenham lines", |b| b.iter(|| {
        let mut rng = rand::thread_rng();
        let line = line2d(
            LineAlg::Bresenham, 
            Point::new(rng.gen_range(1,200), rng.gen_range(1,200)), 
            Point::new(rng.gen_range(1,200), rng.gen_range(1,200)), 
        );
    }));
    c.bench_function("vector lines", |b| b.iter(|| {
        let mut rng = rand::thread_rng();
        let line = line2d(
            LineAlg::Vector, 
            Point::new(rng.gen_range(1,200), rng.gen_range(1,200)), 
            Point::new(rng.gen_range(1,200), rng.gen_range(1,200)), 
        );
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
