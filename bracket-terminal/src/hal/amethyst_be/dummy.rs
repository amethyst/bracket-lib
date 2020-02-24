use crate::Result;
use crate::hal::BTermPlatform;
use super::font::Font;
use super::shader::Shader;

pub struct SimpleConsoleBackend {}

impl SimpleConsoleBackend {
    pub fn new(_gl: &BTermPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend {}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &BTermPlatform,
        _height: u32,
        _width: u32,
        _tiles: &[crate::prelude::Tile],
        _offset_x: f32,
        _offset_y: f32,
        _scale: f32,
        _scale_center: (i32, i32),
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &Font,
        _shader: &Shader,
        _platform: &BTermPlatform,
        _width: u32,
        _height: u32,
    ) -> Result<()> {
        Ok(())
    }
}

pub struct SparseConsoleBackend {}

impl SparseConsoleBackend {
    pub fn new(_gl: &BTermPlatform, _width: usize, _height: usize) -> SparseConsoleBackend {
        SparseConsoleBackend {}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &BTermPlatform,
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
        _font: &Font,
        _shader: &Shader,
        _platform: &BTermPlatform,
        _tiles: &[crate::sparse_console::SparseTile],
    ) -> Result<()> {
        Ok(())
    }
}
