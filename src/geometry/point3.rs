use std::ops;

#[cfg(not(feature = "serialization"))]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
/// Helper struct defining a 2D point in space.
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[cfg(feature = "serialization")]
#[derive(Eq, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize, Debug)]
/// Helper struct defining a 2D point in space.
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    /// Create a new point from an x/y/z coordinate.
    pub fn new(x: i32, y: i32, z: i32) -> Point3 {
        Point3 { x, y, z }
    }
}

///////////////////////////////////////////////////////////////////////////////////////
/// Overloads: We support basic point math

/// Support adding a point to a point
impl ops::Add<Point3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Point3) -> Point3 {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/// Support adding an int to a point
impl ops::Add<i32> for Point3 {
    type Output = Point3;
    fn add(self, rhs: i32) -> Point3 {
        Point3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

/// Support subtracting a point from a point
impl ops::Sub<Point3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Point3) -> Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// Support subtracting an int from a point
impl ops::Sub<i32> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: i32) -> Point3 {
        Point3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

/// Support multiplying a point by a point
impl ops::Mul<Point3> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: Point3) -> Point3 {
        Point3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

/// Support multiplying a point by an int
impl ops::Mul<i32> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: i32) -> Point3 {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// Support multiplying a point by an f32
impl ops::Mul<f32> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: f32) -> Point3 {
        Point3::new(
            (self.x as f32 * rhs) as i32,
            (self.y as f32 * rhs) as i32,
            (self.z as f32 * rhs) as i32,
        )
    }
}

/// Support dividing a point by a point
impl ops::Div<Point3> for Point3 {
    type Output = Point3;
    fn div(self, rhs: Point3) -> Point3 {
        Point3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

/// Support dividing a point by an int
impl ops::Div<i32> for Point3 {
    type Output = Point3;
    fn div(self, rhs: i32) -> Point3 {
        Point3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

/// Support dividing a point by an f32
impl ops::Div<f32> for Point3 {
    type Output = Point3;
    fn div(self, rhs: f32) -> Point3 {
        Point3::new(
            (self.x as f32 / rhs) as i32,
            (self.y as f32 / rhs) as i32,
            (self.z as f32 / rhs) as i32,
        )
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::Point3;

    #[test]
    fn new_point3() {
        let pt = Point3::new(1, 2, 3);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
        assert_eq!(pt.z, 3);
    }

    #[test]
    fn add_point_to_point3() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt + Point3::new(1, 2, 3);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 3);
    }

    #[test]
    fn add_point3_to_int() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt + 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }

    #[test]
    fn sub_point3_to_point() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt - Point3::new(1, 2, 3);
        assert_eq!(p2.x, -1);
        assert_eq!(p2.y, -2);
        assert_eq!(p2.z, -3);
    }

    #[test]
    fn sub_point3_to_int() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt - 2;
        assert_eq!(p2.x, -2);
        assert_eq!(p2.y, -2);
        assert_eq!(p2.z, -2);
    }

    #[test]
    fn mul_point3_to_point() {
        let pt = Point3::new(1, 1, 1);
        let p2 = pt * Point3::new(1, 2, 4);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 4);
    }

    #[test]
    fn mul_point3_to_int() {
        let pt = Point3::new(1, 1, 1);
        let p2 = pt * 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }

    #[test]
    fn mul_point3_to_float() {
        let pt = Point3::new(1, 1, 1);
        let p2 = pt * 4.0;
        assert_eq!(p2.x, 4);
        assert_eq!(p2.y, 4);
        assert_eq!(p2.z, 4);
    }

    #[test]
    fn div_point3_to_point() {
        let pt = Point3::new(4, 4, 4);
        let p2 = pt / Point3::new(2, 4, 1);
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 1);
        assert_eq!(p2.z, 4);
    }

    #[test]
    fn div_point3_to_int() {
        let pt = Point3::new(4, 4, 4);
        let p2 = pt / 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }

    #[test]
    fn div_point3_to_float() {
        let pt = Point3::new(4, 4, 4);
        let p2 = pt / 2.0;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }
}
