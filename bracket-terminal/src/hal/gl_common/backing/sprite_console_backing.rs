use crate::hal::{
    vao_float_builder, BufferId, Font, Shader, VertexArrayEntry, VertexArrayId, BACKEND,
};
use crate::prelude::{RenderSprite, SpriteSheet};
use crate::Result;
use bracket_color::prelude::RGBA;
use glow::HasContext;

pub struct SpriteConsoleBackend {
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<i32>,
    vbo: BufferId,
    vao: VertexArrayId,
    ebo: BufferId,
}

impl SpriteConsoleBackend {
    pub fn new(_width: usize, _height: usize, gl: &glow::Context) -> SpriteConsoleBackend {
        let (vbo, vao, ebo) = SpriteConsoleBackend::init_gl_for_console(gl);
        SpriteConsoleBackend {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            vbo,
            vao,
            ebo,
        }
    }

    fn init_gl_for_console(gl: &glow::Context) -> (BufferId, VertexArrayId, BufferId) {
        vao_float_builder(
            gl,
            &[
                VertexArrayEntry { index: 0, size: 2 }, // Position
                VertexArrayEntry { index: 1, size: 2 }, // Transform (x/y)
                VertexArrayEntry { index: 2, size: 4 }, // Color
                VertexArrayEntry { index: 3, size: 2 }, // Texture Coordinate
                VertexArrayEntry { index: 4, size: 2 }, // Scale
            ],
        )
    }

    /// Helper to push a point to the shader.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        vertex_buffer: &mut Vec<f32>,
        rel_x: f32,
        rel_y: f32,
        trans_x: f32,
        trans_y: f32,
        fg: RGBA,
        ux: f32,
        uy: f32,
        scale: (f32, f32),
    ) {
        vertex_buffer.extend_from_slice(&[
            rel_x, rel_y, trans_x, trans_y, fg.r, fg.g, fg.b, fg.a, ux, uy, scale.0, scale.1,
        ]);
    }

    /// Helper to build vertices for the sparse grid.
    #[allow(clippy::too_many_arguments)]
    pub fn rebuild_vertices(
        &mut self,
        height: u32,
        width: u32,
        sprites: &[RenderSprite],
        sprite_sheet: &SpriteSheet,
    ) {
        if sprites.is_empty() {
            return;
        }

        let scale_x = 1.0 / (width as f32 * 0.5);
        let scale_y = 1.0 / (height as f32 * 0.5);

        let offset_x = (width as f32 / 2.0) * scale_x;
        let offset_y = (height as f32 / 2.0) * scale_y;

        self.vertex_buffer.clear();
        self.index_buffer.clear();

        let mut index_count: i32 = 0;
        for s in sprites.iter() {
            let sprite_sheet = &sprite_sheet;
            let ss_x = 1.0 / sprite_sheet.backing.as_ref().unwrap().width as f32;
            let ss_y = 1.0 / sprite_sheet.backing.as_ref().unwrap().height as f32;
            let sprite_pos = sprite_sheet.sprites[s.index].sheet_location;
            let sprite_left = sprite_pos.x1 as f32 * ss_x;
            let sprite_bottom = sprite_pos.y1 as f32 * ss_y;
            let sprite_right = sprite_pos.x2 as f32 * ss_x;
            let sprite_top = sprite_pos.y2 as f32 * ss_y;

            let render_width = s.destination.width() as f32;
            let sprite_width = sprite_pos.width() as f32;

            let render_height = s.destination.height() as f32;
            let sprite_height = sprite_pos.height() as f32;
            let scale = (
                (render_width / sprite_width) * scale_x,
                (render_height / sprite_height) * scale_y,
            );

            let mut sd = s.destination;
            sd.y2 = height as i32 - s.destination.y1;
            sd.y1 = height as i32 - s.destination.y2;

            SpriteConsoleBackend::push_point(
                &mut self.vertex_buffer,
                0.5,
                0.5,
                (sd.x2 as f32 * scale_x) - offset_x,
                (sd.y2 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_right,
                sprite_top,
                scale,
            );
            SpriteConsoleBackend::push_point(
                &mut self.vertex_buffer,
                0.5,
                -0.5,
                (sd.x2 as f32 * scale_x) - offset_x,
                (sd.y1 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_right,
                sprite_bottom,
                scale,
            );
            SpriteConsoleBackend::push_point(
                &mut self.vertex_buffer,
                -0.5,
                -0.5,
                (sd.x1 as f32 * scale_x) - offset_x,
                (sd.y1 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_left,
                sprite_bottom,
                scale,
            );
            SpriteConsoleBackend::push_point(
                &mut self.vertex_buffer,
                -0.5,
                0.5,
                (sd.x1 as f32 * scale_x) - offset_x,
                (sd.y2 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_left,
                sprite_top,
                scale,
            );

            self.index_buffer.push(index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(3 + index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(2 + index_count);
            self.index_buffer.push(3 + index_count);

            index_count += 4;
        }

        let be = BACKEND.lock();
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

    pub fn gl_draw(&mut self, font: &Font, shader: &Shader, tiles: &[RenderSprite]) -> Result<()> {
        let be = BACKEND.lock();
        let gl = be.gl.as_ref().unwrap();
        unsafe {
            // bind Texture
            font.bind_texture(gl);

            // render container
            shader.useProgram(gl);
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.draw_elements(
                glow::TRIANGLES,
                (tiles.len() * 6) as i32,
                glow::UNSIGNED_INT,
                0,
            );
            gl.disable(glow::BLEND);
        }
        Ok(())
    }
}
