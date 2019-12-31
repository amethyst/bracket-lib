// Platform to integrate into Amethyst
pub mod font;
mod init;
pub mod shader;
pub use init::*;
mod mainloop;
pub use mainloop::*;
mod dummy;
pub use dummy::*;
