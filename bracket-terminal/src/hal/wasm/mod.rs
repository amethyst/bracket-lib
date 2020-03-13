pub use winit::event::VirtualKeyCode;
mod init;
pub mod shader_strings;
pub use init::*;
mod events;
pub use events::*;
mod mainloop;
pub use mainloop::*;
use parking_lot::Mutex;
use std::any::Any;
use crate::hal::{SimpleConsoleBackend, SparseConsoleBackend, FancyConsoleBackend, SpriteConsoleBackend, ConsoleBacking};

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
    pub backing_buffer: Option<super::Framebuffer>,
    pub gl_callback: Option<GlCallback>,
}

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        gl: None,
        quad_vao: None,
        gl_callback: None,
        backing_buffer: None
    });
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

lazy_static! {
    static ref CONSOLE_BACKING: Mutex<Vec<ConsoleBacking>> = Mutex::new(Vec::new());
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
