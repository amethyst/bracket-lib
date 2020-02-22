use crate::prelude::BaseMap;
use std::convert::TryInto;
use bracket_geometry::prelude::Point;

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm2D: BaseMap {
    /// Convert a Point (x/y) to an array index. Defaults to an index based on an array
    /// strided X first.
    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x)
            .try_into()
            .expect("Not a valid usize")
    }

    /// Convert an array index to a point. Defaults to an index based on an array
    /// strided X first.
    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w: usize = bounds.x.try_into().expect("Not a valid usize");
        Point::new(idx % w, idx / w)
    }

    /// Retrieve the map's dimensions. Made optional to reduce API breakage.
    fn dimensions(&self) -> Point {
        panic!("You must either define the dimensions function (trait Algorithm2D) on your map, or define the various point2d_to_index and index_to_point2d functions.");
    }

    // Optional - check that an x/y coordinate is within the map bounds. If not provided,
    // it falls back to using the map's dimensions from that trait implementation. Most of
    // the time, that's what you want.
    fn in_bounds(&self, pos: Point) -> bool {
        let bounds = self.dimensions();
        pos.x > 0 && pos.x < bounds.x && pos.y > 0 && pos.y < bounds.y
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Algorithm2D, BaseMap};
    use bracket_geometry::prelude::Point;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    #[should_panic]
    fn test_unimplemented_dimensions() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm2D for TestMap{}

        let map = TestMap{};
        assert!(map.in_bounds(Point::new(1,1)));
    }

    #[test]
    fn test_in_bounds() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm2D for TestMap{
            fn dimensions(&self) -> Point {
                Point::new(2, 2)
            }
        }

        let map = TestMap{};
        assert!(map.in_bounds(Point::new(1,1)));
        assert!(!map.in_bounds(Point::new(3,3)));
    }

    #[test]
    fn test_point2d_to_index() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm2D for TestMap{
            fn dimensions(&self) -> Point {
                Point::new(10, 10)
            }
        }

        let map = TestMap{};
        assert!(map.point2d_to_index(Point::new(0,0)) == 0);
        assert!(map.point2d_to_index(Point::new(1,0)) == 1);
        assert!(map.point2d_to_index(Point::new(0,1)) == 10);
        assert!(map.point2d_to_index(Point::new(9,9)) == 99);
    }

    #[test]
    fn test_index_to_point2d() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm2D for TestMap{
            fn dimensions(&self) -> Point {
                Point::new(10, 10)
            }
        }

        let map = TestMap{};
        let mut x = 0;
        let mut y = 0;
        for i in 0..100 {
            assert!(map.index_to_point2d(i) == Point::new(x, y));

            x += 1;
            if x > 9 {
                x = 0;
                y += 1;
            }
        }
    }
}