use amethyst::{
    renderer::SpriteSheet,
    assets::Handle
};

pub struct Font{
    pub tile_size: (u32, u32),
    pub filename : String,
    pub ss : Option<Handle<SpriteSheet>>
}

impl Font {
    pub fn load<S: ToString>(filename: S, tile_size: (u32, u32)) -> Font {
        Font{
            tile_size,
            filename : filename.to_string(),
            ss : None
        }
    }

    pub fn setup_gl_texture(&mut self, _gl: &crate::hal::RltkPlatform) {

    }

    pub fn bind_texture(&self, _gl: &crate::hal::RltkPlatform) {

    }
}
