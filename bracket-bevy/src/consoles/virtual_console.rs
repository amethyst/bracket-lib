use bracket_color::prelude::RGBA;

use super::{common_draw, ConsoleFrontEnd, TerminalGlyph};
use crate::{BracketContext, Point, Rect};

pub struct VirtualConsole {
    pub width: usize,
    pub height: usize,
    pub terminal: Vec<TerminalGlyph>,
    pub clipping: Option<Rect>,
}

impl VirtualConsole {
    /// Creates a new virtual console of arbitrary dimensions.
    pub fn new(dimensions: Point) -> Self {
        let num_tiles: usize = (dimensions.x * dimensions.y) as usize;
        let mut console = VirtualConsole {
            width: dimensions.x as usize,
            height: dimensions.y as usize,
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
            width: width,
            height: lines.len(),
            terminal: Vec::with_capacity(num_tiles),
            clipping: None,
        };
        //println!("{}x{}", console.width, console.height);

        for _ in 0..num_tiles {
            console.terminal.push(TerminalGlyph::default());
        }

        for (i, line) in lines.iter().enumerate() {
            console.print(0, i, line);
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
                if let Some(idx) = self.try_at(source_x as usize, source_y as usize) {
                    let t = self.terminal[idx];
                    if t.glyph > 0 {
                        target.set(
                            x as usize,
                            y as usize,
                            t.foreground.into(),
                            t.background.into(),
                            t.glyph,
                        );
                    }
                }
            }
        }
        target.set_clipping(None);
    }

    fn at(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y) * self.width) + x
    }
}

impl ConsoleFrontEnd for VirtualConsole {
    fn get_char_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get_pixel_size(&self) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn at(&self, x: usize, y: usize) -> usize {
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

    fn set(&mut self, x: usize, y: usize, fg: RGBA, bg: RGBA, glyph: u16) {
        if let Some(idx) = self.try_at(x, y) {
            self.terminal[idx] = TerminalGlyph {
                glyph,
                foreground: fg.as_rgba_f32(),
                background: bg.as_rgba_f32(),
            };
        }
    }

    fn set_bg(&mut self, x: usize, y: usize, bg: RGBA) {
        if let Some(idx) = self.try_at(x, y) {
            self.terminal[idx].background = bg.as_rgba_f32();
        }
    }

    fn print(&mut self, x: usize, y: usize, text: &str) {
        common_draw::print(self, x, y, text);
    }

    fn print_color(&mut self, x: usize, y: usize, text: &str, foreground: RGBA, background: RGBA) {
        common_draw::print_color(self, x, y, text, foreground, background);
    }

    fn printer(
        &mut self,
        context: &BracketContext,
        x: usize,
        y: usize,
        output: &str,
        align: crate::consoles::TextAlign,
        background: Option<RGBA>,
    ) {
        common_draw::printer(self, context, x, y, output, align, background);
    }

    fn print_centered(&mut self, y: usize, text: &str) {
        self.print((self.width / 2) - (text.to_string().len() / 2), y, text);
    }

    fn print_centered_at(&mut self, x: usize, y: usize, text: &str) {
        self.print(x - (text.to_string().len() / 2), y, text);
    }

    fn print_color_centered(&mut self, y: usize, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(
            (self.width / 2) - (text.to_string().len() / 2),
            y,
            text,
            fg,
            bg,
        );
    }

    fn print_color_centered_at(&mut self, x: usize, y: usize, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(x - (text.to_string().len() / 2), y, text, fg, bg);
    }

    fn print_right(&mut self, x: usize, y: usize, text: &str) {
        let len = text.len();
        let actual_x = x - len;
        self.print(actual_x, y, text);
    }

    fn print_color_right(&mut self, x: usize, y: usize, fg: RGBA, bg: RGBA, text: &str) {
        let len = text.len();
        let actual_x = x - len;
        self.print_color(actual_x, y, text, fg, bg);
    }

    fn draw_box(&mut self, sx: usize, sy: usize, width: usize, height: usize, fg: RGBA, bg: RGBA) {
        common_draw::draw_box(self, sx, sy, width, height, fg, bg);
    }

    fn draw_hollow_box(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: RGBA,
        bg: RGBA,
    ) {
        common_draw::draw_hollow_box(self, x, y, width, height, fg, bg);
    }

    fn draw_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: RGBA,
        bg: RGBA,
    ) {
        common_draw::draw_box_double(self, x, y, width, height, fg, bg);
    }

    fn draw_hollow_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: RGBA,
        bg: RGBA,
    ) {
        common_draw::draw_hollow_box_double(self, x, y, width, height, fg, bg);
    }

    fn fill_region(&mut self, target: Rect, glyph: u16, fg: RGBA, bg: RGBA) {
        target.for_each(|point| {
            self.set(point.x as usize, point.y as usize, fg, bg, glyph);
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
}
