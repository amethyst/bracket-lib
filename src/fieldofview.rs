use super::{Algorithm2D, BresenhamCircle, Point, VectorLine};
use std::collections::HashSet;

/// Calculates field-of-view for a map that supports Algorithm2D, returning a HashSet. This is a bit faster
/// than coercing the results into a vector, since internally it uses the set for de-duplication.
pub fn field_of_view_set(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> HashSet<Point> {
    let mut visible_points: HashSet<Point> =
        HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

    BresenhamCircle::new(start, range).for_each(|point| {
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

    impl crate::BaseMap for Map {}

    impl super::Algorithm2D for Map {
        fn dimensions(&self) -> Point {
            Point::new(TESTMAP_W, TESTMAP_H)
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
