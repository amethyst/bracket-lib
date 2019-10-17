// Enable modules based on target architecture
#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

/// Provides a base abstract platform for RLTK to run on, with specialized content.
pub struct RltkPlatform {
    pub gl: glow::Context,
    pub platform : PlatformGL
}
