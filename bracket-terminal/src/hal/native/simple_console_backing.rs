use super::BACKEND;
use crate::hal::{font::Font, shader::Shader};
use crate::prelude::Tile;
use crate::Result;
use bracket_color::prelude::RGB;
use glow::HasContext;
use std::mem;

pub struct SimpleConsoleBackend {
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<i32>,
    vbo: u32,
    vao: u32,
    ebo: u32,
    vertex_counter: usize,
    index_counter: usize,
}

impl SimpleConsoleBackend {
    pub fn new(width: usize, height: usize, gl: &glow::Context) -> SimpleConsoleBackend {
        let vertex_capacity: usize = (11 * width as usize * height as usize) * 4;
        let index_capacity: usize = 6 * width as usize * height as usize;
        let (vbo, vao, ebo) = SimpleConsoleBackend::init_gl_for_console(gl);
        let mut result = SimpleConsoleBackend {
            vertex_buffer: Vec::with_capacity(vertex_capacity),
            index_buffer: Vec::with_capacity(index_capacity),
            vbo,
            vao,
            ebo,
            vertex_counter: 0,
            index_counter: 0,
        };
        for _ in 0..vertex_capacity {
            result.vertex_buffer.push(0.0);
        }
        for _ in 0..index_capacity {
            result.index_buffer.push(0);
        }
        result
    }

    fn init_gl_for_console(gl: &glow::Context) -> (u32, u32, u32) {
        let (texture, vbo, vao, ebo);

        unsafe {
            // Generate buffers and arrays, as well as attributes.
            vao = gl.create_vertex_array().unwrap();
            vbo = gl.create_buffer().unwrap();
            ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let stride = 11 * mem::size_of::<f32>() as i32;
            // position attribute
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);
            // color attribute
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                stride,
                (3 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(1);
            // bgcolor attribute
            gl.vertex_attrib_pointer_f32(
                2,
                3,
                glow::FLOAT,
                false,
                stride,
                (6 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(2);
            // texture coord attribute
            gl.vertex_attrib_pointer_f32(
                3,
                2,
                glow::FLOAT,
                false,
                stride,
                (9 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(3);

            texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture)); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
                                                              // set the texture wrapping parameters
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            ); // set texture wrapping to glow::REPEAT (default wrapping method)
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
        };

        (vbo, vao, ebo)
    }

    /// Helper function to add all the elements required by the shader for a given point.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        &mut self,
        x: f32,
        y: f32,
        fg: RGB,
        bg: RGB,
        ux: f32,
        uy: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        self.vertex_buffer[self.vertex_counter] = x + offset_x;
        self.vertex_buffer[self.vertex_counter + 1] = y + offset_y;
        self.vertex_buffer[self.vertex_counter + 2] = 0.0f32;
        self.vertex_buffer[self.vertex_counter + 3] = fg.r;
        self.vertex_buffer[self.vertex_counter + 4] = fg.g;
        self.vertex_buffer[self.vertex_counter + 5] = fg.b;
        self.vertex_buffer[self.vertex_counter + 6] = bg.r;
        self.vertex_buffer[self.vertex_counter + 7] = bg.g;
        self.vertex_buffer[self.vertex_counter + 8] = bg.b;
        self.vertex_buffer[self.vertex_counter + 9] = ux;
        self.vertex_buffer[self.vertex_counter + 10] = uy;
        self.vertex_counter += 11;
    }

    /// Rebuilds the OpenGL backing buffer.
    pub fn rebuild_vertices(
        &mut self,
        height: u32,
        width: u32,
        tiles: &[Tile],
        offset_x: f32,
        offset_y: f32,
        scale: f32,
        scale_center: (i32, i32),
    ) {
        self.vertex_counter = 0;
        self.index_counter = 0;
        let glyph_size_x: f32 = 1.0f32 / 16.0f32;
        let glyph_size_y: f32 = 1.0f32 / 16.0f32;

        let step_x: f32 = scale * 2.0f32 / width as f32;
        let step_y: f32 = scale * 2.0f32 / height as f32;

        let mut index_count: i32 = 0;
        let mut screen_y: f32 = -1.0 * scale
            + 2.0 * (scale_center.1 - height as i32 / 2) as f32 * (scale - 1.0) / height as f32;
        for y in 0..height {
            let mut screen_x: f32 = -1.0 * scale
                - 2.0 * (scale_center.0 - width as i32 / 2) as f32 * (scale - 1.0) / width as f32;
            for x in 0..width {
                let fg = tiles[((y * width) + x) as usize].fg;
                let bg = tiles[((y * width) + x) as usize].bg;
                let glyph = tiles[((y * width) + x) as usize].glyph;
                let glyph_x = glyph % 16;
                let glyph_y = 16 - (glyph / 16);

                let glyph_left = f32::from(glyph_x) * glyph_size_x;
                let glyph_right = f32::from(glyph_x + 1) * glyph_size_x;
                let glyph_top = f32::from(glyph_y) * glyph_size_y;
                let glyph_bottom = (f32::from(glyph_y) - 0.95) * glyph_size_y;

                self.push_point(
                    screen_x + step_x,
                    screen_y + step_y,
                    fg,
                    bg,
                    glyph_right,
                    glyph_top,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x + step_x,
                    screen_y,
                    fg,
                    bg,
                    glyph_right,
                    glyph_bottom,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x,
                    screen_y,
                    fg,
                    bg,
                    glyph_left,
                    glyph_bottom,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x,
                    screen_y + step_y,
                    fg,
                    bg,
                    glyph_left,
                    glyph_top,
                    offset_x,
                    offset_y,
                );

                self.index_buffer[self.index_counter] = index_count;
                self.index_buffer[self.index_counter + 1] = 1 + index_count;
                self.index_buffer[self.index_counter + 2] = 3 + index_count;
                self.index_buffer[self.index_counter + 3] = 1 + index_count;
                self.index_buffer[self.index_counter + 4] = 2 + index_count;
                self.index_buffer[self.index_counter + 5] = 3 + index_count;
                self.index_counter += 6;

                index_count += 4;
                screen_x += step_x;
            }
            screen_y += step_y;
        }

        let be = BACKEND.lock().unwrap();
        let gl = be.gl.as_ref().unwrap();
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                &self.vertex_buffer.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                &self.index_buffer.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );
        }
    }

    pub fn gl_draw(&mut self, font: &Font, shader: &Shader, width: u32, height: u32) -> Result<()> {
        let be = BACKEND.lock().unwrap();
        let gl = be.gl.as_ref().unwrap();
        unsafe {
            // bind Texture
            font.bind_texture(gl);

            // render container
            shader.useProgram(gl);
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.draw_elements(
                glow::TRIANGLES,
                (width * height * 6) as i32,
                glow::UNSIGNED_INT,
                0,
            );
        }
        Ok(())
    }
}
