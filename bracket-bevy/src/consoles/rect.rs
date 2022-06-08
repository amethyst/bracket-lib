use super::point::Point;
use std::collections::HashSet;
use std::convert::TryInto;
use std::ops;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Default for Rect {
    fn default() -> Rect {
        Rect::zero()
    }
}

impl Rect {
    // Create a new rectangle, specifying X/Y Width/Height
    pub fn with_size<T>(x: T, y: T, w: T, h: T) -> Rect
    where
        T: TryInto<i32>,
    {
        let x_i32: i32 = x.try_into().ok().unwrap();
        let y_i32: i32 = y.try_into().ok().unwrap();
        Rect {
            x1: x_i32,
            y1: y_i32,
            x2: x_i32 + w.try_into().ok().unwrap(),
            y2: y_i32 + h.try_into().ok().unwrap(),
        }
    }

    // Create a new rectangle, specifying exact dimensions
    pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> Rect
    where
        T: TryInto<i32>,
    {
        Rect {
            x1: x1.try_into().ok().unwrap(),
            y1: y1.try_into().ok().unwrap(),
            x2: x2.try_into().ok().unwrap(),
            y2: y2.try_into().ok().unwrap(),
        }
    }

    // Creates a zero rectangle
    pub fn zero() -> Rect {
        Rect {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    // Returns the center of the rectangle
    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    // Returns true if a point is inside the rectangle
    pub fn point_in_rect(&self, point: Point) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    // Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                f(Point::new(x, y));
            }
        }
    }

    // Gets a set of all tiles in the rectangle
    pub fn point_set(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                result.insert(Point::new(x, y));
            }
        }
        result
    }

    // Returns the rectangle's width
    pub fn width(&self) -> i32 {
        i32::abs(self.x2 - self.x1)
    }

    // Returns the rectangle's height
    pub fn height(&self) -> i32 {
        i32::abs(self.y2 - self.y1)
    }
}

impl ops::Add<Rect> for Rect {
    type Output = Rect;
    fn add(mut self, rhs: Rect) -> Rect {
        let w = self.width();
        let h = self.height();
        self.x1 += rhs.x1;
        self.x2 = self.x1 + w;
        self.y1 += rhs.y1;
        self.y2 = self.y1 + h;
        self
    }
}
