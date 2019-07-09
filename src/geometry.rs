use super::Point;
use std::cmp::{max, min};

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
pub fn distance2d_squared(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min (start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min (start.y, end.y)) as f32;
    return (dx * dx) + (dy * dy);
}

#[allow(dead_code)]
/// Calculates a Pythagoras distance between two points.
pub fn distance2d(start: Point, end: Point) -> f32 {
    let dsq = distance2d_squared(start, end);
    return f32::sqrt(dsq);
}