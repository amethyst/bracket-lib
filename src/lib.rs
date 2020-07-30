//! bracket-lib is a wrapper of the bracket- set of crates designed initally
//! for roguelike development (as RLTK) and later transitioned into a general
//! use crate.

/// prelude
pub mod prelude {
    pub use bracket_algorithm_traits::prelude::*;
    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;
    pub use bracket_terminal::prelude::*;
    pub use bracket_terminal::{add_wasm_support, embedded_resource, link_resource};
}

/// bracket-algorithm-traits provides traits for use in the bracket-pathfinding
/// and bracket-geometry
pub mod algorithm_traits {
    pub use bracket_algorithm_traits::prelude::*;
}

/// bracket-color provides a color system for use in the bracket-terminal
pub mod color {
    pub use bracket_color::prelude::*;
}
/// bracket-geometry provides some geometric primitives (Point, Point3D, Rect),
/// support functions and distance calculations. It also includes Bresenham's
/// line algorithm, a vector line algorithm, and Bresenham's Circle algorithm.
pub mod geometry {
    pub use bracket_geometry::prelude::*;
}

/// bracket-noise covers all the commonly used types of noise,
pub mod noise {
    pub use bracket_noise::prelude::*;
}
/// bracket-pathfinding (in conjunction with bracket-algorithm-traits) provides
/// pathfinding functionality. A-Star (A*) and Dijkstra are supported. It also
// provides field of view (FOV) functionality.
pub mod pathfinding {
    pub use bracket_pathfinding::prelude::*;
}

/// bracket-random provides a dice-oriented random number generation
pub mod random {
    pub use bracket_random::prelude::*;
}

/// bracket-terminal provides a virtual ASCII/Codepage-437 terminal (with
/// optional tile graphic support and layers), and a game loop
pub mod terminal {
    pub use bracket_terminal::prelude::*;
}
