// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crate::{GameState, Rltk};

mod keycodes;
pub use keycodes::VirtualKeyCode;

use pancurses::{initscr, Window, noecho, resize_term};

mod main_loop;
pub use main_loop::main_loop;

mod simple_console_backing;
mod sparse_console_backing;
pub use simple_console_backing::SimpleConsoleBackend;
pub use sparse_console_backing::SparseConsoleBackend;

pub struct PlatformGL {
    window : Window
}

pub mod shader {
    pub struct Shader{}
}

pub mod font {
    pub struct Font{
        pub tile_size: (u32, u32)
    }

    impl Font {
        pub fn load<S: ToString>(_filename: S, _tile_size: (u32, u32)) -> Font {
            Font{
                tile_size : (1, 1)
            }
        }

        pub fn setup_gl_texture(&mut self, _gl: &crate::hal::RltkPlatform) {

        }

        pub fn bind_texture(&self, _gl: &crate::hal::RltkPlatform) {

        }
    }
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
) -> crate::Rltk 
{
    let window = initscr();
    resize_term(height_pixels as i32/8, width_pixels as i32/8);
    noecho();
    window.nodelay(true);
    window.keypad(true);
    pancurses::start_color();
    pancurses::mousemask(pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION, std::ptr::null_mut());

    crate::Rltk {
        backend: super::RltkPlatform { 
            platform: PlatformGL{
                window
            } 
        },
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders : Vec::new(),
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    }
}

fn find_nearest_color(color : crate::RGB) -> i16 {
    0
}
