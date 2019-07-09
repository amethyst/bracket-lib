#[allow(dead_code)]
extern crate image;
use image::GenericImageView;
use std::os::raw::c_void;
use super::gl;

#[derive(PartialEq, Clone)]
pub struct Font {
    pub bitmap_file : String,
    pub width: u32,
    pub height: u32,
    pub gl_id: Option<u32>,
    pub tile_size: (u32, u32)
}

#[allow(non_snake_case)]
impl Font {
    // Creates an unloaded texture with filename and size parameters provided.
    pub fn new<S:ToString>(filename : S, width: u32, height: u32, tile_size : (u32, u32)) -> Font {
        Font { bitmap_file : filename.to_string(), width: width, height: height, gl_id: None, tile_size: tile_size }
    }

    // Loads a font file (texture) to obtain the width and height for you
    pub fn load<S:ToString>(filename: S, tile_size : (u32, u32)) -> Font {
        let img = image::open(std::path::Path::new(&filename.to_string())).expect("Failed to load texture");
        Font { bitmap_file: filename.to_string(), width: img.width(), height: img.height(), gl_id : None, tile_size: tile_size }
    }

    // Load a font, and allocate it as an OpenGL resource. Returns the OpenGL binding number (which is also set in the structure).
    pub fn setup_gl_texture(&mut self, gl : &gl::Gles2) -> u32 {
        let mut texture : u32 = 0;
        
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
            // set the texture wrapping parameters
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let img_orig = image::open(std::path::Path::new(&self.bitmap_file)).expect("Failed to load texture");
            let img = img_orig.flipv();
            let data = img.raw_pixels();
            gl.TexImage2D(gl::TEXTURE_2D,
                        0,
                        gl::RGB as i32,
                        img.width() as i32,
                        img.height() as i32,
                        0,
                        gl::RGB,
                        gl::UNSIGNED_BYTE,
                        &data[0] as *const u8 as *const c_void);
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }

        self.gl_id = Some(texture);

        texture
    }

    // Sets this font file as the active texture
    pub fn bind_texture(&self, gl : &gl::Gles2) {
        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, self.gl_id.unwrap());
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
        let f = Font::new("test.png", 1, 2, (8,8));
        assert_eq!(f.bitmap_file, "test.png");
        assert_eq!(f.width, 1);
        assert_eq!(f.height, 2);
    }

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_font_from_file() {
        let f = Font::load("resources/terminal8x8.jpg", (8,8));
        assert_eq!(f.width, 128);
        assert_eq!(f.height, 128);
    }
}