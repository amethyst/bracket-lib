use std::sync::Mutex;

pub use winit::event::VirtualKeyCode;

mod main_loop;
pub use main_loop::*;

pub mod font;
pub mod shader;
mod init;
pub use init::*;

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

pub struct PlatformGL {
    old_width: u16,
    old_height: u16,
    pub frame_sleep_time: Option<u64>,
}

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        old_width: 0,
        old_height: 0,
        frame_sleep_time: None
    });
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

pub fn log(s: &str) {
    println!("{}", s);
}
