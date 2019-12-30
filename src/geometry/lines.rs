use super::{DistanceAlg, LineAlg, Point};
use bresenham::Bresenham;

/// Plots a line between two 2D points and returns a vector of points along the line.
pub fn line2d(algorithm: LineAlg, start: Point, end: Point) -> Vec<Point> {
    match algorithm {
        LineAlg::Bresenham => line2d_bresenham(start, end),
        LineAlg::Vector => line2d_vector(start, end),
    }
}

/// Uses a Bresenham's algorithm to plot a line between two points. On some CPUs, this is faster
/// than Bresenham.
pub fn line2d_bresenham(start: Point, end: Point) -> Vec<Point> {
    let line = Bresenham::new(
        (start.x as isize, start.y as isize),
        (end.x as isize, end.y as isize),
    );
    line.map(|p| Point::new(p.0 as i32, p.1 as i32))
        .chain(std::iter::once(end))
        .collect()
}

/// Uses a 2D vector algorithm to plot a line between two points. On some CPUs, this is faster
/// than Bresenham.
pub fn line2d_vector(start: Point, end: Point) -> Vec<Point> {
    if start == end {
        return vec![start];
    }
    let mut pos: (f32, f32) = (start.x as f32 + 0.5, start.y as f32 + 0.5);
    let dest: (f32, f32) = (end.x as f32 + 0.5, end.y as f32 + 0.5);
    let n_steps = DistanceAlg::Pythagoras.distance2d(start, end);
    let slope: (f32, f32) = ((dest.0 - pos.0) / n_steps, (dest.1 - pos.1) / n_steps);
    let mut result: Vec<Point> = Vec::with_capacity(n_steps as usize);
    result.push(start);

    loop {
        pos.0 += slope.0;
        pos.1 += slope.1;
        let new_point = Point::new(pos.0 as i32, pos.1 as i32);
        if result[result.len() - 1] != new_point {
            result.push(new_point);
            if new_point == end {
                // arrived
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{line2d_bresenham, line2d_vector, Point};

    #[test]
    fn vector_line_h() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(5, 0);
        let line = line2d_vector(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0)
            ]
        );
    }

    #[test]
    fn vector_line_v() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, 5);
        let line = line2d_vector(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
                Point::new(0, 4),
                Point::new(0, 5)
            ]
        );
    }

    #[test]
    fn vector_line_v_neg() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, -5);
        let line = line2d_vector(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(0, -2),
                Point::new(0, -3),
                Point::new(0, -4),
                Point::new(0, -5)
            ]
        );
    }

    #[test]
    fn bresenham_line_h() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(5, 0);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0)
            ]
        );
    }

    #[test]
    fn bresenham_line_v() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, 5);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
                Point::new(0, 4),
                Point::new(0, 5)
            ]
        );
    }

    #[test]
    fn bresenham_line_v_neg() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, -5);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(0, -2),
                Point::new(0, -3),
                Point::new(0, -4),
                Point::new(0, -5)
            ]
        );
    }
}
