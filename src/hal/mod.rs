// Enable modules based on target architecture
#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

mod framebuffer;
pub use framebuffer::Framebuffer;

mod shader;
pub use shader::Shader;

pub mod font;

/// Provides a base abstract platform for RLTK to run on, with specialized content.
pub struct RltkPlatform {
    pub gl: glow::Context,
    pub platform : PlatformGL
}


#[cfg(not(target_arch = "wasm32"))]
pub fn log<S:ToString>(message: S) {
    println!("{}", message.to_string());
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {   
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}