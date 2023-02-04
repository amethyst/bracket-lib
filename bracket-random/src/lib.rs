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

pub mod rand {
    pub use rand::*;
}
