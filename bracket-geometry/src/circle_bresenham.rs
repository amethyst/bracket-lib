use crate::prelude::Point;

/// An implementation of [Bresenham's circle algorithm].
/// [Bresenham's circle algorithm]: http://members.chello.at/~easyfilter/bresenham.html
/// Derived from the line_drawing crate, but specialized to use BTerm's types.
pub struct BresenhamCircle {
    x: i32,
    y: i32,
    center: Point,
    radius: i32,
    error: i32,
    quadrant: u8,
}

impl BresenhamCircle {
    #[inline]
    #[allow(dead_code)]
    pub fn new(center: Point, radius: i32) -> Self {
        Self {
            center,
            radius,
            x: -radius,
            y: 0,
            error: 2 - 2 * radius,
            quadrant: 1,
        }
    }
}

impl Iterator for BresenhamCircle {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 {
            let point = match self.quadrant {
                1 => (self.center.x - self.x, self.center.y + self.y),
                2 => (self.center.x - self.y, self.center.y - self.x),
                3 => (self.center.x + self.x, self.center.y - self.y),
                4 => (self.center.x + self.y, self.center.y + self.x),
                _ => unreachable!(),
            };

            // Update the variables after each set of quadrants
            if self.quadrant == 4 {
                self.radius = self.error;

                if self.radius <= self.y {
                    self.y += 1;
                    self.error += self.y * 2 + 1;
                }

                if self.radius > self.x || self.error > self.y {
                    self.x += 1;
                    self.error += self.x * 2 + 1;
                }
            }

            self.quadrant = self.quadrant % 4 + 1;

            Some(Point::from_tuple(point))
        } else {
            None
        }
    }
}

/// A version of the Bresenham circle that does not make diagonal jumps
pub struct BresenhamCircleNoDiag {
    x: i32,
    y: i32,
    center: Point,
    // radius: i32,
    error: i32,
    quadrant: u8,
}

impl BresenhamCircleNoDiag {
    #[inline]
    #[allow(dead_code)]
    pub fn new(center: Point, radius: i32) -> Self {
        Self {
            center,
            x: -radius,
            y: 0,
            error: 0,
            quadrant: 1,
        }
    }
}

impl Iterator for BresenhamCircleNoDiag {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 {
            let point = match self.quadrant {
                1 => (self.center.x - self.x, self.center.y + self.y),
                2 => (self.center.x - self.y, self.center.y - self.x),
                3 => (self.center.x + self.x, self.center.y - self.y),
                4 => (self.center.x + self.y, self.center.y + self.x),
                _ => unreachable!(),
            };

            // Update the variables after each set of quadrants.
            if self.quadrant == 4 {
                // This version moves in x or in y - not both - depending on the error.
                if (self.error + 2 * self.x + 1).abs() <= (self.error + 2 * self.y + 1).abs() {
                    self.error += self.x * 2 + 1;
                    self.x += 1;
                } else {
                    self.error += self.y * 2 + 1;
                    self.y += 1;
                }
            }

            self.quadrant = self.quadrant % 4 + 1;

            Some(Point::from_tuple(point))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{BresenhamCircle, BresenhamCircleNoDiag, Point};

    #[test]
    fn circle_test_radius1() {
        let circle = BresenhamCircle::new(Point::new(0, 0), 1);
        let points: Vec<Point> = circle.collect();
        assert_eq!(
            points,
            vec![
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(-1, 0),
                Point::new(0, -1)
            ]
        );
    }

    #[test]
    fn circle_test_radius3() {
        let circle = BresenhamCircle::new(Point::new(0, 0), 3);
        let points: Vec<Point> = circle.collect();
        assert_eq!(
            points,
            vec![
                Point { x: 3, y: 0 },
                Point { x: 0, y: 3 },
                Point { x: -3, y: 0 },
                Point { x: 0, y: -3 },
                Point { x: 3, y: 1 },
                Point { x: -1, y: 3 },
                Point { x: -3, y: -1 },
                Point { x: 1, y: -3 },
                Point { x: 2, y: 2 },
                Point { x: -2, y: 2 },
                Point { x: -2, y: -2 },
                Point { x: 2, y: -2 },
                Point { x: 1, y: 3 },
                Point { x: -3, y: 1 },
                Point { x: -1, y: -3 },
                Point { x: 3, y: -1 }
            ]
        );
    }

    #[test]
    fn circle_nodiag_test_radius3() {
        let circle = BresenhamCircleNoDiag::new(Point::new(0, 0), 3);
        let points: Vec<Point> = circle.collect();
        assert_eq!(
            points,
            vec![
                Point { x: 3, y: 0 },
                Point { x: 0, y: 3 },
                Point { x: -3, y: 0 },
                Point { x: 0, y: -3 },
                Point { x: 3, y: 1 },
                Point { x: -1, y: 3 },
                Point { x: -3, y: -1 },
                Point { x: 1, y: -3 },
                Point { x: 2, y: 1 },
                Point { x: -1, y: 2 },
                Point { x: -2, y: -1 },
                Point { x: 1, y: -2 },
                Point { x: 2, y: 2 },
                Point { x: -2, y: 2 },
                Point { x: -2, y: -2 },
                Point { x: 2, y: -2 },
                Point { x: 1, y: 2 },
                Point { x: -2, y: 1 },
                Point { x: -1, y: -2 },
                Point { x: 2, y: -1 },
                Point { x: 1, y: 3 },
                Point { x: -3, y: 1 },
                Point { x: -1, y: -3 },
                Point { x: 3, y: -1 }
            ]
        );
    }
}
