#![allow(unused_variables)]

// Benchmark field of view calculations,
// most of the code copied from ex04-fov.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bracket_pathfinding::prelude::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("a_star_test_map", |b| {
        b.iter(|| {
            let map = Map::new();
            let path = a_star_search(
                map.point2d_to_index(START_POINT),
                map.point2d_to_index(END_POINT),
                &map,
            );
            black_box(path);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use bracket_random::prelude::RandomNumberGenerator;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 20;
pub const MAP_TILES: usize = MAP_WIDTH * MAP_HEIGHT;
pub const START_POINT: Point = Point::constant(2, MAP_HEIGHT as i32 / 2);
pub const END_POINT: Point = Point::constant(MAP_WIDTH as i32 - 2, MAP_HEIGHT as i32 / 2);

pub struct Map {
    pub tiles: Vec<char>,
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = Self {
            tiles: vec!['.'; MAP_TILES],
        };

        // Add random walls
        let n_walls = 200;
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..n_walls {
            let target = Point::new(
                rng.roll_dice(1, MAP_WIDTH as i32 - 1),
                rng.roll_dice(1, MAP_HEIGHT as i32 - 1),
            );
            if target != START_POINT && target != END_POINT {
                let idx = tiles.point2d_to_index(target);
                tiles.tiles[idx] = '#';
            }
        }

        tiles
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;

        if destination.x < 0 || destination.y < 0 {
            return None
        }

        let idx = self.point2d_to_index(destination);
        if self.in_bounds(destination) && self.tiles[idx] == '.' {
            Some(idx)
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == '#'
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }
}
