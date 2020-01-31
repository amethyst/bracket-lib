mod keycodes;
pub use keycodes::*;
mod quadrender;
pub use quadrender::*;
mod init;
pub mod shader_strings;
pub use init::*;
mod events;
pub use events::*;
mod mainloop;
pub use mainloop::*;
mod simple_console_backing;
pub use simple_console_backing::*;
mod sparse_console_backing;
pub use sparse_console_backing::*;
pub mod font;
pub mod shader;

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
        }
    }
}

pub struct PlatformGL {
    pub gl: glow::Context,
    pub context_wrapper: Option<WrappedContext>,
    pub quad_vao: glow::WebVertexArrayKey,
}

pub struct WrappedContext {}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}