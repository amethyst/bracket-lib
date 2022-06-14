use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::{Point, Rect};

use super::{common_draw, ConsoleFrontEnd, TerminalGlyph};
use crate::{BracketContext, FontCharType};

pub struct VirtualConsole {
    pub width: i32,
    pub height: i32,
    pub terminal: Vec<TerminalGlyph>,
    pub clipping: Option<Rect>,
}

impl VirtualConsole {
    /// Creates a new virtual console of arbitrary dimensions.
    pub fn new(dimensions: Point) -> Self {
        let num_tiles: usize = (dimensions.x * dimensions.y) as usize;
        let mut console = VirtualConsole {
            width: dimensions.x,
            height: dimensions.y,
            terminal: Vec::with_capacity(num_tiles),
            clipping: None,
        };
        for _ in 0..num_tiles {
            console.terminal.push(TerminalGlyph::default());
        }
        console
    }

    /// Creates a new virtual console from a blob of text.
    /// Useful if you want to scroll through manuals!
    pub fn from_text(text: &str, width: usize) -> Self {
        let raw_lines = text.split('\n');
        let mut lines: Vec<String> = Vec::new();
        for line in raw_lines {
            let mut newline: String = String::from("");

            line.chars().for_each(|c| {
                newline.push(c);
                if newline.len() > width {
                    lines.push(newline.clone());
                    newline.clear();
                }
            });
            lines.push(newline.clone());
        }

        let num_tiles: usize = width * lines.len();
        let mut console = VirtualConsole {
            width: width as i32,
            height: lines.len() as i32,
            terminal: Vec::with_capacity(num_tiles),
            clipping: None,
        };
        //println!("{}x{}", console.width, console.height);

        for _ in 0..num_tiles {
            console.terminal.push(TerminalGlyph::default());
        }

        for (i, line) in lines.iter().enumerate() {
            console.print(0, i as i32, line);
        }

        console
    }

    /// Send a portion of the Virtual Console to a physical console, specifying both source and destination
    /// rectangles and the target console.
    pub fn print_sub_rect(
        &self,
        source: Rect,
        dest: Rect,
        target_layer: usize,
        context: &BracketContext,
    ) {
        let mut lock = context.terminals.lock();
        let target = &mut lock[target_layer];
        target.set_clipping(Some(dest));
        for y in dest.y1..dest.y2 {
            let source_y = y + source.y1 - dest.y1;
            for x in dest.x1..dest.x2 {
                let source_x = x + source.x1 - dest.x1;
                if let Some(idx) = self.try_at(source_x, source_y) {
                    let t = self.terminal[idx];
                    if t.glyph > 0 {
                        target.set(x, y, t.foreground.into(), t.background.into(), t.glyph);
                    }
                }
            }
        }
        target.set_clipping(None);
    }

    fn at(&self, x: i32, y: i32) -> usize {
        if let Ok(pos) = (((self.height as i32 - 1 - y) * self.width as i32) + x).try_into() {
            pos
        } else {
            0
        }
    }
}

impl ConsoleFrontEnd for VirtualConsole {
    fn get_char_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    fn get_pixel_size(&self) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn at(&self, x: i32, y: i32) -> usize {
        self.at(x, y)
    }

    fn get_clipping(&self) -> Option<crate::consoles::Rect> {
        self.clipping
    }

    fn set_clipping(&mut self, clipping: Option<crate::consoles::Rect>) {
        self.clipping = clipping;
    }

    fn cls(&mut self) {
        self.terminal.iter_mut().for_each(|c| c.glyph = 32);
    }

    fn cls_bg(&mut self, color: RGBA) {
        self.terminal
            .iter_mut()
            .for_each(|c| c.background = color.as_rgba_f32());
    }

    fn set(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: FontCharType) {
        if let Some(idx) = self.try_at(x, y) {
            self.terminal[idx] = TerminalGlyph {
                glyph,
                foreground: fg.as_rgba_f32(),
                background: bg.as_rgba_f32(),
            };
        }
    }

    fn set_bg(&mut self, x: i32, y: i32, bg: RGBA) {
        if let Some(idx) = self.try_at(x, y) {
            self.terminal[idx].background = bg.as_rgba_f32();
        }
    }

    fn print(&mut self, x: i32, y: i32, text: &str) {
        common_draw::print(self, x, y, text);
    }

    fn print_color(&mut self, x: i32, y: i32, text: &str, foreground: RGBA, background: RGBA) {
        common_draw::print_color(self, x, y, text, foreground, background);
    }

    fn printer(
        &mut self,
        context: &BracketContext,
        x: i32,
        y: i32,
        output: &str,
        align: crate::consoles::TextAlign,
        background: Option<RGBA>,
    ) {
        common_draw::printer(self, context, x, y, output, align, background);
    }

    fn print_centered(&mut self, y: i32, text: &str) {
        self.print(
            (self.width as i32 / 2) - (text.to_string().len() / 2) as i32,
            y,
            text,
        );
    }

    fn print_centered_at(&mut self, x: i32, y: i32, text: &str) {
        self.print(x - (text.to_string().len() / 2) as i32, y, text);
    }

    fn print_color_centered(&mut self, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(
            (self.width as i32 / 2) - (text.to_string().len() / 2) as i32,
            y,
            text,
            fg,
            bg,
        );
    }

    fn print_color_centered_at(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(x - (text.to_string().len() / 2) as i32, y, text, fg, bg);
    }

    fn print_right(&mut self, x: i32, y: i32, text: &str) {
        let len = text.len() as i32;
        let actual_x = x - len;
        self.print(actual_x, y, text);
    }

    fn print_color_right(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        let len = text.len() as i32;
        let actual_x = x - len;
        self.print_color(actual_x, y, text, fg, bg);
    }

    fn draw_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        common_draw::draw_box(self, sx, sy, width, height, fg, bg);
    }

    fn draw_hollow_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        common_draw::draw_hollow_box(self, x, y, width, height, fg, bg);
    }

    fn draw_box_double(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        common_draw::draw_box_double(self, x, y, width, height, fg, bg);
    }

    fn draw_hollow_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    ) {
        common_draw::draw_hollow_box_double(self, x, y, width, height, fg, bg);
    }

    fn fill_region(&mut self, target: Rect, glyph: FontCharType, fg: RGBA, bg: RGBA) {
        target.for_each(|point| {
            self.set(point.x, point.y, fg, bg, glyph);
        });
    }

    fn draw_bar_horizontal(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGBA,
        bg: RGBA,
    ) {
        common_draw::draw_bar_horizontal(self, x, y, width, n, max, fg, bg);
    }

    fn draw_bar_vertical(
        &mut self,
        x: i32,
        y: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGBA,
        bg: RGBA,
    ) {
        common_draw::draw_bar_vertical(self, x, y, height, n, max, fg, bg);
    }

    fn set_all_alpha(&mut self, fg: f32, bg: f32) {
        self.terminal.iter_mut().for_each(|t| {
            t.foreground[3] = fg;
            t.background[3] = bg;
        });
    }

    fn set_all_bg_alpha(&mut self, alpha: f32) {
        self.terminal.iter_mut().for_each(|t| {
            t.background[3] = alpha;
        });
    }

    fn set_all_fg_alpha(&mut self, alpha: f32) {
        self.terminal.iter_mut().for_each(|t| {
            t.foreground[3] = alpha;
        });
    }

    fn new_mesh(
        &mut self,
        _ctx: &BracketContext,
        _meshes: &mut bevy::prelude::Assets<bevy::prelude::Mesh>,
        _scaler: &super::ScreenScaler,
    ) -> Option<bevy::prelude::Handle<bevy::prelude::Mesh>> {
        None
    }

    fn resize(&mut self, _available_size: &(f32, f32)) {
        // Does nothing yet
    }

    fn get_mouse_position_for_current_layer(&self) -> Point {
        Point::new(0, 0)
    }

    fn set_mouse_position(&mut self, _position: (f32, f32), _scaler: &super::ScreenScaler) {
        // Do nothing
    }

    fn get_font_index(&self) -> usize {
        // Doesn't make sense here
        0
    }
}
