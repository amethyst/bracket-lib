use crate::prelude::embedding;
use crate::BResult;
use bracket_color::prelude::RGB;
use image::GenericImageView;

use super::WgpuLink;

#[derive(PartialEq, Clone)]
/// BTerm's representation of a font or tileset file.
pub struct Font {
    pub bitmap_file: String,
    pub width: u32,
    pub height: u32,

    pub texture_id: Option<usize>,

    pub tile_size: (u32, u32),
    pub explicit_background: Option<RGB>,
    pub font_dimensions_glyphs: (u32, u32),
}

#[allow(non_snake_case)]
impl Font {
    /// Creates an unloaded texture with filename and size parameters provided.
    pub fn new<S: ToString>(filename: S, width: u32, height: u32, tile_size: (u32, u32)) -> Font {
        Font {
            bitmap_file: filename.to_string(),
            width,
            height,
            texture_id: None,
            tile_size,
            explicit_background: None,
            font_dimensions_glyphs: (tile_size.0 / width, tile_size.1 / height),
        }
    }

    fn load_image(filename: &str) -> image::DynamicImage {
        let resource = embedding::EMBED.lock().get_resource(filename.to_string());
        match resource {
            None => image::open(std::path::Path::new(&filename.to_string()))
                .expect("Failed to load texture"),
            Some(res) => image::load_from_memory(res).expect("Failed to load texture from memory"),
        }
    }

    /// Loads a font file (texture) to obtain the width and height for you
    pub fn load<S: ToString>(
        filename: S,
        tile_size: (u32, u32),
        explicit_background: Option<RGB>,
    ) -> Font {
        let img = Font::load_image(&filename.to_string());
        Font {
            bitmap_file: filename.to_string(),
            width: img.width(),
            height: img.height(),
            texture_id: None,
            tile_size,
            explicit_background,
            font_dimensions_glyphs: (img.width() / tile_size.0, img.height() / tile_size.1),
        }
    }

    /// Load a font, and allocate it as an OpenGL resource. Returns the OpenGL binding number (which is also set in the structure).
    pub fn setup_wgpu_texture(&mut self, wgpu: &WgpuLink) -> BResult<usize> {
        let texture = 0;

        Ok(texture)
    }

    // Sets this font file as the active texture
    /*pub fn bind_texture(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, self.gl_id);
            gl_error(gl);
        }
    }*/
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
        let f = Font::load("resources/terminal8x8.png", (8, 8), None);
        assert_eq!(f.width, 128);
        assert_eq!(f.height, 128);
    }
}
