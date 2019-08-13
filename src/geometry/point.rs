use std::ops;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
/// Helper struct defining a 2D point in space.
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point from an x/y coordinate.
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

///////////////////////////////////////////////////////////////////////////////////////
/// Overloads: We support basic point math

/// Support adding a point to a point
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(mut self, rhs: Point) -> Point {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

/// Support adding an int to a point
impl ops::Add<i32> for Point {
    type Output = Point;
    fn add(mut self, rhs: i32) -> Point {
        self.x += rhs;
        self.y += rhs;
        self
    }
}

/// Support subtracting a point from a point
impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(mut self, rhs: Point) -> Point {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

/// Support subtracting an int from a point
impl ops::Sub<i32> for Point {
    type Output = Point;
    fn sub(mut self, rhs: i32) -> Point {
        self.x -= rhs;
        self.y -= rhs;
        self
    }
}

/// Support multiplying a point by a point
impl ops::Mul<Point> for Point {
    type Output = Point;
    fn mul(mut self, rhs: Point) -> Point {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self
    }
}

/// Support multiplying a point by an int
impl ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(mut self, rhs: i32) -> Point {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

/// Support multiplying a point by an f32
impl ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(mut self, rhs: f32) -> Point {
        self.x = (self.x as f32 * rhs) as i32;
        self.y = (self.y as f32 * rhs) as i32;
        self
    }
}

/// Support dividing a point by a point
impl ops::Div<Point> for Point {
    type Output = Point;
    fn div(mut self, rhs: Point) -> Point {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self
    }
}

/// Support dividing a point by an int
impl ops::Div<i32> for Point {
    type Output = Point;
    fn div(mut self, rhs: i32) -> Point {
        self.x /= rhs;
        self.y /= rhs;
        self
    }
}

/// Support dividing a point by an f32
impl ops::Div<f32> for Point {
    type Output = Point;
    fn div(mut self, rhs: f32) -> Point {
        self.x = (self.x as f32 / rhs) as i32;
        self.y = (self.y as f32 / rhs) as i32;
        self
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn new_point() {
        let pt = Point::new(1, 2);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
    }

    #[test]
    fn add_point_to_point() {
        let pt = Point::new(0, 0);
        let p2 = pt + Point::new(1, 2);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn add_point_to_int() {
        let pt = Point::new(0, 0);
        let p2 = pt + 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn sub_point_to_point() {
        let pt = Point::new(0, 0);
        let p2 = pt - Point::new(1, 2);
        assert_eq!(p2.x, -1);
        assert_eq!(p2.y, -2);
    }

    #[test]
    fn sub_point_to_int() {
        let pt = Point::new(0, 0);
        let p2 = pt - 2;
        assert_eq!(p2.x, -2);
        assert_eq!(p2.y, -2);
    }

    #[test]
    fn mul_point_to_point() {
        let pt = Point::new(1, 1);
        let p2 = pt * Point::new(1, 2);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn mul_point_to_int() {
        let pt = Point::new(1, 1);
        let p2 = pt * 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn mul_point_to_float() {
        let pt = Point::new(1, 1);
        let p2 = pt * 4.0;
        assert_eq!(p2.x, 4);
        assert_eq!(p2.y, 4);
    }

    #[test]
    fn div_point_to_point() {
        let pt = Point::new(4, 4);
        let p2 = pt / Point::new(2, 4);
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 1);
    }

    #[test]
    fn div_point_to_int() {
        let pt = Point::new(4, 4);
        let p2 = pt / 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn div_point_to_float() {
        let pt = Point::new(4, 4);
        let p2 = pt / 2.0;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
    }
}
