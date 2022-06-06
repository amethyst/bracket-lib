use super::{
    back_end::{SimpleBackendWithBackground, SimpleConsoleBackend},
    TerminalGlyph,
};
use crate::{consoles::ConsoleFrontEnd, fonts::FontStore, prelude::string_to_cp437};
use bevy::{
    prelude::{Assets, Color, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};

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
    ) {
        let back_end = SimpleBackendWithBackground::new(
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

    fn print(&mut self, mut x: usize, y: usize, text: &str) {
        let bytes = string_to_cp437(text);
        for glyph in bytes {
            let idx = self.at(x, y);
            self.terminal[idx] = TerminalGlyph {
                glyph,
                foreground: Color::WHITE.as_rgba_f32(),
                background: Color::BLACK.as_rgba_f32(),
            };
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
            let idx = self.at(x, y);
            self.terminal[idx] = TerminalGlyph {
                glyph,
                foreground: foreground.as_rgba_f32(),
                background: background.as_rgba_f32(),
            };
            x += 1;
        }
    }

    fn update_mesh(&self, _ctx: &crate::BracketContext, meshes: &mut bevy::prelude::Assets<Mesh>) {
        if let Some(back_end) = &self.back_end {
            back_end.update_mesh(&self, meshes);
        }
    }
}
