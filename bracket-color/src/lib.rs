#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

//! This crate is part of the `bracket-lib` family.
//!
//! It provides RGB, HSV and` ColorPair` support for the bracket-lib library.
//! You can construct an RGB with `new`, `from_f32`, `from_u8` or `named`.
//! It exports a large number of named colors (the W3C web named colors) for easy access.
//! It also provides convenience functions such as `to_greyscale`, `desaturate` and `lerp`.
//!
//! For example:
//! ```rust
//! use bracket_color::prelude::*;
//!
//! let red = RGB::named(RED);
//! let blue = RGB::named(BLUE);
//! for lerp in 0 .. 100 {
//!     let lerp_by = lerp as f32 / 100.0;
//!     let color = red.lerp(blue, lerp_by);
//!     println!("{:?}", color);
//!     let gray = color.to_greyscale();
//!     println!("{:?}", gray);
//!     let desat = color.desaturate();
//!     println!("{:?}", desat);
//! }
//! ```
//!
//! If you use the `serde` feature flag, the exposed types are serializable/de-serializable.

#[cfg(feature = "palette")]
#[macro_use]
extern crate lazy_static;

/// Import color pair support
pub mod color_pair;
/// Import HSV color support
pub mod hsv;
/// Import Lerp as an iterator
pub mod lerpit;
/// Import library of named colors
pub mod named;
/// Import Palette support
#[cfg(feature = "palette")]
pub mod palette;
/// Import RGB color support
pub mod rgb;
/// Import RGBA color support
pub mod rgba;

/// Exports the color functions/types in the `prelude` namespace.
pub mod prelude {
    pub use crate::color_pair::*;
    pub use crate::hsv::*;
    pub use crate::lerpit::*;
    pub use crate::named::*;
    #[cfg(feature = "palette")]
    pub use crate::palette::*;
    pub use crate::rgb::*;
    pub use crate::rgba::*;
}
