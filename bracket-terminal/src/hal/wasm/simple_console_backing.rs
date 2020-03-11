use crate::prelude::Tile;
use crate::prelude::{font::Font, Shader};
use crate::Result;
use glow::HasContext;

pub struct SimpleConsoleBackend {
    charbuffer: glow::WebTextureKey,
    background: glow::WebTextureKey,
    foreground: glow::WebTextureKey,
    offset_x: f32,
    offset_y: f32,
}

fn make_backing_texture(gl: &glow::Context, width: usize, height: usize) -> glow::WebTextureKey {
    unsafe {
        let texture = gl.create_texture().unwrap();
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

        let data = vec![0u8; width * height * 4];
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&data.align_to::<u8>().1),
        );
        texture
    }
}

impl SimpleConsoleBackend {
    pub fn new(gl: &glow::Context, width: usize, height: usize) -> SimpleConsoleBackend {
        let texture = make_backing_texture(gl, width, height);
        let texture2 = make_backing_texture(gl, width, height);
        let texture3 = make_backing_texture(gl, width, height);

        SimpleConsoleBackend {
            charbuffer: texture,
            background: texture2,
            foreground: texture3,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// Rebuilds the OpenGL backing buffer.
    pub fn rebuild_vertices(
        &mut self,
        gl: &glow::Context,
        height: u32,
        width: u32,
        tiles: &Vec<Tile>,
        offset_x: f32,
        offset_y: f32,
        _scale: f32,
        _scale_center: (i32, i32),
        needs_resize: bool,
    ) {
        if needs_resize {
            unsafe {
                gl.delete_texture(self.charbuffer);
                gl.delete_texture(self.foreground);
                gl.delete_texture(self.background);
                self.charbuffer = make_backing_texture(gl, width as usize, height as usize);
                self.foreground = make_backing_texture(gl, width as usize, height as usize);
                self.background = make_backing_texture(gl, width as usize, height as usize);
                super::log(&format!("Textures rebuilt. {}x{}", width, height));
            }
        }

        unsafe {
            let mut data = vec![0u8; width as usize * height as usize * 4];
            let mut data2 = vec![0u8; width as usize * height as usize * 4];
            let mut data3 = vec![0u8; width as usize * height as usize * 4];

            for (i, t) in tiles.iter().enumerate() {
                data[i * 4] = t.glyph as u8;
                data[(i * 4) + 1] = (t.fg.r * 255.0) as u8;
                data[(i * 4) + 2] = (t.fg.g * 255.0) as u8;
                data[(i * 4) + 3] = (t.fg.b * 255.0) as u8;

                data2[(i * 4)] = (t.bg.r * 255.0) as u8;
                data2[(i * 4) + 1] = (t.bg.g * 255.0) as u8;
                data2[(i * 4) + 2] = (t.bg.b * 255.0) as u8;
                data2[(i * 4) + 3] = (t.bg.a * 255.0) as u8;

                data3[(i * 4)] = (t.fg.r * 255.0) as u8;
                data3[(i * 4) + 1] = (t.fg.g * 255.0) as u8;
                data3[(i * 4) + 2] = (t.fg.b * 255.0) as u8;
                data3[(i * 4) + 3] = (t.fg.a * 255.0) as u8;
            }

            gl.bind_texture(glow::TEXTURE_2D, Some(self.charbuffer));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&data.align_to::<u8>().1),
            );

            gl.bind_texture(glow::TEXTURE_2D, Some(self.background));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&data2.align_to::<u8>().1),
            );

            gl.bind_texture(glow::TEXTURE_2D, Some(self.foreground));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(&data3.align_to::<u8>().1),
            );
        }

        self.offset_x = offset_x / width as f32;
        self.offset_y = offset_y / height as f32;
    }

    pub fn gl_draw(
        &mut self,
        font: &Font,
        shader: &Shader,
        gl: &glow::Context,
        _width: u32,
        _height: u32,
    ) -> Result<()> {
        unsafe {
            gl.active_texture(glow::TEXTURE0);
            font.bind_texture(gl);

            gl.active_texture(glow::TEXTURE1);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.charbuffer));
            shader.setInt(gl, "glyphBuffer", 1);

            gl.active_texture(glow::TEXTURE2);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.background));
            shader.setInt(gl, "bgBuffer", 2);

            gl.active_texture(glow::TEXTURE3);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.foreground));
            shader.setInt(gl, "fgBuffer", 3);

            shader.setVec3(
                gl,
                "font",
                font.width as f32 / font.font_dimensions_glyphs.0 as f32,
                font.height as f32 / font.font_dimensions_glyphs.1 as f32,
                0.0,
            );
            shader.setBool(gl, "hasBackground", true);
            shader.setVec3(gl, "offset", self.offset_x, self.offset_y, 0.0);
            shader.setVec2(
                gl,
                "font_dimensions_glyph",
                font.font_dimensions_glyphs.0 as f32,
                font.font_dimensions_glyphs.1 as f32,
            );
            //super::log(&format!("{}x{}", font.font_dimensions_glyphs.0, font.font_dimensions_glyphs.1));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
        Ok(())
    }
}
