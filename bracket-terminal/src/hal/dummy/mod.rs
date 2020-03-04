// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crate::{BTerm, GameState};

mod keycodes;
pub use keycodes::VirtualKeyCode;

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

pub struct PlatformGL {}

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

        pub fn setup_gl_texture(&mut self, _gl: &crate::hal::BTermPlatform) {}

        pub fn bind_texture(&self, _gl: &crate::hal::BTermPlatform) {}
    }
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
    _platform_hints: InitHints,
) -> crate::BTerm {
    crate::BTerm {
        backend: super::BTermPlatform {
            platform: PlatformGL {},
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

pub fn main_loop<GS: GameState>(mut _BTerm: BTerm, mut _gamestate: GS) {}

pub struct SimpleConsoleBackend {}

impl SimpleConsoleBackend {
    pub fn new(_gl: &super::BTermPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend {}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::BTermPlatform,
        _height: u32,
        _width: u32,
        _tiles: &[crate::Tile],
        _offset_x: f32,
        _offset_y: f32,
        _scale: f32,
        _scale_center: (i32, i32),
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        _platform: &super::BTermPlatform,
        _width: u32,
        _height: u32,
    ) {
    }
}

pub struct SparseConsoleBackend {}

impl SparseConsoleBackend {
    pub fn new(_gl: &super::BTermPlatform, _width: usize, _height: usize) -> SparseConsoleBackend {
        SparseConsoleBackend {}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::BTermPlatform,
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
        _platform: &super::BTermPlatform,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }
}

pub fn log(s: &str) {
    println!("{}", s);
}
