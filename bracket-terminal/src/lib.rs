#[macro_use]
extern crate lazy_static;
mod bterm;
mod consoles;
pub mod embedding;
mod gamestate;
mod hal;
mod initializer;
mod input;
pub mod rex;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Result<T> = core::result::Result<T, Error>;
pub(crate) use input::clear_input_state;
pub type FontCharType = u16;
pub use consoles::console;

#[cfg(all(feature = "opengl", any(feature = "crossterm", any(feature = "curses", any(feature = "amethyst_engine_vulkan", feature = "amethyst_engine_metal")))))]
compile_error!("Default features (opengl) must be disabled for other back-ends");

pub mod prelude {

    pub use crate::bterm::*;
    pub use crate::consoles::*;
    pub use crate::embedding;
    pub use crate::embedding::EMBED;
    pub use crate::gamestate::GameState;
    pub use crate::hal::{init_raw, BTermPlatform, Font, InitHints, Shader, BACKEND};
    pub use crate::initializer::*;
    pub use crate::input::{BEvent, Input, INPUT};
    pub use crate::rex;
    pub use crate::rex::*;
    pub use crate::FontCharType;
    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub type BError = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    pub use glutin::event::VirtualKeyCode;

    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    pub use crate::hal::GlCallback;

    #[cfg(all(
        not(feature = "opengl"),
        any(feature = "amethyst_engine_vulkan", feature = "amethyst_engine_metal")
    ))]
    pub use amethyst::input::VirtualKeyCode;

    #[cfg(target_arch = "wasm32")]
    pub use crate::hal::VirtualKeyCode;

    #[cfg(feature = "curses")]
    pub use crate::hal::VirtualKeyCode;

    #[cfg(feature = "crossterm")]
    pub use crate::hal::VirtualKeyCode;
}

#[macro_export]
macro_rules! add_wasm_support {
    () => {
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::prelude::*;

        #[cfg(target_arch = "wasm32")]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
        pub fn wasm_main() {
            main().expect("Error in main");
        }
    };
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
