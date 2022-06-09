use std::collections::HashSet;

use super::{SparseBackendNoBackground, SparseBackendWithBackground, SparseConsoleBackend};
use crate::{
    consoles::{common_draw, ConsoleFrontEnd, Rect, TerminalGlyph},
    fonts::FontStore,
    BracketContext, SparseConsoleFeatures,
};
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
use bracket_color::prelude::RGBA;

pub(crate) struct SparseConsole {
    pub(crate) font_index: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) terminal: Vec<(usize, usize, TerminalGlyph)>,
    back_end: Option<Box<dyn SparseConsoleBackend>>,
    clipping: Option<Rect>,
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

    fn at(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y) * self.width) + x
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

    fn set(&mut self, x: usize, y: usize, fg: RGBA, bg: RGBA, glyph: u16) {
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
        meshes: &mut Assets<Mesh>,
    ) -> Option<Handle<Mesh>> {
        self.back_end.as_ref().map(|be| be.new_mesh(self, meshes))
    }
}
