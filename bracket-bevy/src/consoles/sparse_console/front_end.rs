use std::collections::HashSet;

use super::{SparseBackendNoBackground, SparseBackendWithBackground, SparseConsoleBackend};
use crate::{
    consoles::{common_draw, ConsoleFrontEnd, Rect, ScreenScaler, TerminalGlyph},
    fonts::FontStore,
    BracketContext, SparseConsoleFeatures,
};
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::Point;

pub(crate) struct SparseConsole {
    pub(crate) font_index: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) terminal: Vec<(i32, i32, TerminalGlyph)>,
    back_end: Option<Box<dyn SparseConsoleBackend>>,
    clipping: Option<Rect>,
    mouse_chars: (usize, usize),
}

impl SparseConsole {
    pub fn new(font_index: usize, width: usize, height: usize) -> Self {
        Self {
            font_index,
            width,
            height,
            terminal: Vec::new(),
            back_end: None,
            clipping: None,
            mouse_chars: (0, 0),
        }
    }

    pub(crate) fn initialize(
        &mut self,
        fonts: &[FontStore],
        meshes: &mut Assets<Mesh>,
        features: &HashSet<SparseConsoleFeatures>,
    ) {
        if !features.contains(&SparseConsoleFeatures::WithoutBackground) {
            let back_end = SparseBackendWithBackground::new(
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
            let back_end = SparseBackendNoBackground::new(
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
}

impl ConsoleFrontEnd for SparseConsole {
    fn get_char_size(&self) -> (usize, usize) {
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
        if let Ok(pos) = (((self.height as i32 - 1 - y) * self.width as i32) + x).try_into() {
            pos
        } else {
            0
        }
    }

    fn get_clipping(&self) -> Option<Rect> {
        self.clipping
    }

    fn set_clipping(&mut self, clipping: Option<Rect>) {
        self.clipping = clipping;
    }

    fn cls(&mut self) {
        self.terminal.clear();
    }

    fn cls_bg(&mut self, color: RGBA) {
        self.terminal
            .iter_mut()
            .for_each(|c| c.2.background = color.as_rgba_f32());
    }

    fn set(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: u16) {
        if self.try_at(x, y).is_some() {
            self.terminal.push((
                x,
                y,
                TerminalGlyph {
                    glyph,
                    foreground: fg.as_rgba_f32(),
                    background: bg.as_rgba_f32(),
                },
            ));
        }
    }

    fn set_bg(&mut self, _x: i32, _y: i32, _bg: RGBA) {
        // Does nothing
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
        self.print((self.width as i32 / 2) - (text.to_string().len() as i32 / 2), y, text);
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

    fn draw_hollow_box(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    ) {
        common_draw::draw_hollow_box(self, x, y, width, height, fg, bg);
    }

    fn draw_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    ) {
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
            t.2.foreground[3] = fg;
            t.2.background[3] = bg;
        });
    }

    fn set_all_bg_alpha(&mut self, alpha: f32) {
        self.terminal.iter_mut().for_each(|t| {
            t.2.background[3] = alpha;
        });
    }

    fn set_all_fg_alpha(&mut self, alpha: f32) {
        self.terminal.iter_mut().for_each(|t| {
            t.2.foreground[3] = alpha;
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
            .map(|be| be.new_mesh(self, meshes, scaler))
    }

    fn resize(&mut self, available_size: &(f32, f32)) {
        if let Some(back_end) = &mut self.back_end {
            let (w, h) = back_end.resize(available_size);
            self.width = w;
            self.height = h;
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
