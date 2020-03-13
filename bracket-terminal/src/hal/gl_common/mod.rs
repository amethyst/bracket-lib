mod framebuffer;

pub use framebuffer::*;
mod shader;
pub use shader::*;
mod font;
pub use font::*;
mod quadrender;
pub use quadrender::*;
mod vertex_array_helper;
pub(crate) use vertex_array_helper::*;
mod backing;
pub(crate) use backing::*;

#[cfg(not(target_arch = "wasm32"))]
mod types_native;

#[cfg(not(target_arch = "wasm32"))]
pub use types_native::*;

#[cfg(target_arch = "wasm32")]
mod types_wasm;

#[cfg(target_arch = "wasm32")]
pub use types_wasm::*;
