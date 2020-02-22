use crate::prelude::BaseMap;
use bracket_geometry::prelude::Point3;

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm3D: BaseMap {
    /// Convert a Point (x/y) to an array index. Defaults to a Z, Y, X striding.
    #[allow(clippy::cast_sign_loss)]
    fn point3d_to_index(&self, pt: Point3) -> usize {
        let bounds = self.dimensions();
        ((pt.z * (bounds.x * bounds.y)) + (pt.y * bounds.x) + pt.x) as usize
    }

    /// Convert an array index to a point.
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    fn index_to_point3d(&self, idx: usize) -> Point3 {
        let mut my_idx = idx as i32;
        let bounds = self.dimensions();
        let z = my_idx / (bounds.x * bounds.y);
        my_idx -= z * bounds.x * bounds.y;

        let y = my_idx / bounds.x;
        my_idx -= y * bounds.x;

        let x = my_idx;
        Point3::new(x, y, z)
    }

    /// Dimensions
    fn dimensions(&self) -> Point3 {
        panic!("You must either define the dimensions function (trait Algorithm3D) on your map, or define the various point3d_to_index and index_to_point3d functions.");
    }

    // Optional - check that an x/y/z coordinate is within the map bounds. If not provided,
    // it falls back to using the map's dimensions from that trait implementation. Most of
    // the time, that's what you want.
    fn in_bounds(&self, pos: Point3) -> bool {
        let bounds = self.dimensions();
        pos.x > 0 && pos.x < bounds.x && pos.y > 0 && pos.y < bounds.y && pos.z > 0 && pos.z < bounds.z
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Algorithm3D, BaseMap};
    use bracket_geometry::prelude::Point3;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    #[should_panic]
    fn test_unimplemented_dimensions() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm3D for TestMap{}

        let map = TestMap{};
        assert!(map.in_bounds(Point3::new(1,1,1)));
    }

    #[test]
    fn test_in_bounds() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm3D for TestMap{
            fn dimensions(&self) -> Point3 {
                Point3::new(2, 2, 2)
            }
        }

        let map = TestMap{};
        assert!(map.in_bounds(Point3::new(1,1,1)));
        assert!(!map.in_bounds(Point3::new(3,3,3)));
    }

    #[test]
    fn test_point3d_to_index() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm3D for TestMap{
            fn dimensions(&self) -> Point3 {
                Point3::new(10, 10, 10)
            }
        }

        let map = TestMap{};
        assert!(map.point3d_to_index(Point3::new(0,0,0)) == 0);
        assert!(map.point3d_to_index(Point3::new(1,0,0)) == 1);
        assert!(map.point3d_to_index(Point3::new(0,1,0)) == 10);
        assert!(map.point3d_to_index(Point3::new(9,9,0)) == 99);
        assert!(map.point3d_to_index(Point3::new(9,9,9)) == 999);
    }

    #[test]
    fn test_index_to_point3d() {
        struct TestMap{};
        impl BaseMap for TestMap {}
        impl Algorithm3D for TestMap{
            fn dimensions(&self) -> Point3 {
                Point3::new(10, 10, 10)
            }
        }

        let map = TestMap{};
        let mut x = 0;
        let mut y = 0;
        let mut z: i32 = 0;
        for i in 0..1000 {
            assert!(map.index_to_point3d(i) == Point3::new(x, y, z));

            x += 1;
            if x > 9 {
                x = 0;
                y += 1;
            }
            if y > 9 {
                y = 0;
                z += 1;
            }
        }
    }
}