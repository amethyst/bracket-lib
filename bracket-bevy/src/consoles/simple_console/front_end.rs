use super::{
    back_end::{SimpleBackendNoBackground, SimpleBackendWithBackground, SimpleConsoleBackend},
    TerminalGlyph,
};
use crate::{
    consoles::{common_draw, ConsoleFrontEnd},
    fonts::FontStore,
    SimpleConsoleFeatures,
};
use bevy::{
    prelude::{Assets, Color, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
use std::collections::HashSet;

pub(crate) struct SimpleConsole {
    pub(crate) font_index: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) terminal: Vec<TerminalGlyph>,
    back_end: Option<Box<dyn SimpleConsoleBackend>>,
}

impl SimpleConsole {
    pub fn new(font_index: usize, width: usize, height: usize) -> Self {
        Self {
            font_index,
            width,
            height,
            terminal: vec![TerminalGlyph::default(); width * height],
            back_end: None,
        }
    }

    pub(crate) fn initialize(
        &mut self,
        fonts: &[FontStore],
        meshes: &mut Assets<Mesh>,
        base_z: f32,
        features: &HashSet<SimpleConsoleFeatures>,
    ) {
        if !features.contains(&SimpleConsoleFeatures::WithoutBackground) {
            let back_end = SimpleBackendWithBackground::new(
                &self,
                meshes,
                fonts[self.font_index].chars_per_row,
                fonts[self.font_index].n_rows,
                fonts[self.font_index].font_height_pixels,
                self.width,
                self.height,
                base_z,
                features.contains(&SimpleConsoleFeatures::NoDirtyOptimization),
            );
            self.back_end = Some(Box::new(back_end));
        } else {
            let back_end = SimpleBackendNoBackground::new(
                &self,
                meshes,
                fonts[self.font_index].chars_per_row,
                fonts[self.font_index].n_rows,
                fonts[self.font_index].font_height_pixels,
                self.width,
                self.height,
                base_z,
                features.contains(&SimpleConsoleFeatures::NoDirtyOptimization),
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

    fn at(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y) * self.width) + x
    }
}

impl ConsoleFrontEnd for SimpleConsole {
    fn cls(&mut self) {
        self.terminal.iter_mut().for_each(|c| c.glyph = 32);
    }

    fn cls_bg(&mut self, color: Color) {
        self.terminal
            .iter_mut()
            .for_each(|c| c.background = color.as_rgba_f32());
    }

    fn set(&mut self, x: usize, y: usize, fg: Color, bg: Color, glyph: u16) {
        let idx = self.at(x, y);
        self.terminal[idx] = TerminalGlyph {
            glyph,
            foreground: fg.as_rgba_f32(),
            background: bg.as_rgba_f32(),
        };
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

    fn update_mesh(
        &mut self,
        _ctx: &crate::BracketContext,
        meshes: &mut bevy::prelude::Assets<Mesh>,
    ) {
        if let Some(back_end) = &mut self.back_end {
            back_end.update_dirty(&self.terminal);
        }
        if let Some(back_end) = &self.back_end {
            back_end.update_mesh(&self, meshes);
        }
        if let Some(back_end) = &mut self.back_end {
            back_end.clear_dirty();
        }
    }
}
