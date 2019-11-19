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

pub struct PlatformGL {
    pub context_wrapper: Option<WrappedContext>,
}

pub struct WrappedContext {}
