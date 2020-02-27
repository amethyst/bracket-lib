use crate::Result;
pub struct Font {
    pub tile_size: (u32, u32),
}

impl Font {
    pub fn load<S: ToString>(_filename: S, _tile_size: (u32, u32)) -> Font {
        Font { tile_size: (0, 0) }
    }

    pub fn setup_gl_texture(&mut self, _gl: &crate::hal::BTermPlatform) -> Result<()> {
        Ok(())
    }

    pub fn bind_texture(&self, _gl: &crate::hal::BTermPlatform) {}
}