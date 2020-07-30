mod astar;
mod dijkstra;
mod fieldofview;

pub mod prelude {
    pub use crate::astar::*;
    pub use crate::dijkstra::*;
    pub use crate::fieldofview::*;
    pub use bracket_algorithm_traits::prelude::*;
    pub use bracket_geometry::prelude::*;

    /// Since we use `SmallVec`, it's only polite to export it so you don't have to have multiple copies.
    pub use smallvec::{SmallVec, smallvec};
}
