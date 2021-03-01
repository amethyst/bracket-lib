#[cfg(feature = "parsing")]
#[macro_use]
extern crate lazy_static;

mod random;

#[cfg(feature = "parsing")]
mod parsing;

#[cfg(target_arch = "wasm32")]
mod js_seed;

mod iterators;

pub mod prelude {
    pub use crate::random::*;

    #[cfg(feature = "parsing")]
    pub use crate::parsing::*;

    pub use crate::iterators::*;
}