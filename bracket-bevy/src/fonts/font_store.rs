use bevy::{
    prelude::{Handle, Image},
    sprite::ColorMaterial,
};

/// Stores handles to the components of a font, along with its glyph settings.
pub(crate) struct FontStore {
    //pub(crate) texture_handle: Handle<Image>,
    pub(crate) material_handle: Handle<ColorMaterial>,
    pub(crate) chars_per_row: u16,
    pub(crate) n_rows: u16,
    pub(crate) font_height_pixels: (f32, f32),
}

impl FontStore {
    pub(crate) fn new(
        _texture_handle: Handle<Image>,
        material_handle: Handle<ColorMaterial>,
        chars_per_row: u16,
        n_rows: u16,
        font_height_pixels: (f32, f32),
    ) -> Self {
        Self {
            //texture_handle,
            material_handle,
            chars_per_row,
            n_rows,
            font_height_pixels,
        }
    }
}
