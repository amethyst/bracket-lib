pub use bracket_lib::prelude::*;
pub use bracket_lib::prelude::BTerm as Rltk;
pub use bracket_lib::prelude::BTermBuilder as RltkBuilder;
pub use bracket_lib::prelude::rex;
pub use bracket_lib::prelude::BError as RltkError;

#[macro_export]
macro_rules! link_resource {
    ($resource_name : ident, $filename : expr) => {
        rltk::EMBED
            .lock()
            .unwrap()
            .add_resource($filename.to_string(), $resource_name);
    };
}

pub mod prelude {
    pub use crate::*;
    pub use crate::link_resource;
}