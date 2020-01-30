// Enable modules based on target architecture
#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
mod native;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
pub use native::*;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
mod wasm;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
pub use wasm::*;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
mod framebuffer;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
pub use framebuffer::Framebuffer;

#[cfg(not(feature = "opengl"))]
#[cfg(all(not(feature = "opengl"), feature = "curses"))]
mod curses;

#[cfg(all(not(feature = "opengl"), feature = "curses"))]
pub use curses::*;

#[cfg(not(feature = "opengl"))]
#[cfg(all(not(feature = "opengl"), feature = "crossterm"))]
mod crossterm_be;

#[cfg(all(not(feature = "opengl"), feature = "crossterm"))]
pub use crossterm_be::*;

#[cfg(all(
    not(feature = "opengl"),
    any(feature = "amethyst_engine_vulkan", feature = "amethyst_engine_metal")
))]
mod amethyst_be;

#[cfg(all(
    not(feature = "opengl"),
    any(feature = "amethyst_engine_vulkan", feature = "amethyst_engine_metal")
))]
pub use amethyst_be::*;

#[cfg(all(
    not(feature = "opengl"),
    not(feature = "curses"),
    not(feature = "amethyst_engine_vulkan"),
    not(feature = "amethyst_engine_metal"),
    not(feature = "crossterm")
))]
mod dummy;

#[cfg(all(
    not(feature = "opengl"),
    not(feature = "curses"),
    not(feature = "amethyst_engine_vulkan"),
    not(feature = "amethyst_engine_metal"),
    not(feature = "crossterm")
))]
pub use dummy::*;

pub use shader::Shader;

/// Provides a base abstract platform for RLTK to run on, with specialized content.
pub struct RltkPlatform {
    pub platform: PlatformGL,
}

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
pub fn log<S: ToString>(message: S) {
    println!("{}", message.to_string());
}

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(not(feature = "opengl"))]
pub fn log<S: ToString>(message: S) {
    println!("{}", message.to_string());
}
