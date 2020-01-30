use super::super::RltkPlatform;
use super::font;
use super::shader;
use std::convert::TryInto;
use std::io::{stdout, Write};
use crossterm::{execute, Result, terminal::{ScrollUp, SetSize, size, Clear}};
use crossterm::{queue, QueueableCommand, cursor};
use crossterm::style::Print;

pub struct SimpleConsoleBackend {
    tiles: Vec<crate::Tile>,
}

impl SimpleConsoleBackend {
    pub fn new(_gl: &RltkPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend { tiles: Vec::new() }
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
        _platform: &RltkPlatform,
        width: u32,
        height: u32,
    ) {
        let mut idx = 0;
        for y in 0..height {
            queue!(stdout(), cursor::MoveTo(0, height as u16 - (y as u16 + 1)));
            for x in 0..width {
                let t = &self.tiles[idx];
                queue!(stdout(), crossterm::style::SetForegroundColor(
                    crossterm::style::Color::RGB(
                        r: (t.fg.red * 255.0) as u8,
                        g: (t.fg.green * 255.0) as u8,
                        b: (t.fg.blue * 255.0) as u8,
                    )
                ));
                queue!(stdout(), Print(crate::to_char(t.glyph)));
                idx += 1;
            }
        }
    }
}
