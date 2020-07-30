pub use bracket_lib::prelude::rex;
pub use bracket_lib::prelude::BError as RltkError;
pub use bracket_lib::prelude::BTerm as Rltk;
pub use bracket_lib::prelude::BTermBuilder as RltkBuilder;
pub use bracket_lib::prelude::*;

#[macro_export]
macro_rules! link_resource {
    ($resource_name : ident, $filename : expr) => {
        rltk::EMBED
            .lock()
            .add_resource($filename.to_string(), $resource_name);
    };
}

pub mod prelude {
    pub use crate::link_resource;
    pub use crate::*;
}
