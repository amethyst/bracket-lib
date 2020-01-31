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

pub struct PlatformGL {
    pub gl: glow::Context,
    pub quad_vao: u32,
    pub context_wrapper: Option<WrappedContext>,
    pub backing_buffer: super::Framebuffer,
}

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
        }
    }
}

pub fn log(s: &str) {
    println!("{}", s);
}