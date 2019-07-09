use super::Point;
use std::cmp::{max, min};

extern crate bresenham;
use bresenham::Bresenham;

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
pub fn distance2d_squared(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min (start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min (start.y, end.y)) as f32;
    (dx * dx) + (dy * dy)
}

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two points.
pub fn distance2d(start: Point, end: Point) -> f32 {
    let dsq = distance2d_squared(start, end);
    f32::sqrt(dsq)
}

#[allow(dead_code)]
/// Plots a line between two 2D points and returns a vector of points along the line.
pub fn line2d(start: Point, end: Point) -> Vec<Point> {
    let mut result : Vec<Point> = Vec::new();

    let line = Bresenham::new((start.x as isize, start.y as isize), (end.x as isize, end.y as isize));
    for p in line {
        result.push(Point::new(p.0 as i32, p.1 as i32));
    }

    result
}