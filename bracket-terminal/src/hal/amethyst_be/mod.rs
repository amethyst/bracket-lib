// Platform to integrate into Amethyst
pub mod font;
mod init;
pub mod shader;
pub use init::*;
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
