use std::collections::HashSet;

use super::{SparseBackendNoBackground, SparseBackendWithBackground, SparseConsoleBackend};
use crate::{
    consoles::{ConsoleFrontEnd, TerminalGlyph},
    fonts::FontStore,
    prelude::{string_to_cp437, to_cp437},
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
}

impl SparseConsole {
    pub fn new(font_index: usize, width: usize, height: usize) -> Self {
        Self {
            font_index,
            width,
            height,
            terminal: Vec::new(),
            back_end: None,
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
    fn cls(&mut self) {
        self.terminal.clear();
    }

    fn set(&mut self, x: usize, y: usize, fg: Color, bg: Color, glyph: u16) {
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

    fn print(&mut self, mut x: usize, y: usize, text: &str) {
        let bytes = string_to_cp437(text);
        for glyph in bytes {
            self.terminal.push((
                x,
                y,
                TerminalGlyph {
                    glyph,
                    foreground: Color::WHITE.as_rgba_f32(),
                    background: Color::BLACK.as_rgba_f32(),
                },
            ));
            x += 1;
        }
    }

    fn print_color(
        &mut self,
        mut x: usize,
        y: usize,
        text: &str,
        foreground: Color,
        background: Color,
    ) {
        let bytes = string_to_cp437(text);
        for glyph in bytes {
            self.terminal.push((
                x,
                y,
                TerminalGlyph {
                    glyph,
                    foreground: foreground.as_rgba_f32(),
                    background: background.as_rgba_f32(),
                },
            ));
            x += 1;
        }
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
        for y in sy..sy + height {
            for x in sx..sx + width {
                self.set(x, y, Color::WHITE, Color::BLACK, 32);
            }
        }

        self.set(sx, sy, fg, bg, to_cp437('┌'));
        self.set(sx + width, sy, fg, bg, to_cp437('┐'));
        self.set(sx, sy + height, fg, bg, to_cp437('└'));
        self.set(sx + width, sy + height, fg, bg, to_cp437('┘'));
        for x in sx + 1..sx + width {
            self.set(x, sy, fg, bg, to_cp437('─'));
            self.set(x, sy + height, fg, bg, to_cp437('─'));
        }
        for y in sy + 1..sy + height {
            self.set(sx, y, fg, bg, to_cp437('│'));
            self.set(sx + width, y, fg, bg, to_cp437('│'));
        }
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
