use super::super::RltkPlatform;
use super::find_nearest_color;
use super::font;
use super::shader;

pub struct SparseConsoleBackend {
    width: u32,
    height: u32,
}

impl SparseConsoleBackend {
    pub fn new(_gl: &RltkPlatform, width: usize, height: usize) -> SparseConsoleBackend {
        SparseConsoleBackend {
            width: width as u32,
            height: height as u32,
        }
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &RltkPlatform,
        height: u32,
        width: u32,
        _offset_x: f32,
        _offset_y: f32,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
        self.width = width;
        self.height = height;
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        platform: &RltkPlatform,
        tiles: &[crate::sparse_console::SparseTile],
    ) {
        let window = &platform.platform.window;
        for t in tiles.iter() {
            let x = t.idx as u32 % self.width;
            let y = t.idx as u32 / self.width;

            let cp_fg = find_nearest_color(t.fg, &platform.platform.color_map);
            let cp_bg = find_nearest_color(t.bg, &platform.platform.color_map);
            let pair = (cp_bg * 16) + cp_fg;
            window.attrset(pancurses::COLOR_PAIR(pair as u32));
            window.mvaddch(
                self.height as i32 - (y as i32 + 1),
                x as i32,
                crate::to_char(t.glyph),
            );
        }
    }
}
