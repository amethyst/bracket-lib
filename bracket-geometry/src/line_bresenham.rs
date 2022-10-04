//! Original at: <https://github.com/mbr/bresenham-rs/blob/master/src/lib.rs>
//! Modified to use more BTerm-friendly types
//!
use crate::prelude::Point;
use core::iter::Iterator;

/// Line-drawing iterator
pub struct Bresenham {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    x1: i32,
    diff: i32,
    octant: Octant,
}

struct Octant(u8);

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    #[inline]
    fn from_points(start: Point, end: Point) -> Octant {
        let mut dx = end.x - start.x;
        let mut dy = end.y - start.y;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2;
        }

        if dx < dy {
            octant += 1;
        }

        Octant(octant)
    }

    #[inline]
    fn to_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => Point::new(p.x, p.y),
            1 => Point::new(p.y, p.x),
            2 => Point::new(p.y, -p.x),
            3 => Point::new(-p.x, p.y),
            4 => Point::new(-p.x, -p.y),
            5 => Point::new(-p.y, -p.x),
            6 => Point::new(-p.y, p.x),
            7 => Point::new(p.x, -p.y),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn from_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => Point::new(p.x, p.y),
            1 => Point::new(p.y, p.x),
            2 => Point::new(-p.y, p.x),
            3 => Point::new(-p.x, p.y),
            4 => Point::new(-p.x, -p.y),
            5 => Point::new(-p.y, -p.x),
            6 => Point::new(p.y, -p.x),
            7 => Point::new(p.x, -p.y),
            _ => unreachable!(),
        }
    }
}

impl Bresenham {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Point, end: Point) -> Bresenham {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start);
        let end = octant.to_octant0(end);

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        Bresenham {
            x: start.x,
            y: start.y,
            dx,
            dy,
            x1: end.x,
            diff: dy - dx,
            octant,
        }
    }

        /// Return the next point without checking if we are past `end`.
        #[inline]
        pub fn advance(&mut self) -> Point {
            let p = Point::new(self.x, self.y);
    
            if self.diff >= 0 {
                self.y += 1;
                self.diff -= self.dx;
            }
    
            self.diff += self.dy;
    
            // loop inc
            self.x += 1;
    
            self.octant.from_octant0(p)
        }
}

impl Iterator for Bresenham {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            None
        } else {
            Some(self.advance())
        }
    }
}

/// New type over `Bresenham` which include the `end` points when iterated over.
pub struct BresenhamInclusive(Bresenham);
impl BresenhamInclusive {
    /// Creates a new iterator. Yields points `start..=end`.
    #[inline]
    pub fn new(start: Point, end: Point) -> Self {
        Self(Bresenham::new(start, end))
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> Point {
        self.0.advance()
    }
}
impl Iterator for BresenhamInclusive {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.x > self.0.x1 {
            None
        } else {
            Some(self.0.advance())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_empty() {
        let bi = Bresenham::new(Point::new(0, 0), Point::new(0, 0));
        let res: Vec<_> = bi.collect();
        assert_eq!(res, []);

        let bi = BresenhamInclusive::new(Point::new(0, 0), Point::new(0, 0));
        let res: Vec<_> = bi.collect();
        assert_eq!(res, [Point::new(0, 0)]);

        let mut bi = BresenhamInclusive::new(Point::new(0, 0), Point::new(0, 0));
        bi.advance();
        let res: Vec<_> = bi.collect();
        assert_eq!(res, []);
    }

    #[test]
    fn test_wp_example() {
        let start = Point::new(0, 1);
        let end = Point::new(6, 4);

        let bi = Bresenham::new(start, end);
        let res: Vec<_> = bi.collect();
        let mut expected = vec![
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(4, 3),
            Point::new(5, 3)
        ];
        assert_eq!(res, expected);

        let bi = BresenhamInclusive::new(start, end);
        let res: Vec<_> = bi.collect();
        expected.push(end);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_inverse_wp() {
        let start = Point::new(6, 4);
        let end = Point::new(0, 1);

        let bi = Bresenham::new(start, end);
        let res: Vec<_> = bi.collect();
        let mut expected = vec![
            Point::new(6, 4),
            Point::new(5, 4),
            Point::new(4, 3),
            Point::new(3, 3),
            Point::new(2, 2),
            Point::new(1, 2)
        ];
        assert_eq!(res, expected);

        let bi = BresenhamInclusive::new(start, end);
        let res: Vec<_> = bi.collect();
        expected.push(end);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_straight_hline() {
        let start = Point::new(2, 3);
        let end = Point::new(5, 3);

        let bi = Bresenham::new(start, end);
        let res: Vec<_> = bi.collect();
        let mut expected = vec![Point::new(2, 3), Point::new(3, 3), Point::new(4, 3)];
        assert_eq!(res, expected);

        let bi = BresenhamInclusive::new(start, end);
        let res: Vec<_> = bi.collect();
        expected.push(end);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_straight_vline() {
        let start = Point::new(2, 3);
        let end = Point::new(2, 6);

        let bi = Bresenham::new(start, end);
        let res: Vec<_> = bi.collect();
        let mut expected = vec![Point::new(2, 3), Point::new(2, 4), Point::new(2, 5)];
        assert_eq!(res, expected);

        let bi = BresenhamInclusive::new(start, end);
        let res: Vec<_> = bi.collect();
        expected.push(end);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_issue135_line() {
        let line = Bresenham::new(Point::new(0, 6), Point::new(6, 0));
        let res: Vec<Point> = line.collect();
        assert!(res.len() == 6);
        res.iter().for_each(|p| {
            assert!(p.x >= 0);
            assert!(p.x < 7);
            assert!(p.y >= 0);
            assert!(p.y < 7);
        });
    }

    #[test]
    fn test_line_sweep() {
        use crate::prelude::*;
        let mut angle = Degrees::new(0.0);
        let start_point = Point::new(20, 20);
        while angle.0 < 360.0 {
            let end_point = project_angle(Point::new(0, 0), 8.0, angle) + start_point;
            let line = Bresenham::new(start_point, end_point);
            let res: Vec<Point> = line.collect();
            assert!(res.len() > 0);
            res.iter().for_each(|p| {
                assert!(p.x >= 10);
                assert!(p.x < 30);
                assert!(p.y >= 10);
                assert!(p.y < 30);
            });
            angle.0 += 1.0;
        }
    }
}
