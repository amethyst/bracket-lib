use super::super::RltkPlatform;
use super::font;
use super::shader;
use crate::RGB;
use crossterm::style::Print;
use crossterm::{cursor, queue};
use std::io::{stdout, Write};

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
        _platform: &RltkPlatform,
        width: u32,
        height: u32,
    ) {
        let mut idx = 0;
        let mut last_bg = RGB::new();
        let mut last_fg = RGB::new();
        for y in 0..height {
            queue!(stdout(), cursor::MoveTo(0, height as u16 - (y as u16 + 1)))
                .expect("Command fail");
            for _x in 0..width {
                let t = &self.tiles[idx];
                if t.fg != last_fg {
                    queue!(
                        stdout(),
                        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                            r: (t.fg.r * 255.0) as u8,
                            g: (t.fg.g * 255.0) as u8,
                            b: (t.fg.b * 255.0) as u8,
                        })
                    )
                    .expect("Command fail");
                    last_fg = t.fg;
                }
                if t.bg != last_bg {
                    queue!(
                        stdout(),
                        crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb {
                            r: (t.bg.r * 255.0) as u8,
                            g: (t.bg.g * 255.0) as u8,
                            b: (t.bg.b * 255.0) as u8,
                        })
                    )
                    .expect("Command fail");
                    last_bg = t.bg;
                }
                queue!(stdout(), Print(crate::to_char(t.glyph))).expect("Command fail");
                idx += 1;
            }
        }
    }
}
