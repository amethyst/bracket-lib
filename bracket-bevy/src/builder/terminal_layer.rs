use std::collections::HashSet;

use super::font_builder::CharacterTranslationMode;

#[derive(Debug, Clone)]
pub enum TerminalLayer {
    Simple {
        font_index: usize,
        width: i32,
        height: i32,
        features: HashSet<SimpleConsoleFeatures>,
        translation_mode: CharacterTranslationMode,
    },
    Sparse {
        font_index: usize,
        width: i32,
        height: i32,
        features: HashSet<SparseConsoleFeatures>,
        translation_mode: CharacterTranslationMode,
    },
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SimpleConsoleFeatures {
    WithoutBackground,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SparseConsoleFeatures {
    WithoutBackground,
}
