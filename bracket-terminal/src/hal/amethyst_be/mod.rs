// Platform to integrate into Amethyst
mod font;
pub use font::*;
mod init;
mod shader;
pub use init::*;
pub use shader::*;
mod mainloop;
pub use mainloop::*;

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
    pub frame_sleep_time: Option<f32>,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            frame_sleep_time: None,
        }
    }
}

pub fn log(s: &str) {
    println!("{}", s);
}
