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
    fn get_available_exits(&self, _idx: usize) -> Vec<(usize, f32)> {
        Vec::new()
    }

    /// Return the distance you would like to use for path-finding. Generally, Pythagoras distance (implemented in geometry)
    /// is fine, but you might use Manhattan or any other heuristic that fits your problem.
    /// Default implementation returns 1.0, which isn't what you want but prevents you from
    /// having to implement it when not using it.
    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        1.0
    }
}
