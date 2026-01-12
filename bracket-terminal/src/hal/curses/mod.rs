// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crate::prelude::BTerm;
use crate::BResult;
use parking_lot::Mutex;

pub use winit::event::VirtualKeyCode;

use pancurses::Window;

mod main_loop;
pub use main_loop::main_loop;

mod font;
pub use font::*;
mod init;
mod shader;
pub use init::init_raw;
pub use shader::*;
mod color;
pub use color::*;
mod scancode_helper;
pub(crate) use scancode_helper::char_to_keycode;
pub use scancode_helper::virtual_key_code_to_scan;

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
    pub frame_sleep_time: Option<f32>,
    pub fitscreen: bool,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            frame_sleep_time: None,
            fitscreen: false,
        }
    }
}

pub struct PlatformGL {
    window: Option<Window>,
    color_map: Vec<CursesColor>,
    pub frame_sleep_time: Option<u64>,
}

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        window: None,
        color_map: Vec::new(),
        frame_sleep_time: None
    });
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

pub fn log(s: &str) {
    println!("{}", s);
}
