use parking_lot::Mutex;

pub use winit::event::VirtualKeyCode;

mod main_loop;
pub use main_loop::*;

mod font;
pub use font::*;
mod init;
mod shader;
pub use init::*;
pub use shader::*;
mod scancode_helper;
pub use scancode_helper::{keycode_to_key, virtual_key_code_to_scan};

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
