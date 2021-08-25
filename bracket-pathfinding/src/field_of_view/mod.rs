use bracket_algorithm_traits::prelude::Algorithm2D;
use bracket_geometry::prelude::Point;

use std::collections::HashSet;

mod recursive_shadowcasting;
// Default algorithm / backwards compatibility
pub use recursive_shadowcasting::{field_of_view, field_of_view_set};
mod symmetric_shadowcasting;

/// Enumeration of available FOV algorithms
#[derive(Clone, Copy)]
#[non_exhaustive] // Other algorithms may be added in the future
pub enum FieldOfViewAlg {
    RecursiveShadowcasting,
    SymmetricShadowcasting,
}

impl FieldOfViewAlg {
    pub fn field_of_view_set(
        self,
        center: Point,
        range: i32,
        fov_check: &dyn Algorithm2D,
    ) -> HashSet<Point> {
        match self {
            FieldOfViewAlg::RecursiveShadowcasting => {
                recursive_shadowcasting::field_of_view_set(center, range, fov_check)
            }
            FieldOfViewAlg::SymmetricShadowcasting => {
                symmetric_shadowcasting::field_of_view_set(center, range, fov_check)
            }
        }
    }
    pub fn field_of_view(
        self,
        start: Point,
        range: i32,
        fov_check: &dyn Algorithm2D,
    ) -> Vec<Point> {
        match self {
            FieldOfViewAlg::RecursiveShadowcasting => {
                recursive_shadowcasting::field_of_view(start, range, fov_check)
            }
            FieldOfViewAlg::SymmetricShadowcasting => {
                symmetric_shadowcasting::field_of_view(start, range, fov_check)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::FieldOfViewAlg;
    use bracket_algorithm_traits::prelude::{Algorithm2D, BaseMap};
    use bracket_geometry::prelude::{BresenhamCircle, Point};
    use std::cmp::max;
    use std::collections::HashSet;
    use std::hash::Hash;

    const TESTMAP_W: usize = 20;
    const TESTMAP_H: usize = 20;
    const TESTMAP_TILES: usize = (TESTMAP_W * TESTMAP_H) as usize;
    const ALGORITHS: [FieldOfViewAlg; 2] = [
        FieldOfViewAlg::RecursiveShadowcasting,
        FieldOfViewAlg::SymmetricShadowcasting,
    ];

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

    // The map needs to be see-through for the tests to check FOV
    impl BaseMap for Map {
        fn is_opaque(&self, idx: usize) -> bool {
            self.tiles[idx]
        }
    }

    impl Algorithm2D for Map {
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

        for alg in ALGORITHS {
            let visible = alg.field_of_view(Point::new(10, 10), 8, &map);

            assert!(has_unique_elements(&visible));
        }
    }

    // Tests that the bounds-checking trait is applying properly to field-of-view checks
    #[test]
    fn fov_bounds_check() {
        let map = Map::new();

        for alg in ALGORITHS {
            let visible = alg.field_of_view(Point::new(2, 2), 8, &map);

            for t in visible.iter() {
                assert!(t.x >= 0);
                assert!(t.x < TESTMAP_W as i32 - 1);
                assert!(t.y >= 0);
                assert!(t.y < TESTMAP_H as i32 - 1);
            }
        }
    }

    // Tests that the FOV scan does not miss any interior points
    #[test]
    fn fov_inclusive() {
        for radius in 4..=9 {
            let map = Map::new();
            let dimensions = map.dimensions();
            let c = Point::new(10, 10);
            for alg in ALGORITHS {
                let visible = alg.field_of_view(c, radius, &map);
                // let max_radius_sq: i32 = visible.iter().fold(0, |max_r2, p| {
                let max_radius_sq: i32 = BresenhamCircle::new(c, radius).fold(0, |max_r2, p| {
                    let r2 = (p.x - c.x) * (p.x - c.x) + (p.y - c.y) * (p.y - c.y);
                    max(r2, max_r2)
                });
                /*
                for y in 0..dimensions.y {
                    let mut s = "".to_string();
                    for x in 0..dimensions.x {
                        let point = Point::new(x, y);
                        let c = if visible.contains(&point) {
                            '.'
                        } else {
                            '#'
                        };
                        s.push(c);
                    }
                    println!("{}", s);
                }
                */
                for x in 0..dimensions.x {
                    for y in 0..dimensions.y {
                        let r2 = (x - c.x) * (x - c.x) + (y - c.y) * (y - c.y);
                        let point = Point::new(x, y);
                        assert!(
                            r2 >= max_radius_sq || visible.contains(&point),
                            format!("Interior point ({:?}) not in FOV({})", point, radius)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn fov_corridor() {
        let mut map = Map::new();
        let c = Point::new(10, 10);
        let radius: i32 = 5;

        for i in 0..20 {
            let idx = 9 * 20 + i;
            map.tiles[idx] = true;
            let idx = 11 * 20 + i;
            map.tiles[idx] = true;
        }

        for alg in ALGORITHS {
            let visible = alg.field_of_view(c, radius, &map);
            for i in 1..radius * 2 - 2 {
                let pos = Point::new(c.x - radius + i, c.y);
                assert!(visible.contains(&pos));
                let pos = Point::new(c.x - radius + i, c.y - 1);
                assert!(visible.contains(&pos), format!("{:?} not in result", pos));
                let pos = Point::new(c.x - radius + i, c.y + 1);
                assert!(visible.contains(&pos));
            }
        }
    }
}
