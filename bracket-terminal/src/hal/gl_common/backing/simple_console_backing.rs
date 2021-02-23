use crate::hal::{Font, Shader, VertexArray, VertexArrayEntry};
use crate::prelude::Tile;
use crate::BResult;
use bracket_color::prelude::RGBA;

pub struct SimpleConsoleBackend {
    vao: VertexArray,
    vertex_counter: usize,
    index_counter: usize,
}

impl SimpleConsoleBackend {
    pub fn new(width: usize, height: usize, gl: &glow::Context) -> SimpleConsoleBackend {
        let vertex_capacity: usize = (13 * width as usize * height as usize) * 4;
        let index_capacity: usize = 6 * width as usize * height as usize;
        let vao = SimpleConsoleBackend::init_gl_for_console(gl, vertex_capacity, index_capacity);
        let mut result = SimpleConsoleBackend {
            vao,
            vertex_counter: 0,
            index_counter: 0,
        };
        result.vao.vertex_buffer.resize(vertex_capacity, 0.0);
        result.vao.index_buffer.resize(index_capacity, 0);
        result
    }

    fn init_gl_for_console(
        gl: &glow::Context,
        vertex_capacity: usize,
        index_capacity: usize,
    ) -> VertexArray {
        VertexArray::float_builder(
            gl,
            &[
                VertexArrayEntry { index: 0, size: 3 }, // Position
                VertexArrayEntry { index: 1, size: 4 }, // Color
                VertexArrayEntry { index: 2, size: 4 }, // Background
                VertexArrayEntry { index: 3, size: 2 }, // Texture Pos
            ],
            vertex_capacity,
            index_capacity,
        )
    }

    /// Helper function to add all the elements required by the shader for a given point.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        &mut self,
        x: f32,
        y: f32,
        fg: RGBA,
        bg: RGBA,
        ux: f32,
        uy: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        self.vao.vertex_buffer[self.vertex_counter] = x + offset_x;
        self.vao.vertex_buffer[self.vertex_counter + 1] = y + offset_y;
        self.vao.vertex_buffer[self.vertex_counter + 2] = 0.0f32;
        self.vao.vertex_buffer[self.vertex_counter + 3] = fg.r;
        self.vao.vertex_buffer[self.vertex_counter + 4] = fg.g;
        self.vao.vertex_buffer[self.vertex_counter + 5] = fg.b;
        self.vao.vertex_buffer[self.vertex_counter + 6] = fg.a;
        self.vao.vertex_buffer[self.vertex_counter + 7] = bg.r;
        self.vao.vertex_buffer[self.vertex_counter + 8] = bg.g;
        self.vao.vertex_buffer[self.vertex_counter + 9] = bg.b;
        self.vao.vertex_buffer[self.vertex_counter + 10] = bg.a;
        self.vao.vertex_buffer[self.vertex_counter + 11] = ux;
        self.vao.vertex_buffer[self.vertex_counter + 12] = uy;
        self.vertex_counter += 13;
    }

    /// Rebuilds the OpenGL backing buffer.
    #[allow(clippy::too_many_arguments)]
    pub fn rebuild_vertices(
        &mut self,
        height: u32,
        width: u32,
        tiles: &[Tile],
        offset_x: f32,
        offset_y: f32,
        scale: f32,
        scale_center: (i32, i32),
        needs_resize: bool,
        font_dimensions_glyphs: (u32, u32),
    ) {
        if needs_resize {
            let vertex_capacity: usize = (13 * width as usize * height as usize) * 4;
            let index_capacity: usize = 6 * width as usize * height as usize;
            self.vao.vertex_buffer.clear();
            self.vao.index_buffer.clear();
            self.vao.vertex_buffer.resize(vertex_capacity, 0.0);
            self.vao.index_buffer.resize(index_capacity, 0);
        }

        self.vertex_counter = 0;
        self.index_counter = 0;
        let glyphs_on_font_x = font_dimensions_glyphs.0 as f32;
        let glyphs_on_font_y = font_dimensions_glyphs.1 as f32;
        let glyph_size_x: f32 = 1.0f32 / glyphs_on_font_x;
        let glyph_size_y: f32 = 1.0f32 / glyphs_on_font_y;

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
                let glyph_x = glyph % font_dimensions_glyphs.0 as u16;
                let glyph_y =
                    font_dimensions_glyphs.1 as u16 - (glyph / font_dimensions_glyphs.0 as u16);

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

                self.vao.index_buffer[self.index_counter] = index_count;
                self.vao.index_buffer[self.index_counter + 1] = 1 + index_count;
                self.vao.index_buffer[self.index_counter + 2] = 3 + index_count;
                self.vao.index_buffer[self.index_counter + 3] = 1 + index_count;
                self.vao.index_buffer[self.index_counter + 4] = 2 + index_count;
                self.vao.index_buffer[self.index_counter + 5] = 3 + index_count;
                self.index_counter += 6;

                index_count += 4;
                screen_x += step_x;
            }
            screen_y += step_y;
        }

        self.vao.upload_buffers();
    }

    pub fn gl_draw(&mut self, font: &Font, shader: &Shader) -> BResult<()> {
        self.vao.draw_elements(shader, font);
        Ok(())
    }
}
