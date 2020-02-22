use super::{BaseMap, Point3};

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm3D: BaseMap {
    /// Convert a Point (x/y) to an array index.
    fn point3d_to_index(&self, pt: Point3) -> usize;

    /// Convert an array index to a point.
    fn index_to_point3d(&self, idx: usize) -> Point3;
}
