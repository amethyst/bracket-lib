//! This crate is part of the `bracket-lib` family.
//!
//! It provides point (2D and 3D), rectangle, line and circle plotting functionality.
//! It uses UltraViolet behind the scenes for very fast calculations. If you enable the
//! `serde` feature flag, it implements serialization/deserialization of the primitive types.
//!
//! For example:
//! ```rust
//! use bracket_geometry::prelude::*;
//! let my_point = Point::new(5,6);
//! println!("{:?}", my_point);
//! ```
//!
//! ```rust
//! use bracket_geometry::prelude::*;
//!
//! let my_point3 = Point3::new(5,6,7);
//! println!("{:?}", my_point3);
//! ```
//!
//! ```rust
//! use bracket_geometry::prelude::*;
//! let my_rect = Rect::with_size(1, 1, 10, 10);
//! let center = my_rect.center();
//! println!("{:?}", center);
//! ```
//!
//! Line examples:
//!
//! ```rust
//! use bracket_geometry::prelude::*;
//! let bresenham_line = line2d(LineAlg::Bresenham, Point::new(1,1), Point::new(5,5));
//! println!("{:?}", bresenham_line);
//! ```
//!
//! ```rust
//! use bracket_geometry::prelude::*;
//! for point in Bresenham::new(Point::new(1,1), Point::new(5,5)) {
//!     println!("{:?}", point);
//! }
//! ```
//!
//! Circle example:
//!
//! ```rust
//! use bracket_geometry::prelude::*;
//! for point in BresenhamCircle::new(Point::new(10,10), 5) {
//!     println!("{:?}", point);
//! }
//! ```
//! 
//! Distance examples:
//! 
//! ```rust
//! use bracket_geometry::prelude::*;
//! println!("{:?}", DistanceAlg::Pythagoras.distance2d(Point::new(0,0), Point::new(5,5)));
//! println!("{:?}", DistanceAlg::PythagorasSquared.distance2d(Point::new(0,0), Point::new(5,5)));
//! println!("{:?}", DistanceAlg::Manhattan.distance2d(Point::new(0,0), Point::new(5,5)));
//! println!("{:?}", DistanceAlg::Chebyshev.distance2d(Point::new(0,0), Point::new(5,5)));
//! ```

mod line_bresenham;
mod lines;
mod point;
mod point3;
mod line_vector;
mod circle_bresenham;
mod rect;
mod distance;
mod angles;

pub mod prelude {
    pub use crate::angles::*;
    pub use crate::point::*;
    pub use crate::point3::*;
    pub use crate::lines::*;
    pub use crate::line_bresenham::*;
    pub use crate::line_vector::*;
    pub use crate::circle_bresenham::*;
    pub use crate::rect::*;
    pub use crate::distance::*;
}
