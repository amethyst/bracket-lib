// Platform to integrate into Amethyst
pub mod shader;
pub mod font;
mod init;
pub use init::*;
mod mainloop;
pub use mainloop::*;
mod dummy;
pub use dummy::*;
