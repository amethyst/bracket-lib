#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
//! This crate is part of the `bracket-lib` family.
//!
//! It provides traits for you to implement on your map (and other geometric constructs),
//! translating them into a format that works with the `bracket-pathfinding` and `bracket-geometry`
//! systems.
//!
//! It is a separate crate so that both can depend upon it.
//!
//! Defaults are provided to get you up and running quickly, but may (should!) be overridden if you
//! don't want to use my default array striding.
//!
//! For example:
//! ```rust
//! use bracket_algorithm_traits::prelude::{BaseMap, Algorithm2D};
//! use bracket_geometry::prelude::Point;
//!
//! struct TestMap{};
//! impl BaseMap for TestMap {}
//! impl Algorithm2D for TestMap{
//!     fn dimensions(&self) -> Point {
//!         Point::new(2, 2)
//!     }
//! }
//! ```

mod algorithm2d;
mod algorithm3d;
mod basemap;

/// Exported traits
pub mod prelude {
    /// `Algorithm2D` support
    pub use crate::algorithm2d::Algorithm2D;

    /// `Algorithm3D` support
    pub use crate::algorithm3d::Algorithm3D;

    /// `BaseMap` support
    pub use crate::basemap::BaseMap;
}