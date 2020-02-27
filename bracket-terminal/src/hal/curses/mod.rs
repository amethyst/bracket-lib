// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crate::prelude::BTerm;
use crate::Result;
use std::sync::Mutex;

pub use winit::event::VirtualKeyCode;

use pancurses::Window;

mod main_loop;
pub use main_loop::main_loop;

mod init;
pub mod shader;
pub mod font;
pub use init::init_raw;
mod color;
pub use color::*;

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
