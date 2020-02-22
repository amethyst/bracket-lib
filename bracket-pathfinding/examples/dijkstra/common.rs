use bracket_pathfinding::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;
use crossterm::queue;
use crossterm::style::{Color::Rgb, Print, SetForegroundColor};
use std::io::{stdout, Write};
use bracket_color::prelude::*;

// Console Support

pub fn print_color(color: RGB, text: &str) {
    queue!(
        stdout(),
        SetForegroundColor(Rgb {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        })
    )
    .expect("Command Fail");
    queue!(stdout(), Print(text)).expect("Command fail");
}

pub fn flush_console() {
    stdout().flush().expect("Flush Fail");
}

// Map

pub const MAP_WIDTH : usize = 80;
pub const MAP_HEIGHT : usize = 20;
pub const MAP_TILES : usize = MAP_WIDTH * MAP_HEIGHT;
pub const START_POINT : Point = Point::constant(2, MAP_HEIGHT as i32 / 2);
pub const END_POINT : Point = Point::constant(MAP_WIDTH as i32 - 2, MAP_HEIGHT as i32 / 2);

pub struct Map {
    pub tiles : Vec<char>
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = Self {
            tiles : vec!['.' ; MAP_TILES]
        };

        // Add random walls
        let n_walls = 200;
        let mut rng = RandomNumberGenerator::new();
        for _ in 0 .. n_walls {
            let target = Point::new(
                rng.roll_dice(1, MAP_WIDTH as i32-1),
                rng.roll_dice(1, MAP_HEIGHT as i32-1)
            );
            if target != START_POINT && target != END_POINT {
                let idx = tiles.point2d_to_index(target);
                tiles.tiles[idx] = '#';
            }
        }

        tiles
    }

    fn valid_exit(&self, loc : Point, delta : Point) -> Option<usize> {
        let destination = loc + delta;
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

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits: Vec<(usize, f32)> = Vec::new();
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
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::PythagorasSquared.distance2d(
            self.index_to_point2d(idx1),
            self.index_to_point2d(idx2)
        )
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }
}