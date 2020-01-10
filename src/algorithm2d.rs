use super::{BaseMap, Point};
use std::convert::TryInto;

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm2D: BaseMap {
    /// Convert a Point (x/y) to an array index. Defaults to an index based on an array
    /// strided X first.
    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x).try_into().expect("Not a valid usize")
    }

    /// Convert an array index to a point. Defaults to an index based on an array
    /// strided X first.
    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w : usize = bounds.x.try_into().expect("Not a valid usize");
        Point::new(
            idx % w,
            idx / w
        )
    }

    /// Retrieve the map's dimensions
    fn dimensions(&self) -> Point;

    // Optional - check that an x/y coordinate is within the map bounds. If not provided,
    // it falls back to using the map's dimensions from that trait implementation. Most of
    // the time, that's what you want.
    fn in_bounds(&self, pos: Point) -> bool {
        let bounds = self.dimensions();
        pos.x > 0 && pos.x < bounds.x && pos.y > 0 && pos.y < bounds.y
    }    
}