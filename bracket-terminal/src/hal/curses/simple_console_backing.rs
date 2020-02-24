use crate::Result;
use crate::prelude::{BTermPlatform, to_char, Tile};
use super::find_nearest_color;
use super::font;
use super::shader;
use std::convert::TryInto;

pub struct SimpleConsoleBackend {
    tiles: Vec<Tile>,
}

impl SimpleConsoleBackend {
    pub fn new(_gl: &BTermPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend { tiles: Vec::new() }
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &BTermPlatform,
        _height: u32,
        _width: u32,
        tiles: &[Tile],
        _offset_x: f32,
        _offset_y: f32,
        _scale: f32,
        _scale_center: (i32, i32),
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
        platform: &BTermPlatform,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let window = &platform.platform.window;
        let mut idx = 0;
        for y in 0..height {
            for x in 0..width {
                let t = &self.tiles[idx];
                let cp_fg = find_nearest_color(t.fg, &platform.platform.color_map);
                let cp_bg = find_nearest_color(t.bg, &platform.platform.color_map);
                let pair = (cp_bg * 16) + cp_fg;
                window.attrset(pancurses::COLOR_PAIR(pair.try_into()?));
                window.mvaddch(
                    height as i32 - (y as i32 + 1),
                    x as i32,
                    to_char(t.glyph),
                );
                idx += 1;
            }
        }
        Ok(())
    }
}
