use super::Point;
use super::geometry::distance2d_squared;
use super::Algorithm2D;

extern crate bresenham;
use bresenham::Bresenham;

#[allow(dead_code)]
pub fn field_of_view(start : Point, range : i32, fov_check : &Algorithm2D) -> Vec<Point> {
    let mut result : Vec<Point> = Vec::new();

    let left = start.x - range;
    let right = start.x + range;
    let top = start.y - range;
    let bottom = start.y + range;
    let range_squared : f32 = (range as f32) * (range as f32);

    for x in left .. right+1 {
        for pt in scan_fov_line(start, Point::new(x, top, ), range_squared, fov_check) {
            result.push(pt);
        }
        for pt in scan_fov_line(start, Point::new(x, bottom), range_squared, fov_check) {
            result.push(pt);
        }
    }

    for y in top .. bottom+1 {
        for pt in scan_fov_line(start, Point::new(left, y), range_squared, fov_check) {
            result.push(pt);
        }
        for pt in scan_fov_line(start, Point::new(right, y), range_squared, fov_check) {
            result.push(pt);
        }
    }

    return result;
}

fn scan_fov_line(start: Point, end: Point, range_squared : f32, fov_check : &Algorithm2D) -> Vec<Point> {
    let mut result : Vec<Point> = Vec::new();
    let line = Bresenham::new((start.x as isize, start.y as isize), (end.x as isize, end.y as isize));

    let mut blocked = false;

    for (x, y) in line {
        if !blocked {
            let target = Point::new(x as i32, y as i32);
            let dsq = distance2d_squared(start, target);
            if dsq <= range_squared {
                if fov_check.is_opaque(fov_check.point2d_to_index(target)) {
                    blocked = true;
                }
                result.push(target);
            } else {
                blocked = true;
            }
        }
    }
    return result;
}
