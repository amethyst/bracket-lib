#![allow(unused_variables)]

// Benchmark field of view calculations,
// most of the code copied from ex04-fov.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bracket_pathfinding::prelude::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("all fov 8", |b| {
        b.iter(|| {
            let s = State::new();
            let x = W / 2;
            let y = H / 2;
            let idx = xy_idx(x, y);
            if s.map[idx] != TileType::Wall {
                let p = Point::new(x, y);
                let fov = field_of_view_set(p, 8, &s);
                black_box(fov);
            }
        })
    });
    c.bench_function("all fov 20", |b| {
        b.iter(|| {
            let s = State::new();
            let x = W / 2;
            let y = H / 2;
            let idx = xy_idx(x, y);
            if s.map[idx] != TileType::Wall {
                let p = Point::new(x, y);
                let fov = field_of_view_set(p, 20, &s);
                black_box(fov);
            }
        })
    });
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

struct State {
    map: Vec<TileType>,
}

const H: i32 = 50;
const W: i32 = 80;

fn xy_idx(x: i32, y: i32) -> usize {
    ((y * W) + x) as usize
}

#[allow(dead_code)]
fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % W, idx as i32 / W)
}

impl BaseMap for State {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx] == TileType::Wall
    }
    fn get_available_exits(&self, _idx: usize) -> Vec<(usize, f32)> {
        Vec::new()
    }
    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        0.0
    }
}

impl Algorithm2D for State {
    fn dimensions(&self) -> Point {
        Point::new(W, H)
    }
}

impl State {
    pub fn new() -> Self {
        let state = State {
            map: vec![TileType::Floor; (W * H) as usize],
        };

        state
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
