use crate::hal::{Font, Shader, VertexArray, VertexArrayEntry};
use crate::prelude::FlexiTile;
use crate::Result;
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::PointF;

pub struct FancyConsoleBackend {
    vao: VertexArray,
}

impl FancyConsoleBackend {
    pub fn new(_width: usize, _height: usize, gl: &glow::Context) -> FancyConsoleBackend {
        let vao = FancyConsoleBackend::init_gl_for_console(gl, 1000, 1000);
        FancyConsoleBackend { vao }
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
                VertexArrayEntry { index: 3, size: 2 }, // Texture Coordinate
                VertexArrayEntry { index: 4, size: 3 }, // Rotation
                VertexArrayEntry { index: 5, size: 2 }, // Scale
            ],
            vertex_capacity,
            index_capacity,
        )
    }

    /// Helper to push a point to the shader.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        vertex_buffer: &mut Vec<f32>,
        x: f32,
        y: f32,
        fg: RGBA,
        bg: RGBA,
        ux: f32,
        uy: f32,
        rotation: f32,
        screen_x: f32,
        screen_y: f32,
        scale: PointF,
    ) {
        vertex_buffer.extend_from_slice(&[
            x, y, 0.0, fg.r, fg.g, fg.b, fg.a, bg.r, bg.g, bg.b, bg.a, ux, uy, rotation, screen_x,
            screen_y, scale.x, scale.y,
        ]);
    }

    /// Helper to build vertices for the sparse grid.
    #[allow(clippy::too_many_arguments)]
    pub fn rebuild_vertices(
        &mut self,
        height: u32,
        width: u32,
        offset_x: f32,
        offset_y: f32,
        scale: f32,
        scale_center: (i32, i32),
        tiles: &[FlexiTile],
        font_dimensions_glyphs: (u32, u32),
    ) {
        if tiles.is_empty() {
            return;
        }

        self.vao.vertex_buffer.clear();
        self.vao.index_buffer.clear();

        let glyphs_on_font_x = font_dimensions_glyphs.0 as f32;
        let glyphs_on_font_y = font_dimensions_glyphs.1 as f32;
        let glyph_size_x: f32 = 1.0 / glyphs_on_font_x;
        let glyph_size_y: f32 = 1.0 / glyphs_on_font_y;

        let step_x: f32 = scale * 2.0 / width as f32;
        let step_y: f32 = scale * 2.0 / height as f32;

        let mut index_count: i32 = 0;
        let screen_x_start: f32 = -1.0 * scale
            - 2.0 * (scale_center.0 - width as i32 / 2) as f32 * (scale - 1.0) / width as f32;
        let screen_y_start: f32 = -1.0 * scale
            + 2.0 * (scale_center.1 - height as i32 / 2) as f32 * (scale - 1.0) / height as f32;

        for t in tiles.iter() {
            let x = t.position.x;
            let y = t.position.y;

            let screen_x = ((step_x * x) + screen_x_start) + offset_x;
            let screen_y = ((step_y * y) + screen_y_start) + offset_y;
            let fg = t.fg;
            let bg = t.bg;
            let glyph = t.glyph;
            let glyph_x = glyph % font_dimensions_glyphs.0 as u16;
            let glyph_y =
                font_dimensions_glyphs.1 as u16 - (glyph / font_dimensions_glyphs.0 as u16);

            let glyph_left = f32::from(glyph_x) * glyph_size_x;
            let glyph_right = f32::from(glyph_x + 1) * glyph_size_x;
            let glyph_top = f32::from(glyph_y) * glyph_size_y;
            let glyph_bottom = f32::from(glyph_y - 1) * glyph_size_y;

            let rot_center_x = screen_x + (step_x / 2.0);
            let rot_center_y = screen_y + (step_y / 2.0);

            FancyConsoleBackend::push_point(
                &mut self.vao.vertex_buffer,
                screen_x + step_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_right,
                glyph_top,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
            );
            FancyConsoleBackend::push_point(
                &mut self.vao.vertex_buffer,
                screen_x + step_x,
                screen_y,
                fg,
                bg,
                glyph_right,
                glyph_bottom,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
            );
            FancyConsoleBackend::push_point(
                &mut self.vao.vertex_buffer,
                screen_x,
                screen_y,
                fg,
                bg,
                glyph_left,
                glyph_bottom,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
            );
            FancyConsoleBackend::push_point(
                &mut self.vao.vertex_buffer,
                screen_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_left,
                glyph_top,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
            );

            self.vao.index_buffer.push(index_count);
            self.vao.index_buffer.push(1 + index_count);
            self.vao.index_buffer.push(3 + index_count);
            self.vao.index_buffer.push(1 + index_count);
            self.vao.index_buffer.push(2 + index_count);
            self.vao.index_buffer.push(3 + index_count);

            index_count += 4;
        }

        self.vao.upload_buffers();
    }

    pub fn gl_draw(&mut self, font: &Font, shader: &Shader) -> Result<()> {
        self.vao.draw_elements(shader, font);
        Ok(())
    }
}
