use super::super::RltkPlatform;
use super::font;
use super::shader;

pub struct SimpleConsoleBackend {
    tiles : Vec<crate::Tile>
}

impl SimpleConsoleBackend {
    pub fn new(_gl: &RltkPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend{
            tiles : Vec::new()
        }
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &RltkPlatform,
        _height: u32,
        _width: u32,
        tiles: &[crate::Tile],
        _offset_x: f32,
        _offset_y: f32,
    ) {
        self.tiles.clear();
        for t in tiles.iter() {
            self.tiles.push(*t);
        }
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        platform: &RltkPlatform,
        width: u32,
        height: u32,
    ) {
        let window = &platform.platform.window;
        let mut idx = 0;
        for y in 0..height {
            for x in 0..width {
                let t = &self.tiles[idx];
                window.mvaddch(height as i32 - y as i32, x as i32, crate::to_char(t.glyph));
                idx += 1;
            }            
        }
    }
}