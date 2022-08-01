//! The `bracket-embedding` crate is used to provide resource embedding.
//! This allows you to include binary assets inside your program when shipping,
//! with no external files. This can be especially useful for WASM builds.
//! 
//! For example:
//! 
//! ```rust
//! use bracket_embedding::prelude::*;
//! 
//! embedded_resource!(SOURCE_FILE, "embedding.rs");
//! 
//! fn main() {
//!    // This helper macro links the above embedding, allowing it to be accessed as a resource from various parts of the program.
//!    link_resource!(SOURCE_FILE, "embedding.rs");
//! }
//! ```
//! 
//! This crate isn't very useful on its own, but is heavily used by the other parts of `bracket-lib`.

#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::needless_doctest_main)]
mod embedding;

pub mod prelude {
    pub use crate::embedding::*;
    pub use crate::{embedded_resource, link_resource};
}

/// Declare an embedded resource.
/// 
/// # Arguments
/// 
/// * `resource_name` - a constant that will represent the resource.
/// * `filename` - the path to the file to embed.
/// 
/// Once embedded, you need to use `link_resource` to make it available.
/// 
/// # Example
/// 
/// ```rust
/// use bracket_embedding::prelude::*;
/// 
/// embedded_resource!(SOURCE_FILE, "embedding.rs");
/// 
/// fn main() {
///    // This helper macro links the above embedding, allowing it to be accessed as a resource from various parts of the program.
///    link_resource!(SOURCE_FILE, "embedding.rs");
/// }
/// ```
#[macro_export]
macro_rules! embedded_resource {
    ($resource_name : ident, $filename : expr) => {
        const $resource_name: &'static [u8] = include_bytes!($filename);
    };
}

/// Link an embedded resource, making it available to `bracket-lib` via the resources
/// system.
/// 
/// # Arguments
/// 
/// * `resource_name` - a constant that will represent the resource.
/// * `filename` - the path to the file to embed.
/// 
/// The resource must be previously declared with `embedded_resource!`.
/// 
/// # Example
/// 
/// ```rust
/// use bracket_embedding::prelude::*;
/// 
/// embedded_resource!(SOURCE_FILE, "embedding.rs");
/// 
/// fn main() {
///    // This helper macro links the above embedding, allowing it to be accessed as a resource from various parts of the program.
///    link_resource!(SOURCE_FILE, "embedding.rs");
/// }
/// ```
#[macro_export]
macro_rules! link_resource {
    ($resource_name : ident, $filename : expr) => {
        EMBED
            .lock()
            .add_resource($filename.to_string(), $resource_name);
    };
}
