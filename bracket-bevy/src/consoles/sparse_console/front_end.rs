use std::collections::HashSet;

use super::{SparseBackendNoBackground, SparseBackendWithBackground, SparseConsoleBackend};
use crate::{
    consoles::{common_draw, ConsoleFrontEnd, Rect, TerminalGlyph},
    fonts::FontStore,
    SparseConsoleFeatures,
};
use bevy::{
    prelude::{Assets, Color, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};

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
        base_z: f32,
        features: &HashSet<SparseConsoleFeatures>,
    ) {
        if !features.contains(&SparseConsoleFeatures::WithoutBackground) {
            let back_end = SparseBackendWithBackground::new(
                &self,
                meshes,
                fonts[self.font_index].chars_per_row,
                fonts[self.font_index].n_rows,
                fonts[self.font_index].font_height_pixels,
                self.width,
                self.height,
                base_z,
            );
            self.back_end = Some(Box::new(back_end));
        } else {
            let back_end = SparseBackendNoBackground::new(
                &self,
                meshes,
                fonts[self.font_index].chars_per_row,
                fonts[self.font_index].n_rows,
                fonts[self.font_index].font_height_pixels,
                self.width,
                self.height,
                base_z,
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

    fn cls_bg(&mut self, color: Color) {
        self.terminal
            .iter_mut()
            .for_each(|c| c.2.background = color.as_rgba_f32());
    }

    fn set(&mut self, x: usize, y: usize, fg: Color, bg: Color, glyph: u16) {
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

    fn print_color(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
        foreground: Color,
        background: Color,
    ) {
        common_draw::print_color(self, x, y, text, foreground, background);
    }

    fn print_centered(&mut self, y: usize, text: &str) {
        self.print((self.width / 2) - (text.to_string().len() / 2), y, text);
    }

    fn draw_box(
        &mut self,
        sx: usize,
        sy: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) {
        common_draw::draw_box(self, sx, sy, width, height, fg, bg);
    }

    fn draw_hollow_box(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) {
        common_draw::draw_hollow_box(self, x, y, width, height, fg, bg);
    }

    fn draw_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) {
        common_draw::draw_box_double(self, x, y, width, height, fg, bg);
    }

    fn draw_hollow_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) {
        common_draw::draw_hollow_box_double(self, x, y, width, height, fg, bg);
    }

    fn fill_region(&mut self, target: Rect, glyph: u16, fg: Color, bg: Color) {
        target.for_each(|point| {
            self.set(point.x as usize, point.y as usize, fg, bg, glyph);
        });
    }

    fn update_mesh(
        &mut self,
        _ctx: &crate::BracketContext,
        meshes: &mut bevy::prelude::Assets<Mesh>,
    ) {
        if let Some(back_end) = &self.back_end {
            back_end.update_mesh(&self, meshes);
        }
    }
}
