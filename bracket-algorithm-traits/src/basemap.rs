//! # `BaseMap`
//!
//! `BaseMap` provides map traits required for path-finding and field-of-view operations. Implement these
//! if you want to use these features from `bracket-lib`.
//!
//! `is_opaque` specifies is you can see through a tile, required for field-of-view.
//!
//! `get_available_exits` lists the indices to which one can travel from a given tile, along with a relative
//! cost of each exit. Required for path-finding.
//!
//! `get_pathing_distance` allows you to implement your heuristic for determining remaining distance to a
//! target.

use smallvec::SmallVec;

/// Implement this trait to support path-finding functions.
pub trait BaseMap {
    /// True is you cannot see through the tile, false otherwise. Default implementation
    /// always returns true, and is provided so you don't have to implement it if you
    /// aren't using it.
    fn is_opaque(&self, _idx: usize) -> bool {
        true
    }

    /// Return a vector of tile indices to which one can path from the idx.
    /// These do NOT have to be contiguous - if you want to support teleport pads, that's awesome.
    /// Default implementation is provided that proves an empty list, in case you aren't using
    /// it.
    ///
    /// Note that you should never return the current tile as an exit. The A* implementation
    /// really doesn't like that.
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        SmallVec::new()
    }

    /// Return the distance you would like to use for path-finding. Generally, Pythagoras distance (implemented in geometry)
    /// is fine, but you might use Manhattan or any other heuristic that fits your problem.
    /// Default implementation returns 1.0, which isn't what you want but prevents you from
    /// having to implement it when not using it.
    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        1.0
    }
}
