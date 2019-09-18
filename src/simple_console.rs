use super::{color, gui_helpers, rex::XpLayer, Console, Font, Shader, Tile, RGB};
//use glow::types::*;
use glow::HasContext;
use std::mem;

/// A simple console with background color.
pub struct SimpleConsole {
    pub width: u32,
    pub height: u32,

    // Private
    tiles: Vec<Tile>,
    is_dirty: bool,
    vertex_counter: usize,
    index_counter: usize,

    // To handle offset tiles for people who want thin walls between tiles
    offset_x: f32,
    offset_y: f32,

    // GL Stuff
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<i32>,

    #[cfg(not(target_arch = "wasm32"))]
    vbo: u32,

    #[cfg(not(target_arch = "wasm32"))]
    vao: u32,

    #[cfg(not(target_arch = "wasm32"))]
    ebo: u32,

    #[cfg(target_arch = "wasm32")]
    vbo: glow::WebBufferKey,

    #[cfg(target_arch = "wasm32")]
    vao: glow::WebVertexArrayKey,

    #[cfg(target_arch = "wasm32")]
    ebo: glow::WebBufferKey,
}

impl SimpleConsole {
    /// Initializes a console, ready to add to RLTK's console list.
    pub fn init(width: u32, height: u32, gl: &glow::Context) -> Box<SimpleConsole> {
        // Console backing init
        let num_tiles: usize = (width * height) as usize;
        let mut tiles: Vec<Tile> = Vec::with_capacity(num_tiles);
        for _ in 0..num_tiles {
            tiles.push(Tile {
                glyph: 0,
                fg: RGB::named(color::WHITE),
                bg: RGB::named(color::BLACK),
            });
        }

        let (vbo, vao, ebo) = SimpleConsole::init_gl_for_console(gl);

        let vertex_capacity: usize = (11 * width as usize * height as usize) * 4;
        let index_capacity: usize = 6 * width as usize * height as usize;

        let mut new_console = SimpleConsole {
            width,
            height,
            vbo,
            vao,
            ebo,
            tiles,
            is_dirty: true,
            vertex_buffer: Vec::with_capacity(vertex_capacity),
            index_buffer: Vec::with_capacity(index_capacity),
            vertex_counter: 0,
            index_counter: 0,
            offset_x: 0.0,
            offset_y: 0.0,
        };

        for _ in 0..vertex_capacity {
            new_console.vertex_buffer.push(0.0);
        }
        for _ in 0..index_capacity {
            new_console.index_buffer.push(0);
        }

        Box::new(new_console)
    }

    /// Sets up the OpenGL backing.
    #[cfg(not(target_arch = "wasm32"))]
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
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32); // set texture wrapping to glow::REPEAT (default wrapping method)
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            // set texture filtering parameters
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
        };

        (vbo, vao, ebo)
    }

    #[cfg(target_arch = "wasm32")]
    fn init_gl_for_console(
        gl: &glow::Context,
    ) -> (
        glow::WebBufferKey,
        glow::WebVertexArrayKey,
        glow::WebBufferKey,
    ) {
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
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32); // set texture wrapping to glow::REPEAT (default wrapping method)
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            // set texture filtering parameters
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
        };

        (vbo, vao, ebo)
    }

    /// Helper function to add all the elements required by the shader for a given point.
    fn push_point(&mut self, x: f32, y: f32, fg: RGB, bg: RGB, ux: f32, uy: f32) {
        self.vertex_buffer[self.vertex_counter] = x + self.offset_x;
        self.vertex_buffer[self.vertex_counter + 1] = y + self.offset_y;
        self.vertex_buffer[self.vertex_counter + 2] = 0.0;
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
    fn rebuild_vertices(&mut self, gl: &glow::Context) {
        self.vertex_counter = 0;
        self.index_counter = 0;
        let glyph_size_x: f32 = 1.0 / 16.0;
        let glyph_size_y: f32 = 1.0 / 16.0;

        let step_x: f32 = 2.0 / self.width as f32;
        let step_y: f32 = 2.0 / self.height as f32;

        let mut index_count: i32 = 0;
        let mut screen_y: f32 = -1.0;
        for y in 0..self.height {
            let mut screen_x: f32 = -1.0;
            for x in 0..self.width {
                let fg = self.tiles[((y * self.width) + x) as usize].fg;
                let bg = self.tiles[((y * self.width) + x) as usize].bg;
                let glyph = self.tiles[((y * self.width) + x) as usize].glyph;
                let glyph_x = glyph % 16;
                let glyph_y = 16 - (glyph / 16);

                let glyph_left = f32::from(glyph_x) * glyph_size_x;
                let glyph_right = f32::from(glyph_x + 1) * glyph_size_x;
                let glyph_top = f32::from(glyph_y) * glyph_size_y;
                let glyph_bottom = f32::from(glyph_y - 1) * glyph_size_y;

                self.push_point(
                    screen_x + step_x,
                    screen_y + step_y,
                    fg,
                    bg,
                    glyph_right,
                    glyph_top,
                );
                self.push_point(
                    screen_x + step_x,
                    screen_y,
                    fg,
                    bg,
                    glyph_right,
                    glyph_bottom,
                );
                self.push_point(screen_x, screen_y, fg, bg, glyph_left, glyph_bottom);
                self.push_point(screen_x, screen_y + step_y, fg, bg, glyph_left, glyph_top);

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
}

impl Console for SimpleConsole {
    /// Check if the console has changed, and if it has rebuild the backing buffer.
    fn rebuild_if_dirty(&mut self, gl: &glow::Context) {
        if self.is_dirty {
            self.rebuild_vertices(gl);
            self.is_dirty = false;
        }
    }

    /// Sends the console to OpenGL.
    fn gl_draw(&mut self, font: &Font, shader: &Shader, gl: &glow::Context) {
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
                (self.width * self.height * 6) as i32,
                glow::UNSIGNED_INT,
                0,
            );
        }
        self.is_dirty = false;
    }

    /// Translate an x/y into an array index.
    fn at(&self, x: i32, y: i32) -> usize {
        (((self.height - 1 - y as u32) * self.width) + x as u32) as usize
    }

    /// Clears the screen.
    fn cls(&mut self) {
        self.is_dirty = true;
        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = RGB::named(color::WHITE);
            tile.bg = RGB::named(color::BLACK);
        }
    }

    /// Clears the screen with a background color.
    fn cls_bg(&mut self, background: RGB) {
        self.is_dirty = true;
        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = RGB::named(color::WHITE);
            tile.bg = background;
        }
    }

    /// Prints a string at x/y.
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);
        for glyph in bytes {
            if idx < self.tiles.len() {
                self.tiles[idx].glyph = glyph;
                idx += 1;
            }
        }
    }

    /// Prints a string at x/y, with foreground and background colors.
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);
        for glyph in bytes {
            if idx < self.tiles.len() {
                self.tiles[idx].glyph = glyph;
                self.tiles[idx].bg = bg;
                self.tiles[idx].fg = fg;
                idx += 1;
            }
        }
    }

    /// Sets a single cell in the console
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        let idx = self.at(x, y);
        self.tiles[idx].glyph = glyph;
        self.tiles[idx].fg = fg;
        self.tiles[idx].bg = bg;
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

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = layer.get_mut(x as usize, y as usize).unwrap();
                let idx = self.at(x as i32, y as i32);
                cell.ch = u32::from(self.tiles[idx].glyph);
                cell.fg = self.tiles[idx].fg.to_xp();
                cell.bg = self.tiles[idx].bg.to_xp();
            }
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
