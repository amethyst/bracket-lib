use super::Point;
use std::collections::HashSet;
use std::ops;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
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
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Create a new rectangle, specifying exact dimensions
    pub fn new_exact(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
        Rect { x1, y1, x2, y2 }
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
        point.x >= self.x1 && point.x <= self.x2 && point.y >= self.y1 && point.y <= self.y2
    }

    // Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y1..=self.y2 {
            for x in self.x1..=self.x2 {
                f(Point::new(x, y));
            }
        }
    }

    // Gets a set of all tiles in the rectangle
    pub fn point_set(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for y in self.y1..=self.y2 {
            for x in self.x1..=self.x2 {
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

#[cfg(test)]
mod tests {
    use super::{Point, Rect};

    #[test]
    fn test_dimensions() {
        let rect = Rect::new(0, 0, 10, 10);
        assert!(rect.width() == 10);
        assert!(rect.height() == 10);
    }

    #[test]
    fn test_add() {
        let rect = Rect::new(0, 0, 10, 10) + Rect::new(1, 1, 1, 1);
        assert!(rect.x1 == 1 && rect.y1 == 1);
        assert!(rect.x2 == 11 && rect.y2 == 11);
    }

    #[test]
    fn test_intersect() {
        let r1 = Rect::new(0, 0, 10, 10);
        let r2 = Rect::new(5, 5, 10, 10);
        let r3 = Rect::new(100, 100, 5, 5);
        assert!(r1.intersect(&r2));
        assert!(!r1.intersect(&r3));
    }

    #[test]
    fn test_center() {
        let r1 = Rect::new(0, 0, 10, 10);
        let center = r1.center();
        assert!(center.x == 5 && center.y == 5);
    }

    #[test]
    fn test_point_in_rect() {
        let r1 = Rect::new(0, 0, 10, 10);
        assert!(r1.point_in_rect(Point::new(5, 5)));
        assert!(!r1.point_in_rect(Point::new(100, 100)));
    }

    #[test]
    fn test_rect_set() {
        let r1 = Rect::new(0, 0, 1, 1);
        let points = r1.point_set();
        assert!(points.contains(&Point::new(0, 0)));
        assert!(points.contains(&Point::new(1, 0)));
        assert!(points.contains(&Point::new(0, 1)));
        assert!(points.contains(&Point::new(1, 1)));
    }

    #[test]
    fn test_rect_callback() {
        use std::collections::HashSet;

        let r1 = Rect::new(0, 0, 1, 1);
        let mut points: HashSet<Point> = HashSet::new();
        r1.for_each(|p| {
            points.insert(p);
        });
        assert!(points.contains(&Point::new(0, 0)));
        assert!(points.contains(&Point::new(1, 0)));
        assert!(points.contains(&Point::new(0, 1)));
        assert!(points.contains(&Point::new(1, 1)));
    }
}
