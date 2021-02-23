use crate::BResult;

#[derive(Clone)]
pub struct Font {
    pub tile_size: (u32, u32),
}

impl Font {
    pub fn load<S: ToString>(
        _filename: S,
        _tile_size: (u32, u32),
        _explicit_background: Option<bracket_color::prelude::RGB>,
    ) -> Font {
        Font { tile_size: (0, 0) }
    }

    pub fn setup_gl_texture(&mut self, _gl: &crate::hal::BTermPlatform) -> BResult<()> {
        Ok(())
    }

    pub fn bind_texture(&self, _gl: &crate::hal::BTermPlatform) {}
}
