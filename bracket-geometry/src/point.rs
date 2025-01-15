use std::convert::{From, TryInto};
use std::ops;
use ultraviolet::Vec2;

/// A 2D floating-point position.
pub type PointF = Vec2;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
/// Helper struct defining a 2D point in space.
pub struct Point {
    /// The point's X location
    pub x: i32,
    /// The point's Y location
    pub y: i32,
}

#[cfg(feature = "specs")]
impl specs::prelude::Component for Point {
    type Storage = specs::prelude::VecStorage<Self>;
}

#[cfg(feature = "bevy")]
impl bevy::ecs::component::Component for Point {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;
}

impl Point {
    /// Create a new point from an x/y coordinate.
    #[inline]
    #[must_use]
    pub fn new<T>(x: T, y: T) -> Point
    where
        T: TryInto<i32>,
    {
        Point {
            x: x.try_into().ok().unwrap_or(0),
            y: y.try_into().ok().unwrap_or(0),
        }
    }

    /// Create a new point from i32, this can be constant
    #[must_use]
    pub const fn constant(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Create a zero point
    #[inline]
    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    #[inline]
    /// Create a point from a tuple of two i32s
    pub fn from_tuple<T>(t: (T, T)) -> Self
    where
        T: TryInto<i32>,
    {
        Point::new(t.0, t.1)
    }

    #[inline]
    /// Helper for map index conversion
    #[must_use]
    pub fn to_index<T>(self, width: T) -> usize
    where
        T: TryInto<usize>,
    {
        let x: usize = self.x.try_into().ok().unwrap();
        let y: usize = self.y.try_into().ok().unwrap();
        let w: usize = width.try_into().ok().unwrap();
        (y * w) + x
    }

    /// Converts the point to an i32 tuple
    #[must_use]
    pub fn to_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Converts the point to a usize tuple
    ///
    /// # Panics
    ///
    /// This can panic if X or Y are not convertible to a `usize`.
    #[must_use]
    pub fn to_unsigned_tuple(self) -> (usize, usize) {
        (
            self.x.try_into().ok().unwrap(),
            self.y.try_into().ok().unwrap(),
        )
    }

    /// Converts the point to an UltraViolet vec2
    #[must_use]
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    /*
    // This doesn't seem to exist anymore?
    /// Converts the point to an UltraViolet vec2i
    pub fn to_vec2i(self) -> Vec2i {
        Vec2i::new(self.x, self.y)
    }
    */

    /// Creates a point from an `UltraViolet` vec2
    pub fn from_vec2(v: Vec2) -> Self {
        Self::new(v.x as i32, v.y as i32)
    }

    /*
    /// Creates a point from an `UltraViolet` vec2i
    pub fn from_vec2i(v: Vec2i) -> Self {
        Self::new(v.x, v.y)
    }
    */
}

impl From<(i32, i32)> for Point {
    fn from(item: (i32, i32)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl From<(f32, f32)> for Point {
    fn from(item: (f32, f32)) -> Self {
        Self {
            x: item.0 as i32,
            y: item.1 as i32,
        }
    }
}

impl From<Vec2> for Point {
    fn from(item: Vec2) -> Self {
        Self {
            x: item.x as i32,
            y: item.y as i32,
        }
    }
}

/*
impl From<Vec2i> for Point {
    fn from(item: Vec2i) -> Self {
        Self {
            x: item.x,
            y: item.y,
        }
    }
}
*/

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

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl ops::MulAssign for Point {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl ops::DivAssign for Point {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
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
    fn add_assign_point_to_point() {
        let mut pt = Point::new(0, 0);
        pt += Point::new(1, 2);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
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
    fn sub_assign_point_to_point() {
        let mut pt = Point::new(0, 0);
        pt -= Point::new(1, 2);
        assert_eq!(pt.x, -1);
        assert_eq!(pt.y, -2);
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
    fn mul_assign_point_to_point() {
        let mut pt = Point::new(1, 1);
        pt *= Point::new(1, 2);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
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
    fn div_assign_point_to_point() {
        let mut pt = Point::new(4, 4);
        pt /= Point::new(2, 4);
        assert_eq!(pt.x, 2);
        assert_eq!(pt.y, 1);
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
