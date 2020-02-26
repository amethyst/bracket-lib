use super::BACKEND;
use crate::hal::{font::Font, shader::Shader};
use crate::sparse_console::SparseTile;
use crate::Result;
use bracket_color::prelude::RGB;
use glow::HasContext;
use std::mem;

pub struct SparseConsoleBackend {
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<i32>,
    vbo: u32,
    vao: u32,
    ebo: u32,
}

impl SparseConsoleBackend {
    pub fn new(_width: usize, _height: usize, gl: &glow::Context) -> SparseConsoleBackend {
        let (vbo, vao, ebo) = SparseConsoleBackend::init_gl_for_console(gl);
        SparseConsoleBackend {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            vbo,
            vao,
            ebo,
        }
    }

    fn init_gl_for_console(gl: &glow::Context) -> (u32, u32, u32) {
        let (vbo, vao, ebo);

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
        };

        (vbo, vao, ebo)
    }

    /// Helper to push a point to the shader.
    fn push_point(
        vertex_buffer: &mut Vec<f32>,
        x: f32,
        y: f32,
        fg: RGB,
        bg: RGB,
        ux: f32,
        uy: f32,
    ) {
        vertex_buffer.extend_from_slice(&[x, y, 0.0, fg.r, fg.g, fg.b, bg.r, bg.g, bg.b, ux, uy]);
    }

    /// Helper to build vertices for the sparse grid.
    pub fn rebuild_vertices(
        &mut self,
        height: u32,
        width: u32,
        offset_x: f32,
        offset_y: f32,
        scale: f32,
        scale_center: (i32, i32),
        tiles: &[SparseTile],
    ) {
        if tiles.is_empty() {
            return;
        }

        self.vertex_buffer.clear();
        self.index_buffer.clear();

        let glyph_size_x: f32 = 1.0 / 16.0;
        let glyph_size_y: f32 = 1.0 / 16.0;

        let step_x: f32 = scale * 2.0 / width as f32;
        let step_y: f32 = scale * 2.0 / height as f32;

        let mut index_count: i32 = 0;
        let screen_x_start: f32 = -1.0 * scale
            - 2.0 * (scale_center.0 - width as i32 / 2) as f32 * (scale - 1.0) / width as f32;
        let screen_y_start: f32 = -1.0 * scale
            + 2.0 * (scale_center.1 - height as i32 / 2) as f32 * (scale - 1.0) / height as f32;
        for t in tiles.iter() {
            let x = t.idx % width as usize;
            let y = t.idx / width as usize;

            let screen_x = ((step_x * x as f32) + screen_x_start) + offset_x;
            let screen_y = ((step_y * y as f32) + screen_y_start) + offset_y;
            let fg = t.fg;
            let bg = t.bg;
            let glyph = t.glyph;
            let glyph_x = glyph % 16;
            let glyph_y = 16 - (glyph / 16);

            let glyph_left = f32::from(glyph_x) * glyph_size_x;
            let glyph_right = f32::from(glyph_x + 1) * glyph_size_x;
            let glyph_top = f32::from(glyph_y) * glyph_size_y;
            let glyph_bottom = f32::from(glyph_y - 1) * glyph_size_y;

            SparseConsoleBackend::push_point(
                &mut self.vertex_buffer,
                screen_x + step_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_right,
                glyph_top,
            );
            SparseConsoleBackend::push_point(
                &mut self.vertex_buffer,
                screen_x + step_x,
                screen_y,
                fg,
                bg,
                glyph_right,
                glyph_bottom,
            );
            SparseConsoleBackend::push_point(
                &mut self.vertex_buffer,
                screen_x,
                screen_y,
                fg,
                bg,
                glyph_left,
                glyph_bottom,
            );
            SparseConsoleBackend::push_point(
                &mut self.vertex_buffer,
                screen_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_left,
                glyph_top,
            );

            self.index_buffer.push(index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(3 + index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(2 + index_count);
            self.index_buffer.push(3 + index_count);

            index_count += 4;
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

    pub fn gl_draw(&mut self, font: &Font, shader: &Shader, tiles: &[SparseTile]) -> Result<()> {
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
                (tiles.len() * 6) as i32,
                glow::UNSIGNED_INT,
                0,
            );
        }
        Ok(())
    }
}
