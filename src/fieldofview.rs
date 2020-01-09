use super::{Algorithm2D, BresenhamCircle, Point, VectorLine};
use std::collections::HashSet;

/// Calculates field-of-view for a map that supports Algorithm2D, returning a HashSet. This is a bit faster
/// than coercing the results into a vector, since internally it uses the set for de-duplication.
pub fn field_of_view_set(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> HashSet<Point> {
    let mut visible_points: HashSet<Point> =
        HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

    BresenhamCircle::new(start.x, start.y, range).for_each(|point| {
        scan_fov_line(start, point, fov_check, &mut visible_points);
    });

    visible_points
}

/// Calculates field-of-view for a map that supports Algorithm2D.
pub fn field_of_view(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> Vec<Point> {
    field_of_view_set(start, range, fov_check)
        .into_iter()
        .collect()
}

/// Helper method to scan along a line.
fn scan_fov_line(
    start: Point,
    end: Point,
    fov_check: &dyn Algorithm2D,
    visible_points: &mut HashSet<Point>,
) {
    let line = VectorLine::new(start, end);

    for target in line {
        if !fov_check.in_bounds(target) {
            // We're outside of the map
            break;
        }
        visible_points.insert(target);
        if fov_check.is_opaque(fov_check.point2d_to_index(target)) {
            // FoV is blocked
            break;
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::Point;
    use std::collections::HashSet;
    use std::hash::Hash;

    const TESTMAP_W: usize = 20;
    const TESTMAP_H: usize = 20;
    const TESTMAP_TILES: usize = (TESTMAP_W * TESTMAP_H) as usize;

    struct Map {
        pub tiles: Vec<bool>,
    }

    impl Map {
        fn new() -> Map {
            Map {
                tiles: vec![false; TESTMAP_TILES],
            }
        }
    }

    fn mapidx(x: usize, y: usize) -> usize {
        (y * TESTMAP_W) + x
    }

    impl crate::BaseMap for Map {
        fn is_opaque(&self, _idx: usize) -> bool {
            true
        }

        fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
            let mut result: Vec<(usize, f32)> = Vec::new();
            let pos = (idx % TESTMAP_W, idx / TESTMAP_W);
            if pos.0 > 0 {
                result.push((mapidx(pos.0 - 1, pos.1), 1.0));
            }
            if pos.0 < TESTMAP_W - 1 {
                result.push((mapidx(pos.0 + 1, pos.1), 1.0));
            }
            if pos.1 > 0 {
                result.push((mapidx(pos.0, pos.1 - 1), 1.0));
            }
            if pos.1 < TESTMAP_H - 1 {
                result.push((mapidx(pos.0, pos.1 + 1), 1.0));
            }
            result
        }

        fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
            crate::DistanceAlg::Pythagoras.distance2d(
                Point::new(idx1 % TESTMAP_W, idx1 / TESTMAP_W),
                Point::new(idx2 % TESTMAP_W, idx2 / TESTMAP_W),
            )
        }
    }

    impl super::Algorithm2D for Map {
        fn point2d_to_index(&self, pt: Point) -> usize {
            ((pt.y * TESTMAP_W as i32) + pt.x) as usize
        }

        fn index_to_point2d(&self, idx: usize) -> Point {
            Point::new(idx % TESTMAP_W, idx / TESTMAP_W)
        }

        fn in_bounds(&self, pos: Point) -> bool {
            pos.x > 0 && pos.x < TESTMAP_W as i32 && pos.y > 0 && pos.y < TESTMAP_W as i32
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

        let visible = super::field_of_view(Point::new(10, 10), 8, &map);

        assert!(has_unique_elements(&visible));
    }

    // Tests that the bounds-checking trait is applying properly to field-of-view checks
    #[test]
    fn fov_bounds_check() {
        let map = Map::new();

        let visible = super::field_of_view(Point::new(2, 2), 8, &map);

        for t in visible.iter() {
            assert!(t.x > 0);
            assert!(t.x < TESTMAP_W as i32);
            assert!(t.y > 0);
            assert!(t.y < TESTMAP_H as i32);
        }
    }
}
