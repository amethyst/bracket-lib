use std::cmp::{max, min};
mod line_bresenham;
pub mod lines;
pub mod point;
pub mod point3;
pub use {lines::line2d, point::Point, point3::Point3};
mod line_vector;
pub use line_bresenham::Bresenham;
pub use line_vector::VectorLine;
mod circle_bresenham;
pub use circle_bresenham::BresenhamCircle;
mod rect;
pub use rect::Rect;

/// Enumeration of available 2D Distance algorithms
pub enum DistanceAlg {
    Pythagoras,
    PythagorasSquared,
    Manhattan,
    Chebyshev,
}

impl DistanceAlg {
    /// Provides a 2D distance between points, using the specified algorithm.
    pub fn distance2d(self, start: Point, end: Point) -> f32 {
        match self {
            DistanceAlg::Pythagoras => distance2d_pythagoras(start, end),
            DistanceAlg::PythagorasSquared => distance2d_pythagoras_squared(start, end),
            DistanceAlg::Manhattan => distance2d_manhattan(start, end),
            DistanceAlg::Chebyshev => distance2d_chebyshev(start, end),
        }
    }
    /// Provides a 3D distance between points, using the specified algorithm.
    pub fn distance3d(self, start: Point3, end: Point3) -> f32 {
        match self {
            DistanceAlg::Pythagoras => distance3d_pythagoras(start, end),
            DistanceAlg::PythagorasSquared => distance3d_pythagoras_squared(start, end),
            DistanceAlg::Manhattan => distance3d_manhattan(start, end),
            DistanceAlg::Chebyshev => distance3d_pythagoras(start, end),
        }
    }
}

/// Enumeration of available 2D Distance algorithms
pub enum LineAlg {
    Bresenham,
    Vector,
}

/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
fn distance2d_pythagoras_squared(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    (dx * dx) + (dy * dy)
}

/// Calculates a Manhattan distance between two points
fn distance2d_manhattan(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    dx + dy
}

/// Calculates a Manhattan distance between two 3D points
fn distance3d_manhattan(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    dx + dy + dz
}

/// Calculates a Chebyshev distance between two points
/// See: http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html
fn distance2d_chebyshev(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    if dx > dy {
        (dx - dy) + 1.0 * dy
    } else {
        (dy - dx) + 1.0 * dx
    }
}

/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras_squared(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    (dx * dx) + (dy * dy) + (dz * dz)
}

/// Calculates a Pythagoras distance between two points.
fn distance2d_pythagoras(start: Point, end: Point) -> f32 {
    let dsq = distance2d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras(start: Point3, end: Point3) -> f32 {
    let dsq = distance3d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

/// From a given start point, project forward radius units at an angle of angle_radians degrees.
/// 0 Degrees is north (negative Y), 90 degrees is east (positive X)
pub fn project_angle(start: Point, radius: f32, angle_radians: f32) -> Point {
    let degrees_radians = angle_radians + std::f32::consts::PI;
    Point::new(
        (0.0 - (start.x as f32 + radius * f32::sin(degrees_radians))) as i32,
        (start.y as f32 + radius * f32::cos(degrees_radians)) as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::{project_angle, DistanceAlg, Point, Point3};

    #[test]
    fn test_pythagoras_distance() {
        let mut d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 7.071_068) < std::f32::EPSILON);
    }

    #[test]
    fn test_pythagoras_distance3d() {
        let mut d = DistanceAlg::Pythagoras.distance3d(Point3::new(0, 0, 0), Point3::new(5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance3d(Point3::new(0, 0, 0), Point3::new(-5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance3d(Point3::new(0, 0, 0), Point3::new(5, 5, 5));
        assert!(f32::abs(d - 8.660_254_5) < std::f32::EPSILON);
    }

    #[test]
    fn test_pythagoras_squared_distance() {
        let mut d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 50.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_pythagoras_squared_distance3d() {
        let mut d =
            DistanceAlg::PythagorasSquared.distance3d(Point3::new(0, 0, 0), Point3::new(5, 0, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance3d(Point3::new(0, 0, 0), Point3::new(-5, 0, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance3d(Point3::new(0, 0, 0), Point3::new(5, 5, 5));
        assert!(f32::abs(d - 75.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_manhattan_distance() {
        let mut d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 10.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_manhattan_distance3d() {
        let mut d = DistanceAlg::Manhattan.distance3d(Point3::new(0, 0, 0), Point3::new(5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance3d(Point3::new(0, 0, 0), Point3::new(-5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance3d(Point3::new(0, 0, 0), Point3::new(5, 5, 5));
        assert!(f32::abs(d - 15.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_chebyshev_distance() {
        let mut d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_project_angle() {
        let start = Point::new(0, 0);
        let mut dest = project_angle(start, 10.0, 0.0);
        assert_eq!(dest, Point::new(0, -10));

        dest = project_angle(start, 10.0, std::f32::consts::PI); // 180 degrees
        assert_eq!(dest, Point::new(0, 10));

        dest = project_angle(start, 10.0, std::f32::consts::PI / 2.0); // 90 degrees, east
        assert_eq!(dest, Point::new(10, 0));

        dest = project_angle(
            start,
            10.0,
            std::f32::consts::PI + (std::f32::consts::PI / 2.0),
        ); // 270 degrees, west
        assert_eq!(dest, Point::new(-10, 0));

        dest = project_angle(start, 10.0, std::f32::consts::FRAC_PI_4); // 45 degrees, north-east
        assert_eq!(dest, Point::new(7, -7));

        dest = project_angle(start, 10.0, 2.35619); // 135 degrees, south-east
        assert_eq!(dest, Point::new(7, 7));

        dest = project_angle(start, 10.0, 3.92699); // 225 degrees, south-west
        assert_eq!(dest, Point::new(-7, 7));

        dest = project_angle(start, 10.0, 5.49779); // 315 degrees, north-west
        assert_eq!(dest, Point::new(-7, -7));
    }
}
