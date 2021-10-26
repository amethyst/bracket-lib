mod platform;
pub use platform::*;
mod init;
pub use init::*;
mod font;
pub use font::*;
mod shader;
pub use shader::*;
mod backend;
pub use backend::*;
mod mainloop;
pub use mainloop::*;
pub use winit::event::VirtualKeyCode;
mod backing;
pub(crate) use backing::*;

pub fn log(s: &str) {
    println!("{}", s);
}
