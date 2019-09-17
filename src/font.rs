use glow::HasContext;
use image::GenericImageView;
use super::platform_specific;

#[derive(PartialEq, Clone)]
/// RLTK's representation of a font or tileset file.
pub struct Font {
    pub bitmap_file: String,
    pub width: u32,
    pub height: u32,

    #[cfg(not(target_arch = "wasm32"))]
    pub gl_id: Option<u32>,

    #[cfg(target_arch = "wasm32")]
    pub gl_id: Option<glow::WebTextureKey>,

    pub tile_size: (u32, u32),
}

#[allow(non_snake_case)]
impl Font {
    /// Creates an unloaded texture with filename and size parameters provided.
    pub fn new<S: ToString>(filename: S, width: u32, height: u32, tile_size: (u32, u32)) -> Font {
        Font {
            bitmap_file: filename.to_string(),
            width,
            height,
            gl_id: None,
            tile_size,
        }
    }

    /// Loads a font file (texture) to obtain the width and height for you
    pub fn load<S: ToString>(filename: S, tile_size: (u32, u32)) -> Font {
        let img = image::open(std::path::Path::new(&filename.to_string()))
            .expect("Failed to load texture");
        Font {
            bitmap_file: filename.to_string(),
            width: img.width(),
            height: img.height(),
            gl_id: None,
            tile_size,
        }
    }

    /// Load a font, and allocate it as an OpenGL resource. Returns the OpenGL binding number (which is also set in the structure).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn setup_gl_texture(&mut self, gl: &glow::Context) -> u32 {
        let texture = platform_specific::setup_gl_texture(gl, &self.bitmap_file);
        self.gl_id = Some(texture);
        texture
    }

    /// Load a font, and allocate it as an OpenGL resource. Returns the OpenGL binding number (which is also set in the structure).
    #[cfg(target_arch = "wasm32")]
    pub fn setup_gl_texture(&mut self, gl: &glow::Context) -> glow::WebTextureKey {
        let texture = platform_specific::setup_gl_texture(gl, &self.bitmap_file);
        self.gl_id = Some(texture);
        texture
    }

    /// Sets this font file as the active texture
    pub fn bind_texture(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, self.gl_id);
        }
    }
}

// Some unit testing for fonts

#[cfg(test)]
mod tests {
    use super::Font;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_font_minimal() {
        let f = Font::new("test.png", 1, 2, (8, 8));
        assert_eq!(f.bitmap_file, "test.png");
        assert_eq!(f.width, 1);
        assert_eq!(f.height, 2);
    }

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_font_from_file() {
        let f = Font::load("resources/terminal8x8.jpg", (8, 8));
        assert_eq!(f.width, 128);
        assert_eq!(f.height, 128);
    }
}
