extern crate bresenham;
use bresenham::Bresenham;
use super::{Point, DistanceAlg, distance2d, LineAlg};

#[allow(dead_code)]
/// Plots a line between two 2D points and returns a vector of points along the line.
pub fn line2d(algorithm : LineAlg, start: Point, end: Point) -> Vec<Point> {
    match algorithm {
        LineAlg::Bresenham => { line2d_bresenham(start, end) }
        LineAlg::Vector => { line2d_vector(start, end) }
    }
}

#[allow(dead_code)]
/// Uses a Bresenham's algorithm to plot a line between two points. On some CPUs, this is faster
/// than Bresenham.
pub fn line2d_bresenham(start: Point, end: Point) -> Vec<Point> {
    let mut result : Vec<Point> = Vec::new();

    let line = Bresenham::new((start.x as isize, start.y as isize), (end.x as isize, end.y as isize));
    for p in line {
        result.push(Point::new(p.0 as i32, p.1 as i32));
    }
    result.push(end);

    result
}

#[allow(dead_code)]
/// Uses a 2D vector algorithm to plot a line between two points. On some CPUs, this is faster
/// than Bresenham.
pub fn line2d_vector(start: Point, end:Point) -> Vec<Point> {
    let mut pos : (f32, f32) = ( start.x as f32 + 0.5, start.y as f32 + 0.5 );
    let dest : (f32, f32) = ( end.x as f32 + 0.5, end.y as f32 + 0.5 );
    let n_steps = distance2d(DistanceAlg::Pythagoras, start, end);
    let slope : (f32, f32) = ( (dest.0 - pos.0 ) / n_steps, (dest.1 - pos.1 ) / n_steps );
    let mut result : Vec<Point> = Vec::new();
    result.push(start);

    let mut arrived = false;
    while !arrived {
        pos.0 += slope.0;
        pos.1 += slope.1;
        let new_point = Point::new(f32::floor(pos.0) as i32, f32::floor(pos.1) as i32);
        if result.is_empty() || result[result.len()-1] != new_point {
            result.push(new_point);
        }
        if new_point == end { arrived = true; }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{Point, line2d_vector, line2d_bresenham};

    #[test]
    fn vector_line_h() {
        let pt = Point::new(0,0);
        let pt2 = Point::new(5, 0);
        let line = line2d_vector(pt, pt2);
        assert_eq!(line, vec![Point::new(0,0), Point::new(1,0), Point::new(2,0), Point::new(3,0), Point::new(4,0), Point::new(5,0)]);
    }

    #[test]
    fn vector_line_v() {
        let pt = Point::new(0,0);
        let pt2 = Point::new(0, 5);
        let line = line2d_vector(pt, pt2);
        assert_eq!(line, vec![Point::new(0,0), Point::new(0,1), Point::new(0,2), Point::new(0,3), Point::new(0,4), Point::new(0,5)]);
    }

    #[test]
    fn vector_line_v_neg() {
        let pt = Point::new(0,0);
        let pt2 = Point::new(0, -5);
        let line = line2d_vector(pt, pt2);
        assert_eq!(line, vec![Point::new(0,0), Point::new(0,-1), Point::new(0,-2), Point::new(0,-3), Point::new(0,-4), Point::new(0,-5)]);
    }

    #[test]
    fn bresenham_line_h() {
        let pt = Point::new(0,0);
        let pt2 = Point::new(5, 0);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(line, vec![Point::new(0,0), Point::new(1,0), Point::new(2,0), Point::new(3,0), Point::new(4,0), Point::new(5,0)]);
    }

    #[test]
    fn bresenham_line_v() {
        let pt = Point::new(0,0);
        let pt2 = Point::new(0, 5);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(line, vec![Point::new(0,0), Point::new(0,1), Point::new(0,2), Point::new(0,3), Point::new(0,4), Point::new(0,5)]);
    }

    #[test]
    fn bresenham_line_v_neg() {
        let pt = Point::new(0,0);
        let pt2 = Point::new(0, -5);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(line, vec![Point::new(0,0), Point::new(0,-1), Point::new(0,-2), Point::new(0,-3), Point::new(0,-4), Point::new(0,-5)]);
    }
}