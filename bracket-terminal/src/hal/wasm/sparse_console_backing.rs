use crate::prelude::{font::Font, Shader};
use crate::sparse_console::SparseTile;
use crate::Result;
use glow::HasContext;

pub struct SparseConsoleBackend {
    charbuffer: glow::WebTextureKey,
    background: glow::WebTextureKey,
    offset_x: f32,
    offset_y: f32,
}

impl SparseConsoleBackend {
    pub fn new(
        gl: &glow::Context,
        width: usize,
        height: usize,
    ) -> SparseConsoleBackend {
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

            let data = vec![200u8; width * height * 4];
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
        }

        let texture2;
        unsafe {
            texture2 = gl.create_texture().unwrap();
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

            let data = vec![200u8; width * height * 4];
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
        }

        SparseConsoleBackend {
            charbuffer: texture,
            background: texture2,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// Helper to build vertices for the sparse grid.
    pub fn rebuild_vertices(
        &mut self,
        gl: &glow::Context,
        height: u32,
        width: u32,
        offset_x: f32,
        offset_y: f32,
        _scale: f32,
        _scale_center: (i32, i32),
        tiles: &Vec<SparseTile>,
    ) {
        unsafe {
            let mut data = vec![0u8; width as usize * height as usize * 4];
            let mut data2 = vec![0u8; width as usize * height as usize * 4];

            for t in tiles.iter() {
                let i = t.idx;
                data[i * 4] = t.glyph;
                data[(i * 4) + 1] = (t.fg.r * 255.0) as u8;
                data[(i * 4) + 2] = (t.fg.g * 255.0) as u8;
                data[(i * 4) + 3] = (t.fg.b * 255.0) as u8;

                data2[(i * 4)] = (t.bg.r * 255.0) as u8;
                data2[(i * 4) + 1] = (t.bg.g * 255.0) as u8;
                data2[(i * 4) + 2] = (t.bg.b * 255.0) as u8;
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
        }

        self.offset_x = offset_x / width as f32;
        self.offset_y = offset_y / height as f32;
    }

    pub fn gl_draw(
        &mut self,
        font: &Font,
        shader: &Shader,
        gl: &glow::Context,
        _tiles: &Vec<SparseTile>,
    ) -> Result<()> {
        unsafe {
            gl.active_texture(glow::TEXTURE1);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.charbuffer));
            shader.setInt(gl, "glyphBuffer", 1);

            gl.active_texture(glow::TEXTURE2);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.background));
            shader.setInt(gl, "bgBuffer", 2);

            shader.setVec3(
                gl,
                "font",
                font.width as f32 / 16.0,
                font.height as f32 / 16.0,
                0.0,
            );
            shader.setBool(gl, "hasBackground", false);
            shader.setVec3(gl, "offset", self.offset_x, self.offset_y, 0.0);
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
        Ok(())
    }
}
