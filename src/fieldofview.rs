use super::geometry::DistanceAlg;
use super::Algorithm2D;
use super::Point;

/// Calculates field-of-view for a map that supports Algorithm2D.
pub fn field_of_view(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();

    let left = start.x - range;
    let right = start.x + range;
    let top = start.y - range;
    let bottom = start.y + range;
    let range_squared: f32 = (range as f32) * (range as f32);

    for x in left..=right {
        for pt in scan_fov_line(start, Point::new(x, top), range_squared, fov_check) {
            result.push(pt);
        }
        for pt in scan_fov_line(start, Point::new(x, bottom), range_squared, fov_check) {
            result.push(pt);
        }
    }

    for y in top..=bottom {
        for pt in scan_fov_line(start, Point::new(left, y), range_squared, fov_check) {
            result.push(pt);
        }
        for pt in scan_fov_line(start, Point::new(right, y), range_squared, fov_check) {
            result.push(pt);
        }
    }

    result
}

/// Helper method to scan along a line.
fn scan_fov_line(
    start: Point,
    end: Point,
    range_squared: f32,
    fov_check: &dyn Algorithm2D,
) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let line = super::line2d(super::LineAlg::Bresenham, start, end);

    let mut blocked = false;

    for target in &line {
        if !blocked {
            let dsq = DistanceAlg::PythagorasSquared.distance2d(start, *target);
            if dsq <= range_squared {
                if fov_check.is_opaque(fov_check.point2d_to_index(*target)) {
                    blocked = true;
                }
                result.push(*target);
            } else {
                blocked = true;
            }
        }
    }
    result
}
