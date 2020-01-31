// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crate::{GameState, Rltk};

mod keycodes;
pub use keycodes::VirtualKeyCode;

use pancurses::{initscr, noecho, resize_term, Window};

mod main_loop;
pub use main_loop::main_loop;

mod simple_console_backing;
mod sparse_console_backing;
pub use simple_console_backing::SimpleConsoleBackend;
pub use sparse_console_backing::SparseConsoleBackend;

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
        }
    }
}

pub struct PlatformGL {
    window: Window,
    color_map: Vec<CursesColor>,
}

pub mod shader {
    pub struct Shader {}
}

pub mod font {
    pub struct Font {
        pub tile_size: (u32, u32),
    }

    impl Font {
        pub fn load<S: ToString>(_filename: S, _tile_size: (u32, u32)) -> Font {
            Font { tile_size: (1, 1) }
        }

        pub fn setup_gl_texture(&mut self, _gl: &crate::hal::RltkPlatform) {}

        pub fn bind_texture(&self, _gl: &crate::hal::RltkPlatform) {}
    }
}

struct CursesColor {
    r: i16,
    g: i16,
    b: i16,
    rf: f32,
    gf: f32,
    bf: f32,
}

impl CursesColor {
    fn new(red: i16, green: i16, blue: i16) -> CursesColor {
        CursesColor {
            r: red,
            g: green,
            b: blue,
            rf: red as f32 / 1000.0,
            gf: green as f32 / 1000.0,
            bf: blue as f32 / 1000.0,
        }
    }
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
    _platform_hints: InitHints,
) -> crate::Rltk {
    let window = initscr();
    resize_term(height_pixels as i32 / 8, width_pixels as i32 / 8);
    noecho();
    window.nodelay(true);
    window.keypad(true);
    pancurses::start_color();
    pancurses::mousemask(
        pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION,
        std::ptr::null_mut(),
    );

    // Setup basic color mapping
    let mut color_map = Vec::new();
    for i in 0..16 {
        let color = pancurses::color_content(i);
        color_map.push(CursesColor::new(color.0, color.1, color.2));
    }

    let mut counter = 0;
    for bg in 0..16i16 {
        for fg in 0..16i16 {
            pancurses::init_pair(counter as i16, fg, bg);
            counter += 1;
        }
    }

    crate::Rltk {
        backend: super::RltkPlatform {
            platform: PlatformGL { window, color_map },
        },
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders: Vec::new(),
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

fn find_nearest_color(color: crate::RGB, map: &[CursesColor]) -> i16 {
    let mut result = -1;
    let mut best_diff = std::f32::MAX;

    for (i, cc) in map.iter().enumerate() {
        let diff_r = f32::abs(color.r - cc.rf);
        let diff_g = f32::abs(color.g - cc.gf);
        let diff_b = f32::abs(color.b - cc.bf);
        let total_diff = diff_r + diff_g + diff_b;

        if total_diff < best_diff {
            result = i as i16;
            best_diff = total_diff;
        }
    }

    result
}

pub fn log(s: &str) {
    println!("{}", s);
}