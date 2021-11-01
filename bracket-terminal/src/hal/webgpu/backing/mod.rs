//! Provides helper-code to map between console representations and wgpu.

mod simple_console_backing;
pub(crate) use simple_console_backing::*;
mod sparse_console_backing;
pub(crate) use sparse_console_backing::*;
mod fancy_console_backing;
pub(crate) use fancy_console_backing::*;
mod sprite_console_backing;
pub(crate) use sprite_console_backing::*;
pub(crate) mod index_array_helper;
pub(crate) mod vertex_array_helper;

pub(crate) enum ConsoleBacking {
    Simple { backing: SimpleConsoleBackend },
    Sparse { backing: SparseConsoleBackend },
    Fancy { backing: FancyConsoleBackend },
    Sprite { backing: SpriteConsoleBackend },
}
