#![allow(unused_variables)]

// Benchmark field of geometry calculations

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    use bracket_terminal::prelude::*;
    c.bench_function("small_batch", |b| {
        b.iter(|| {
            let mut draw_batch = DrawBatch::new();
            draw_batch.cls();
            draw_batch.print(Point::new(1, 1), "Hello");
            draw_batch.print(Point::new(1, 2), "World");
            draw_batch.submit(0).expect("Batch error");
            clear_command_buffer();
        })
    });
    c.bench_function("large_batch", |b| {
        b.iter(|| {
            let mut draw_batch = DrawBatch::new();
            draw_batch.cls();
            for i in 0..1000 {
                draw_batch.print(Point::new(1, 1), "Hello");
                draw_batch.print(Point::new(1, 2), "World");
            }
            draw_batch.submit(0).expect("Batch error");
            clear_command_buffer();
        })
    });
    c.bench_function("multi_batch", |b| {
        b.iter(|| {
            let mut draw_batch = DrawBatch::new();
            draw_batch.cls();
            for j in 0..10 {
                for i in 0..1000 {
                    draw_batch.print(Point::new(1, 1), "Hello");
                    draw_batch.print(Point::new(1, 2), "World");
                }
                draw_batch.submit(j * 1000).expect("Batch error");
            }
            clear_command_buffer();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
