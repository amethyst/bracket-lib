#![allow(unused_variables)]

// Benchmark field of geometry calculations

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    use rltk::{line2d, LineAlg, Point};
    c.bench_function("bresenham lines", |b| b.iter(|| {
        let line = line2d(
            LineAlg::Bresenham,
            Point::new(1, 150),
            Point::new(1, 105)
        );
        black_box(line);
    }));
    c.bench_function("vector lines", |b| b.iter(|| {
        let line = line2d(
            LineAlg::Vector,
            Point::new(1, 150),
            Point::new(1, 105)
        );
        black_box(line);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
