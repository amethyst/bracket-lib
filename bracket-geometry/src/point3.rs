use std::convert::{TryInto, From};
use std::ops;
use ultraviolet::{Vec3,Vec3i};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
/// Helper struct defining a 2D point in space.
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[cfg(feature = "specs")]
impl specs::prelude::Component for Point3 {
    type Storage = specs::prelude::VecStorage<Self>;
}

impl Point3 {
    /// Create a new point from an x/y/z coordinate.
    pub fn new<T>(x: T, y: T, z: T) -> Self
    where
        T: TryInto<i32>,
    {
        Self {
            x: x.try_into().ok().unwrap(),
            y: y.try_into().ok().unwrap(),
            z: z.try_into().ok().unwrap(),
        }
    }

    /// Create a point from an x/y/z tuple
    pub fn from_tuple(t: (i32, i32, i32)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2
        }
    }

    /// Converts into an UltraViolet Vec3
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Converts into an UltraViolet Vec3
    pub fn to_vec3i(&self) -> Vec3i {
        Vec3i::new(self.x, self.y, self.z)
    }
}

impl From<Vec3> for Point3 {
    fn from(item: Vec3) -> Self {
        Self {
            x: item.x as i32,
            y: item.y as i32,
            z: item.z as i32
        }
    }
}

impl From<Vec3i> for Point3 {
    fn from(item: Vec3i) -> Self {
        Self {
            x: item.x,
            y: item.y,
            z: item.z
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////
/// Overloads: We support basic point math

/// Support adding a point to a point
impl ops::Add<Point3> for Point3 {
    type Output = Point3;
    fn add(mut self, rhs: Point3) -> Point3 {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

/// Support adding an int to a point
impl ops::Add<i32> for Point3 {
    type Output = Point3;
    fn add(mut self, rhs: i32) -> Point3 {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self
    }
}

/// Support subtracting a point from a point
impl ops::Sub<Point3> for Point3 {
    type Output = Point3;
    fn sub(mut self, rhs: Point3) -> Point3 {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

/// Support subtracting an int from a point
impl ops::Sub<i32> for Point3 {
    type Output = Point3;
    fn sub(mut self, rhs: i32) -> Point3 {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self
    }
}

/// Support multiplying a point by a point
impl ops::Mul<Point3> for Point3 {
    type Output = Point3;
    fn mul(mut self, rhs: Point3) -> Point3 {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }
}

/// Support multiplying a point by an int
impl ops::Mul<i32> for Point3 {
    type Output = Point3;
    fn mul(mut self, rhs: i32) -> Point3 {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

/// Support multiplying a point by an f32
impl ops::Mul<f32> for Point3 {
    type Output = Point3;
    fn mul(mut self, rhs: f32) -> Point3 {
        self.x = (self.x as f32 * rhs) as i32;
        self.y = (self.y as f32 * rhs) as i32;
        self.z = (self.z as f32 * rhs) as i32;
        self
    }
}

/// Support dividing a point by a point
impl ops::Div<Point3> for Point3 {
    type Output = Point3;
    fn div(mut self, rhs: Point3) -> Point3 {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self
    }
}

/// Support dividing a point by an int
impl ops::Div<i32> for Point3 {
    type Output = Point3;
    fn div(mut self, rhs: i32) -> Point3 {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

/// Support dividing a point by an f32
impl ops::Div<f32> for Point3 {
    type Output = Point3;
    fn div(mut self, rhs: f32) -> Point3 {
        self.x = (self.x as f32 / rhs) as i32;
        self.y = (self.y as f32 / rhs) as i32;
        self.z = (self.z as f32 / rhs) as i32;
        self
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
