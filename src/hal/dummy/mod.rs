// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crate::{GameState, Rltk};

mod keycodes;
pub use keycodes::VirtualKeyCode;

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

        pub fn setup_gl_texture(&mut self, _gl: &crate::hal::RltkPlatform) {}

        pub fn bind_texture(&self, _gl: &crate::hal::RltkPlatform) {}
    }
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
) -> crate::Rltk {
    crate::Rltk {
        backend: super::RltkPlatform {
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

pub fn main_loop<GS: GameState>(mut _rltk: Rltk, mut _gamestate: GS) {}

pub struct SimpleConsoleBackend {}

impl SimpleConsoleBackend {
    pub fn new(_gl: &super::RltkPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend {}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::RltkPlatform,
        _height: u32,
        _width: u32,
        _tiles: &[crate::Tile],
        _offset_x: f32,
        _offset_y: f32,
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        _platform: &super::RltkPlatform,
        _width: u32,
        _height: u32,
    ) {
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
