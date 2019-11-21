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

/*use glutin::{
    dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};*/

pub struct PlatformGL {
    pub quad_vao: u32,
    pub context_wrapper: Option<WrappedContext>
}

pub struct WrappedContext {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}
