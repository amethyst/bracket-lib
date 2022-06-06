use bevy::prelude::Color;

#[derive(Clone, Copy)]
pub(crate) struct TerminalGlyph {
    pub(crate) glyph: u16,
    pub(crate) foreground: [f32; 4]
}

impl Default for TerminalGlyph {
    fn default() -> Self {
        Self {
            glyph: 65,
            foreground: Color::WHITE.as_rgba_f32(),
        }
    }
}