mod embedding;

pub mod prelude {
    pub use crate::embedding::*;
    pub use crate::{embedded_resource, link_resource};
}

#[macro_export]
macro_rules! embedded_resource {
    ($resource_name : ident, $filename : expr) => {
        const $resource_name: &'static [u8] = include_bytes!($filename);
    };
}

#[macro_export]
macro_rules! link_resource {
    ($resource_name : ident, $filename : expr) => {
        EMBED
            .lock()
            .add_resource($filename.to_string(), $resource_name);
    };
}
