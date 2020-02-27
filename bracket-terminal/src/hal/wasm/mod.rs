pub use winit::event::VirtualKeyCode;
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
use std::sync::Mutex;
use std::any::Any;

pub type GlCallback = fn(&mut dyn Any, &glow::Context);

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
    pub frame_sleep_time: Option<f32>,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            frame_sleep_time: None,
        }
    }
}

pub struct PlatformGL {
    pub gl: Option<glow::Context>,
    pub quad_vao: Option<glow::WebVertexArrayKey>,
    pub gl_callback: Option<GlCallback>
}

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        gl: None,
        quad_vao: None,
        gl_callback : None
    });
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

pub enum ConsoleBacking {
    Simple { backing: SimpleConsoleBackend },
    Sparse { backing: SparseConsoleBackend },
}

lazy_static! {
    static ref CONSOLE_BACKING: Mutex<Vec<ConsoleBacking>> = Mutex::new(Vec::new());
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
