#[macro_use]
extern crate lazy_static;
mod bterm;
mod consoles;
mod gamestate;
mod hal;
mod initializer;
mod input;
pub mod rex;
pub use bracket_embedding::prelude::{EMBED, link_resource, embedded_resource};

pub type BResult<T> = anyhow::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub(crate) use input::clear_input_state;
pub type FontCharType = u16;
pub use consoles::console;

#[cfg(all(
    any(feature = "opengl", feature = "webgpu"),
    any(feature = "crossterm", feature = "curses")
))]
compile_error!("Default features (opengl) must be disabled for other back-ends");

pub mod prelude {

    pub use crate::bterm::*;
    pub use crate::consoles::*;
    pub use bracket_embedding::prelude::{EMBED, link_resource, embedded_resource};
    pub use crate::gamestate::GameState;
    pub use crate::hal::{init_raw, BTermPlatform, Font, InitHints, Shader, BACKEND};
    pub use crate::initializer::*;
    pub use crate::input::{BEvent, Input, INPUT};
    pub use crate::rex;
    pub use crate::rex::*;
    pub use crate::BResult;
    pub use crate::FontCharType;
    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub type BError = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    pub use glutin::event::VirtualKeyCode;

    #[cfg(all(feature = "webgpu", not(feature = "opengl")))]
    pub use crate::hal::VirtualKeyCode;

    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    pub use crate::hal::GlCallback;

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
