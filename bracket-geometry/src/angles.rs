use crate::prelude::Point;

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
    use crate::prelude::{Point, project_angle};

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