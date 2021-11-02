use crate::hal::scaler::FontScaler;
use crate::hal::{Font, Shader, VertexArray, VertexArrayEntry, BACKEND};
use crate::prelude::Tile;
use crate::BResult;
use bracket_color::prelude::RGBA;

pub struct SimpleConsoleBackend {
    vao: VertexArray,
    vertex_counter: usize,
    index_counter: usize,
    previous_console : Option<Vec<Tile>>,
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
            previous_console: None,
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
        tiles: &Vec<Tile>,
        offset_x: f32,
        offset_y: f32,
        scale: f32,
        scale_center: (i32, i32),
        needs_resize: bool,
        font_scaler: FontScaler,
    ) {
        if !needs_resize {
            if let Some(old) = &self.previous_console {
                if old.len() == tiles.len() {
                    let no_change = tiles.iter().zip(old.iter()).all(|(a, b)| *a==*b);
                    if no_change {
                        return;
                    }
                }
            }
        }

        if needs_resize {
            let vertex_capacity: usize = (13 * width as usize * height as usize) * 4;
            let index_capacity: usize = 6 * width as usize * height as usize;
            self.vao.vertex_buffer.clear();
            self.vao.index_buffer.clear();
            self.vao.vertex_buffer.resize(vertex_capacity, 0.0);
            self.vao.index_buffer.resize(index_capacity, 0);
            self.previous_console = None;
        }

        self.vertex_counter = 0;
        self.index_counter = 0;

        let (step_x, step_y, left_x, top_y) = {
            let be = BACKEND.lock();
            let (step_x, step_y) = be.screen_scaler.calc_step(width, height, scale);
            let (left_x, top_y) = be.screen_scaler.top_left_pixel();

            (step_x, step_y, left_x, top_y)
        };

        //let step_x: f32 = scale * 2.0f32 / width as f32;
        //let step_y: f32 = scale * 2.0f32 / height as f32;

        let mut index_count: i32 = 0;
        let mut screen_y: f32 = top_y;
        for y in 0..height {
            let mut screen_x: f32 = left_x;
            for x in 0..width {
                let fg = tiles[((y * width) + x) as usize].fg;
                let bg = tiles[((y * width) + x) as usize].bg;
                let glyph = tiles[((y * width) + x) as usize].glyph;
                let gp = font_scaler.glyph_position(glyph);

                self.push_point(
                    screen_x + step_x,
                    screen_y + step_y,
                    fg,
                    bg,
                    gp.glyph_right,
                    gp.glyph_top,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x + step_x,
                    screen_y,
                    fg,
                    bg,
                    gp.glyph_right,
                    gp.glyph_bottom,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x,
                    screen_y,
                    fg,
                    bg,
                    gp.glyph_left,
                    gp.glyph_bottom,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x,
                    screen_y + step_y,
                    fg,
                    bg,
                    gp.glyph_left,
                    gp.glyph_top,
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
        self.previous_console = Some(tiles.clone());
    }

    pub fn gl_draw(&mut self, font: &Font, shader: &Shader) -> BResult<()> {
        self.vao.draw_elements(shader, font);
        Ok(())
    }
}
