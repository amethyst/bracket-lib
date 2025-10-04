use bevy::{color::ColorToComponents, prelude::Color};

#[derive(Clone, Copy)]
pub struct TerminalGlyph {
    pub(crate) glyph: u16,
    pub(crate) foreground: [f32; 4],
    pub(crate) background: [f32; 4],
}

impl Default for TerminalGlyph {
    fn default() -> Self {
        Self {
            glyph: 32,
            foreground: Color::WHITE.to_srgba().to_f32_array(),
            background: Color::BLACK.to_srgba().to_f32_array(),
        }
    }
}
