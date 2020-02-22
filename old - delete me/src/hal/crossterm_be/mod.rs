// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crossterm::{
    execute,
    terminal::{size, SetSize},
};
use std::io::{stdout, Write};

mod keycodes;
pub use keycodes::VirtualKeyCode;

mod simple_console_backing;
pub use simple_console_backing::*;

mod main_loop;
pub use main_loop::*;

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

pub mod shader {
    pub struct Shader {}
}

pub mod font {
    pub struct Font {
        pub tile_size: (u32, u32),
    }

    impl Font {
        pub fn load<S: ToString>(_filename: S, _tile_size: (u32, u32)) -> Font {
            Font { tile_size: (0, 0) }
        }

        pub fn setup_gl_texture(&mut self, _gl: &crate::hal::RltkPlatform) {}

        pub fn bind_texture(&self, _gl: &crate::hal::RltkPlatform) {}
    }
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
    platform_hints: InitHints,
) -> crate::Rltk {
    let old_size = size().expect("Unable to get console size");
    println!("Old size: {:?}", old_size);
    println!("Resizing to {}x{}", 80, 50);
    execute!(
        stdout(),
        SetSize(width_pixels as u16 / 8, height_pixels as u16 / 8),
    )
    .expect("Console command fail");

    execute!(stdout(), crossterm::cursor::Hide).expect("Command fail");
    execute!(stdout(), crossterm::event::EnableMouseCapture).expect("Command fail");

    crate::Rltk {
        backend: super::RltkPlatform {
            platform: PlatformGL {
                old_width: old_size.0,
                old_height: old_size.1,
                frame_sleep_time: super::convert_fps_to_wait(platform_hints.frame_sleep_time),
            },
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

pub struct SparseConsoleBackend {}

impl SparseConsoleBackend {
    pub fn new(_gl: &super::RltkPlatform, _width: usize, _height: usize) -> SparseConsoleBackend {
        SparseConsoleBackend {}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::RltkPlatform,
        _height: u32,
        _width: u32,
        _offset_x: f32,
        _offset_y: f32,
        _scale: f32,
        _scale_center: (i32, i32),
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        _platform: &super::RltkPlatform,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }
}

pub fn log(s: &str) {
    println!("{}", s);
}
