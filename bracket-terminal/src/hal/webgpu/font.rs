use bracket_color::prelude::RGB;

#[derive(Clone)]
pub struct Font {
    pub tile_size: (u32, u32),
}

impl Font {
    pub fn load<S: ToString>(
        filename: S,
        tile_size: (u32, u32),
        explicit_background: Option<RGB>,
    ) -> Font {
        Font { tile_size: (0, 0) }
    }

    pub fn setup_gl_texture(&mut self, _gl: &crate::hal::BTermPlatform) {}

    pub fn bind_texture(&self, _gl: &crate::hal::BTermPlatform) {}
}
