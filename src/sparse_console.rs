use super::{gui_helpers, rex::XpColor, rex::XpLayer, Console, Font, Shader, RGB};
//use gl::types::*;
use super::gl;
use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

/// Internal storage structure for sparse tiles.
pub struct SparseTile {
    pub idx: usize,
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

/// A sparse console. Rather than storing every cell on the screen, it stores just cells that have
/// data.
pub struct SparseConsole {
    pub width: u32,
    pub height: u32,

    // Private
    tiles: Vec<SparseTile>,
    is_dirty: bool,

    // To handle offset tiles for people who want thin walls between tiles
    offset_x: f32,
    offset_y: f32,

    // GL Stuff
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<i32>,
    vbo: u32,
    vao: u32,
    ebo: u32,
}

impl SparseConsole {
    /// Initializes the console.
    pub fn init(width: u32, height: u32, gl: &gl::Gles2) -> Box<SparseConsole> {
        // Console backing init

        let (vbo, vao, ebo) = SparseConsole::init_gl_for_console(gl);

        let new_console = SparseConsole {
            width,
            height,
            vbo,
            vao,
            ebo,
            tiles: Vec::new(),
            is_dirty: true,
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            offset_x: 0.0,
            offset_y: 0.0,
        };

        Box::new(new_console)
    }

    /// Initializes OpenGL for the sparse console.
    fn init_gl_for_console(gl: &gl::Gles2) -> (u32, u32, u32) {
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

        unsafe {
            // Generate buffers and arrays, as well as attributes.
            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);
            gl.GenBuffers(1, &mut ebo);

            gl.BindVertexArray(vao);

            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

            let stride = 11 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl.EnableVertexAttribArray(0);
            // color attribute
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl.EnableVertexAttribArray(1);
            // bgcolor attribute
            gl.VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl.EnableVertexAttribArray(2);
            // texture coord attribute
            gl.VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (9 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl.EnableVertexAttribArray(3);
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
        vertex_buffer.push(x);
        vertex_buffer.push(y);
        vertex_buffer.push(0.0);
        vertex_buffer.push(fg.r);
        vertex_buffer.push(fg.g);
        vertex_buffer.push(fg.b);
        vertex_buffer.push(bg.r);
        vertex_buffer.push(bg.g);
        vertex_buffer.push(bg.b);
        vertex_buffer.push(ux);
        vertex_buffer.push(uy);
    }

    /// Helper to build vertices for the sparse grid.
    fn rebuild_vertices(&mut self, gl: &gl::Gles2) {
        if self.tiles.is_empty() {
            return;
        }

        self.vertex_buffer.clear();
        self.index_buffer.clear();

        let glyph_size_x: f32 = 1.0 / 16.0;
        let glyph_size_y: f32 = 1.0 / 16.0;

        let step_x: f32 = 2.0 / self.width as f32;
        let step_y: f32 = 2.0 / self.height as f32;

        let mut index_count: i32 = 0;
        for t in self.tiles.iter() {
            let x = t.idx % self.width as usize;
            let y = t.idx / self.width as usize;

            let screen_x = ((step_x * x as f32) - 1.0) + self.offset_x;
            let screen_y = ((step_y * y as f32) - 1.0) + self.offset_y;
            let fg = t.fg;
            let bg = t.bg;
            let glyph = t.glyph;
            let glyph_x = glyph % 16;
            let glyph_y = 16 - (glyph / 16);

            let glyph_left = glyph_x as f32 * glyph_size_x;
            let glyph_right = (glyph_x + 1) as f32 * glyph_size_x;
            let glyph_top = glyph_y as f32 * glyph_size_y;
            let glyph_bottom = (glyph_y - 1) as f32 * glyph_size_y;

            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x + step_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_right,
                glyph_top,
            );
            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x + step_x,
                screen_y,
                fg,
                bg,
                glyph_right,
                glyph_bottom,
            );
            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x,
                screen_y,
                fg,
                bg,
                glyph_left,
                glyph_bottom,
            );
            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_left,
                glyph_top,
            );

            self.index_buffer.push(0 + index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(3 + index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(2 + index_count);
            self.index_buffer.push(3 + index_count);

            index_count += 4;
        }

        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (self.vertex_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &self.vertex_buffer[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.index_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &self.index_buffer[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Console for SparseConsole {
    /// If the console has changed, rebuild the vertex buffer.
    fn rebuild_if_dirty(&mut self, gl: &gl::Gles2) {
        if self.is_dirty {
            self.rebuild_vertices(gl);
            self.is_dirty = false;
        }
    }

    /// Draws the console to OpenGL.
    fn gl_draw(&mut self, font: &Font, shader: &Shader, gl: &gl::Gles2) {
        unsafe {
            // bind Texture
            font.bind_texture(gl);

            // render container
            shader.useProgram(gl);
            gl.BindVertexArray(self.vao);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl.DrawElements(
                gl::TRIANGLES,
                (self.tiles.len() * 6) as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        self.is_dirty = false;
    }

    /// Translates x/y to an index entry. Not really useful.
    fn at(&self, x: i32, y: i32) -> usize {
        (((self.height - 1 - y as u32) * self.width) + x as u32) as usize
    }

    /// Clear the screen.
    fn cls(&mut self) {
        self.is_dirty = true;
        self.tiles.clear();
    }

    /// Clear the screen. Since we don't HAVE a background, it doesn't use it.
    fn cls_bg(&mut self, _background: RGB) {
        self.is_dirty = true;
        self.tiles.clear();
    }

    /// Prints a string to an x/y position.
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);
        for i in 0..bytes.len() {
            self.tiles.push(SparseTile {
                idx,
                glyph: bytes[i],
                fg: RGB::from_f32(1.0, 1.0, 1.0),
                bg: RGB::from_f32(0.0, 0.0, 0.0),
            });
            idx += 1;
        }
    }

    /// Prints a string to an x/y position, with foreground and background colors.
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);
        for i in 0..bytes.len() {
            self.tiles.push(SparseTile {
                idx,
                glyph: bytes[i],
                fg,
                bg,
            });
            idx += 1;
        }
    }

    /// Sets a single cell in the console
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        let idx = self.at(x, y);
        self.tiles.push(SparseTile {
            idx,
            glyph: glyph,
            fg,
            bg,
        });
    }

    /// Sets a single cell in the console's background
    fn set_bg(&mut self, x: i32, y: i32, bg: RGB) {
        let idx = self.at(x, y);
        self.tiles[idx].bg = bg;
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        gui_helpers::draw_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        gui_helpers::draw_box_double(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a horizontal progress bar
    fn draw_bar_horizontal(
        &mut self,
        sx: i32,
        sy: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        gui_helpers::draw_bar_horizontal(self, sx, sy, width, n, max, fg, bg);
    }

    /// Draws a vertical progress bar
    fn draw_bar_vertical(
        &mut self,
        sx: i32,
        sy: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        gui_helpers::draw_bar_vertical(self, sx, sy, height, n, max, fg, bg);
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    fn print_centered(&mut self, y: i32, text: &str) {
        self.is_dirty = true;
        self.print(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            text,
        );
    }

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered(&mut self, y: i32, fg: RGB, bg: RGB, text: &str) {
        self.is_dirty = true;
        self.print_color(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            fg,
            bg,
            text,
        );
    }

    /// Saves the layer to an XpFile structure
    fn to_xp_layer(&self) -> XpLayer {
        let mut layer = XpLayer::new(self.width as usize, self.height as usize);

        // Clear all to transparent
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = layer.get_mut(x as usize, y as usize).unwrap();
                cell.bg = XpColor::TRANSPARENT;
            }
        }

        for c in self.tiles.iter() {
            let x = c.idx % self.width as usize;
            let y = c.idx / self.width as usize;
            let cell = layer.get_mut(x as usize, y as usize).unwrap();
            cell.ch = c.glyph as u32;
            cell.fg = c.fg.to_xp();
            cell.bg = c.bg.to_xp();
        }

        layer
    }

    /// Sets an offset to total console rendering, useful for layers that
    /// draw between tiles. Offsets are specified as a percentage of total
    /// character size; so -0.5 will offset half a character to the left/top.
    fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x * (2.0 / self.width as f32);
        self.offset_y = y * (2.0 / self.height as f32);
    }
}
