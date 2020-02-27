#[macro_use]
extern crate lazy_static;
mod bterm;
mod codepage437;
mod command_buffer;
pub mod console;
pub mod embedding;
mod gamestate;
mod gui_helpers;
mod hal;
mod initializer;
mod multi_tile_sprite;
pub mod rex;
mod simple_console;
mod sparse_console;
mod textblock;

pub(crate) type Error = Box<dyn std::error::Error>;
pub(crate) type Result<T> = core::result::Result<T, Error>;

pub mod prelude {

    pub use crate::bterm::*;
    pub use crate::codepage437::*;
    pub use crate::command_buffer::*;
    pub use crate::console;
    pub use crate::console::{Console, Tile};
    pub use crate::embedding;
    pub use crate::embedding::EMBED;
    pub use crate::gamestate::GameState;
    pub use crate::gui_helpers::*;
    pub use crate::hal::{
        font, init_raw, BTermPlatform, InitHints, Shader, BACKEND
    };
    pub use crate::initializer::*;
    pub use crate::multi_tile_sprite::*;
    pub use crate::rex;
    pub use crate::rex::*;
    pub use crate::simple_console::SimpleConsole;
    pub use crate::sparse_console::SparseConsole;
    pub use crate::textblock::*;
    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub type BError = std::result::Result<(), Box<dyn std::error::Error>>;

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
        //#[cfg(target_arch = "wasm32")]
        //extern crate console_error_panic_hook;

        //#[cfg(target_arch = "wasm32")]
        //use std::panic;

        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::prelude::*;

        #[cfg(target_arch = "wasm32")]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
        pub fn wasm_main() {
            //panic::set_hook(Box::new(console_error_panic_hook::hook));
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
            .unwrap()
            .add_resource($filename.to_string(), $resource_name);
    };
}
