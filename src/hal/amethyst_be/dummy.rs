use super::super::RltkPlatform;
use super::font::Font;
use super::shader::Shader;

pub struct SimpleConsoleBackend {
}

impl SimpleConsoleBackend {
    pub fn new(_gl: &RltkPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend{}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &RltkPlatform,
        _height: u32,
        _width: u32,
        _tiles: &[crate::Tile],
        _offset_x: f32,
        _offset_y: f32,
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &Font,
        _shader: &Shader,
        _platform: &RltkPlatform,
        _width: u32,
        _height: u32,
    ) {
    }
}

pub struct SparseConsoleBackend {
}

impl SparseConsoleBackend {
    pub fn new(_gl: &RltkPlatform, _width: usize, _height: usize) -> SparseConsoleBackend {
        SparseConsoleBackend{}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &RltkPlatform,
        _height: u32,
        _width: u32,
        _offset_x: f32,
        _offset_y: f32,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &Font,
        _shader: &Shader,
        _platform: &RltkPlatform,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }
}