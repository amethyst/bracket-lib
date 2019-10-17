mod quadrender;
pub use quadrender::*;
pub mod shader_strings;
mod init;
pub use init::*;

use glutin::{
    dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};

pub struct PlatformGL {
    pub quad_vao: u32,
    pub context_wrapper: Option<WrappedContext>,
    pub backing_buffer: super::Framebuffer,
}

pub struct WrappedContext {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}