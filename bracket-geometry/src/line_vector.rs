use crate::prelude::Point;
use core::iter::Iterator;
use ultraviolet::Vec2;

/// Define a line using a fast 2D vector. It may not be as pixel-perfect as Bresenham, but with vectorization it is sometimes
/// faster for a quick line solution.
pub struct VectorLine {
    end: Point,
    current_pos: Vec2,
    slope: Vec2,
    finished: bool,
    really_finished: bool,
}

impl VectorLine {
    /// Define a vector line between two points.
    pub fn new(start: Point, end: Point) -> Self {
        let current_pos = Vec2::new(start.x as f32 + 0.5, start.y as f32 + 0.5);
        let destination = Vec2::new(end.x as f32 + 0.5, end.y as f32 + 0.5);
        let slope = (destination - current_pos).normalized();

        VectorLine {
            end,
            current_pos,
            slope,
            finished: false,
            really_finished: false,
        }
    }
}

impl Iterator for VectorLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            if !self.really_finished {
                self.really_finished = true;
                Some(Point::new(
                    self.current_pos.x as i32,
                    self.current_pos.y as i32,
                ))
            } else {
                None
            }
        } else {
            let current_point = Point::new(self.current_pos.x as i32, self.current_pos.y as i32);
            self.current_pos += self.slope;
            let new_point = Point::new(self.current_pos.x as i32, self.current_pos.y as i32);
            if new_point == self.end {
                self.finished = true;
            }
            Some(current_point)
        }
    }
}
