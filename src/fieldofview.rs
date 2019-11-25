use super::geometry::DistanceAlg;
use super::Algorithm2D;
use super::Point;
use std::collections::HashSet;

/// Calculates field-of-view for a map that supports Algorithm2D.
pub fn field_of_view(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> Vec<Point> {
    let mut result: HashSet<Point> = HashSet::new();

    let left = start.x - range;
    let right = start.x + range;
    let top = start.y - range;
    let bottom = start.y + range;
    let range_squared: f32 = (range as f32) * (range as f32);

    for x in left..=right {
        for pt in scan_fov_line(start, Point::new(x, top), range_squared, fov_check) {
            result.insert(pt);
        }
        for pt in scan_fov_line(start, Point::new(x, bottom), range_squared, fov_check) {
            result.insert(pt);
        }
    }

    for y in top+1..bottom {
        for pt in scan_fov_line(start, Point::new(left, y), range_squared, fov_check) {
            result.insert(pt);
        }
        for pt in scan_fov_line(start, Point::new(right, y), range_squared, fov_check) {
            result.insert(pt);
        }
    }

    let mut dedupe = Vec::new();
    for p in result.iter() {
        dedupe.push(*p);
    }
    dedupe
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

    for target in line.iter() {
        if !fov_check.in_bounds(*target) {
            // We're outside of the map
            break;
        }
        let dsq = DistanceAlg::PythagorasSquared.distance2d(start, *target);
        if dsq <= range_squared {
            result.push(*target);
            if fov_check.is_opaque(fov_check.point2d_to_index(*target)) {
                // FoV is blocked
                break;
            }
        } else {
            // FoV is out of range
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use crate::Point;
    use std::collections::HashSet;
    use std::hash::Hash;

    const TESTMAP_W : i32 = 20;
    const TESTMAP_H : i32 = 20;
    const TESTMAP_TILES : usize = (TESTMAP_W * TESTMAP_H) as usize;

    struct Map {
        pub tiles : Vec<bool>
    }

    impl Map {
        fn new() -> Map {
            Map {
                tiles : vec![false; TESTMAP_TILES]
            }
        }
    }

    fn mapidx(x: i32, y: i32) -> i32 {
        (y * TESTMAP_W) + x
    }

    impl crate::BaseMap for Map {
        fn is_opaque(&self, _idx: i32) -> bool { true }

        fn get_available_exits(&self, idx: i32) -> Vec<(i32, f32)> {
            let mut result : Vec<(i32, f32)> = Vec::new();
            let pos = (idx % TESTMAP_W, idx / TESTMAP_W);
            if pos.0 > 0 {
                result.push((mapidx(pos.0-1, pos.1), 1.0));
            }
            if pos.0 < TESTMAP_W-1 {
                result.push((mapidx(pos.0+1, pos.1), 1.0));
            }
            if pos.1 > 0 {
                result.push((mapidx(pos.0, pos.1 - 1), 1.0));
            }
            if pos.1 < TESTMAP_H -1 {
                result.push((mapidx(pos.0, pos.1 + 1), 1.0));
            }
            result
        }

        fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32 {
            super::DistanceAlg::Pythagoras.distance2d(
                Point::new( idx1 % TESTMAP_W, idx1 / TESTMAP_W ),
                Point::new( idx2 % TESTMAP_W, idx2 / TESTMAP_W )
            )
        }
    }

    impl super::Algorithm2D for Map {

        fn point2d_to_index(&self, pt: Point) -> i32 {
            (pt.y * TESTMAP_W) + pt.x
        }

        fn index_to_point2d(&self, idx: i32) -> Point {
            Point::new( idx % TESTMAP_W, idx / TESTMAP_W )
        }

        fn in_bounds(&self, pos : Point) -> bool {
            pos.x > 0 && pos.x < TESTMAP_W && pos.y > 0 && pos.y < TESTMAP_W
        }
    }

    fn has_unique_elements<T>(iter: T) -> bool
        where
            T: IntoIterator,
            T::Item: Eq + Hash,
        {
            let mut uniq = HashSet::new();
            iter.into_iter().all(move |x| uniq.insert(x))
        }

    // Tests that we are correctly de-duplicating field of view
    #[test]
    fn fov_dupes() {
        let map = Map::new();

        let visible = super::field_of_view(
            Point::new(10, 10), 
            8, 
            &map
        );

        assert!(has_unique_elements(&visible));
    }

    // Tests that the bounds-checking trait is applying properly to field-of-view checks
    #[test]
    fn fov_bounds_check() {
        let map = Map::new();

        let visible = super::field_of_view(
            Point::new(2, 2), 
            8, 
            &map
        );

        for t in visible.iter() {
            assert!(t.x > 0);
            assert!(t.x < TESTMAP_W);
            assert!(t.y > 0);
            assert!(t.y < TESTMAP_H);
        }
    }
}