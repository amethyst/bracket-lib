mod quadrender;
pub use quadrender::*;
mod init;
pub mod shader_strings;
pub use init::*;
mod mainloop;
pub use mainloop::*;
mod simple_console_backing;
pub use simple_console_backing::*;
mod sparse_console_backing;
pub use sparse_console_backing::*;
pub mod font;
pub mod shader;
use std::sync::Mutex;

lazy_static! {
    static ref BACKEND : Mutex<PlatformGL> = Mutex::new(PlatformGL{
        gl: None,
        quad_vao: None,
        context_wrapper: None,
        backing_buffer: None,
        frame_sleep_time: None
    });
}

pub struct PlatformGL {
    pub gl: Option<glow::Context>,
    pub quad_vao: Option<u32>,
    pub context_wrapper: Option<WrappedContext>,
    pub backing_buffer: Option<super::Framebuffer>,
    pub frame_sleep_time: Option<u64>,
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

pub struct WrappedContext {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
    pub gl_version: glutin::GlRequest,
    pub gl_profile: glutin::GlProfile,
    pub hardware_acceleration: bool,
    pub srgb: bool,
    pub frame_sleep_time: Option<f32>,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            gl_version: glutin::GlRequest::Latest,
            gl_profile: glutin::GlProfile::Core,
            hardware_acceleration: true,
            srgb: true,
            frame_sleep_time: None,
        }
    }
}

pub fn log(s: &str) {
    println!("{}", s);
}
