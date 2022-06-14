use super::{
    back_end::{SimpleBackendNoBackground, SimpleBackendWithBackground, SimpleConsoleBackend},
    TerminalGlyph,
};
use crate::{
    consoles::{common_draw, ConsoleFrontEnd, Rect, ScreenScaler},
    fonts::FontStore,
    BracketContext, SimpleConsoleFeatures,
};
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::Point;
use std::collections::HashSet;

pub(crate) struct SimpleConsole {
    pub(crate) font_index: usize,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) terminal: Vec<TerminalGlyph>,
    back_end: Option<Box<dyn SimpleConsoleBackend>>,
    clipping: Option<Rect>,
    mouse_chars: (i32, i32),
}

impl SimpleConsole {
    pub fn new(font_index: usize, width: i32, height: i32) -> Self {
        Self {
            font_index,
            width,
            height,
            terminal: vec![TerminalGlyph::default(); (width * height) as usize],
            back_end: None,
            clipping: None,
            mouse_chars: (0, 0),
        }
    }

    pub(crate) fn initialize(
        &mut self,
        fonts: &[FontStore],
        meshes: &mut Assets<Mesh>,
        features: &HashSet<SimpleConsoleFeatures>,
    ) {
        if !features.contains(&SimpleConsoleFeatures::WithoutBackground) {
            let back_end = SimpleBackendWithBackground::new(
                self,
                meshes,
                fonts[self.font_index].chars_per_row,
                fonts[self.font_index].n_rows,
                fonts[self.font_index].font_height_pixels,
                self.width,
                self.height,
            );
            self.back_end = Some(Box::new(back_end));
        } else {
            let back_end = SimpleBackendNoBackground::new(
                self,
                meshes,
                fonts[self.font_index].chars_per_row,
                fonts[self.font_index].n_rows,
                fonts[self.font_index].font_height_pixels,
                self.width,
                self.height,
            );
            self.back_end = Some(Box::new(back_end));
        }
    }

    pub(crate) fn spawn(
        &self,
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        idx: usize,
    ) {
        if let Some(back_end) = &self.back_end {
            back_end.spawn(commands, material, idx);
        }
    }

    fn at(&self, x: i32, y: i32) -> usize {
        if let Ok(pos) = (((self.height as i32 - 1 - y) * self.width as i32) + x).try_into() {
            pos
        } else {
            0
        }
    }
}

impl ConsoleFrontEnd for SimpleConsole {
    fn get_char_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    fn get_pixel_size(&self) -> (f32, f32) {
        let n_chars = self.get_char_size();
        if let Some(back_end) = &self.back_end {
            let char_size = back_end.get_pixel_size();
            (
                n_chars.0 as f32 * char_size.0,
                n_chars.1 as f32 * char_size.1,
            )
        } else {
            (0.0, 0.0)
        }
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

    fn set(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: u16) {
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
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            text,
        );
    }

    fn print_centered_at(&mut self, x: i32, y: i32, text: &str) {
        self.print(x - (text.to_string().len() as i32 / 2), y, text);
    }

    fn print_color_centered(&mut self, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            text,
            fg,
            bg,
        );
    }

    fn print_color_centered_at(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(x - (text.to_string().len() as i32 / 2), y, text, fg, bg);
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

    fn fill_region(&mut self, target: Rect, glyph: u16, fg: RGBA, bg: RGBA) {
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
        meshes: &mut Assets<Mesh>,
        scaler: &ScreenScaler,
    ) -> Option<Handle<Mesh>> {
        self.back_end
            .as_ref()
            .map(|back_end| back_end.new_mesh(self, meshes, scaler))
    }

    fn resize(&mut self, available_size: &(f32, f32)) {
        if let Some(back_end) = &mut self.back_end {
            let (w, h) = back_end.resize(available_size);
            self.width = w;
            self.height = h;
            self.terminal = vec![TerminalGlyph::default(); (w * h) as usize];
        }
    }

    fn get_mouse_position_for_current_layer(&self) -> Point {
        Point::new(self.mouse_chars.0 as usize, self.mouse_chars.1 as usize)
    }

    fn set_mouse_position(&mut self, pos: (f32, f32), scaler: &ScreenScaler) {
        self.mouse_chars = scaler.calc_mouse_position(pos, self.width, self.height);
    }

    fn get_font_index(&self) -> usize {
        self.font_index
    }
}
