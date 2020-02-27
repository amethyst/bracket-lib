use crate::prelude::embedding;
use crate::Result;
use glow::HasContext;
use image::{ColorType, GenericImageView};

#[derive(PartialEq, Clone)]
/// BTerm's representation of a font or tileset file.
pub struct Font {
    pub bitmap_file: String,
    pub width: u32,
    pub height: u32,

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

    fn load_image(filename: &str) -> image::DynamicImage {
        let resource = embedding::EMBED
            .lock()
            .unwrap()
            .get_resource(filename.to_string());
        match resource {
            None => image::open(std::path::Path::new(&filename.to_string()))
                .expect("Failed to load texture"),
            Some(res) => image::load_from_memory(res).expect("Failed to load texture from memory"),
        }
    }

    /// Loads a font file (texture) to obtain the width and height for you
    pub fn load<S: ToString>(filename: S, tile_size: (u32, u32)) -> Font {
        let img = Font::load_image(&filename.to_string());
        Font {
            bitmap_file: filename.to_string(),
            width: img.width(),
            height: img.height(),
            gl_id: None,
            tile_size,
        }
    }

    /// Load a font, and allocate it as an OpenGL resource. Returns the OpenGL binding number (which is also set in the structure).
    pub fn setup_gl_texture(
        &mut self,
        gl: &glow::Context,
    ) -> Result<glow::WebTextureKey> {
        let texture;

        unsafe {
            texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            ); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            // set texture filtering parameters
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            let img_orig = Font::load_image(&self.bitmap_file);
            let img = img_orig.flipv();
            let data = img.raw_pixels();
            let format = match img.color() {
                ColorType::RGB(_) => glow::RGB,
                ColorType::RGBA(_) => glow::RGBA,
                _ => {
                    panic!(
                        "unexpected image format {:?} for {}",
                        img.color(),
                        self.bitmap_file
                    );
                }
            };
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                glow::UNSIGNED_BYTE,
                Some(&data),
            );
        }

        self.gl_id = Some(texture);

        Ok(texture)
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
        let f = Font::load("resources/terminal8x8.png", (8, 8));
        assert_eq!(f.width, 128);
        assert_eq!(f.height, 128);
    }
}
