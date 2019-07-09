use super::{Console, Tile, RGB, color, Font, Shader};
//use gl::types::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use super::gl;
use gl::types::*;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct SimpleConsole {
    pub width :u32,
    pub height: u32,

    // Private
    tiles: Vec<Tile>,
    is_dirty: bool,
    vertex_counter : usize,
    index_counter : usize,

    // GL Stuff
    vertex_buffer : Vec<f32>,
    index_buffer : Vec<i32>,
    VBO: u32,
    VAO: u32,
    EBO: u32
}

#[allow(dead_code)]
impl SimpleConsole {
    #[allow(non_snake_case)]
    pub fn init(width:u32, height: u32, gl : &gl::Gles2) -> Box<SimpleConsole> {
        // Console backing init
        let num_tiles : usize = (width * height) as usize;
        let mut tiles : Vec<Tile> = Vec::with_capacity(num_tiles);
        for _i in 0..num_tiles {
            tiles.push(Tile{glyph: 0, fg: RGB::named(color::WHITE), bg: RGB::named(color::BLACK)});
        }

        let (VBO, VAO, EBO) = SimpleConsole::init_gl_for_console(gl);

        let vertex_capacity : usize = (11 * width as usize * height as usize) * 4;
        let index_capacity : usize = 6 * width as usize * height as usize;

        let mut new_console = SimpleConsole{
            width: width, 
            height: height, 
            VBO: VBO,
            VAO: VAO,
            EBO: EBO,
            tiles: tiles,
            is_dirty: true,
            vertex_buffer : Vec::with_capacity(vertex_capacity),
            index_buffer : Vec::with_capacity(index_capacity),
            vertex_counter : 0,
            index_counter: 0
        };

        for _i in 0..vertex_capacity { new_console.vertex_buffer.push(0.0); }
        for _i in 0..index_capacity { new_console.index_buffer.push(0); }

        Box::new(new_console)
    }

    #[allow(non_snake_case)]
    fn init_gl_for_console(gl : &gl::Gles2) -> (u32, u32, u32) {
        let mut texture = 0;
        let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
        
        unsafe {
            // Generate buffers and arrays, as well as attributes.
            gl.GenVertexArrays(1, &mut VAO);
            gl.GenBuffers(1, &mut VBO);
            gl.GenBuffers(1, &mut EBO);

            gl.BindVertexArray(VAO);

            gl.BindBuffer(gl::ARRAY_BUFFER, VBO);

            let stride = 11 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl.EnableVertexAttribArray(0);
            // color attribute
            gl.VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl.EnableVertexAttribArray(1);
             // bgcolor attribute
            gl.VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
            gl.EnableVertexAttribArray(2);
            // texture coord attribute
            gl.VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, stride, (9 * mem::size_of::<GLfloat>()) as *const c_void);
            gl.EnableVertexAttribArray(3);
            
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
            // set the texture wrapping parameters
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        };

        (VBO, VAO, EBO)
    }

    fn push_point(&mut self, x:f32, y:f32, fg:RGB, bg:RGB, ux:f32, uy:f32) {
        self.vertex_buffer[self.vertex_counter] = x;
        self.vertex_buffer[self.vertex_counter+1] = y;
        self.vertex_buffer[self.vertex_counter+2] = 0.0;
        self.vertex_buffer[self.vertex_counter+3] = fg.r;
        self.vertex_buffer[self.vertex_counter+4] = fg.g;
        self.vertex_buffer[self.vertex_counter+5] = fg.b;
        self.vertex_buffer[self.vertex_counter+6] = bg.r;
        self.vertex_buffer[self.vertex_counter+7] = bg.g;
        self.vertex_buffer[self.vertex_counter+8] = bg.b;
        self.vertex_buffer[self.vertex_counter+9] = ux;
        self.vertex_buffer[self.vertex_counter+10] = uy;
        self.vertex_counter += 11;
    }

    fn rebuild_vertices(&mut self, gl : &gl::Gles2) {
        self.vertex_counter = 0;
        self.index_counter = 0;
        let glyph_size_x : f32 = 1.0 / 16.0;
        let glyph_size_y : f32 = 1.0 / 16.0;

        let step_x : f32 = 2.0 / self.width as f32;
        let step_y : f32 = 2.0 / self.height as f32;

        let mut index_count : i32 = 0;
        let mut screen_y : f32 = -1.0;
        for y in 0 .. self.height {
            let mut screen_x : f32 = -1.0;
            for x in 0 .. self.width {
                let fg = self.tiles[((y * self.width) + x) as usize].fg;
                let bg = self.tiles[((y * self.width) + x) as usize].bg;
                let glyph = self.tiles[((y * self.width) + x) as usize].glyph;
                let glyph_x = glyph % 16;
                let glyph_y = 16 - (glyph / 16);

                let glyph_left = glyph_x as f32 * glyph_size_x;
                let glyph_right = (glyph_x+1) as f32 * glyph_size_x;
                let glyph_top = glyph_y as f32 * glyph_size_y;
                let glyph_bottom = (glyph_y-1) as f32 * glyph_size_y;

                self.push_point(screen_x + step_x, screen_y + step_y, fg, bg, glyph_right, glyph_top);
                self.push_point(screen_x + step_x, screen_y, fg, bg, glyph_right, glyph_bottom);
                self.push_point(screen_x, screen_y, fg, bg, glyph_left, glyph_bottom);
                self.push_point(screen_x, screen_y + step_y, fg, bg, glyph_left, glyph_top);

                self.index_buffer[self.index_counter] = 0 + index_count;
                self.index_buffer[self.index_counter+1] = 1 + index_count;
                self.index_buffer[self.index_counter+2] = 3 + index_count;
                self.index_buffer[self.index_counter+3] = 1 + index_count;
                self.index_buffer[self.index_counter+4] = 2 + index_count;
                self.index_buffer[self.index_counter+5] = 3 + index_count;
                self.index_counter += 6;

                index_count += 4;
                screen_x += step_x;
            }
            screen_y += step_y;
        }
                
        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, self.VBO);
            gl.BufferData(gl::ARRAY_BUFFER,
                        (self.vertex_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &self.vertex_buffer[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
            gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (self.index_buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &self.index_buffer[0] as *const i32 as *const c_void,
                        gl::STATIC_DRAW);
        }
    }
}

impl Console for SimpleConsole {
    fn rebuild_if_dirty(&mut self, gl : &gl::Gles2) {
         if self.is_dirty {
            self.rebuild_vertices(gl);
            self.is_dirty = false;
        }
    }

    fn gl_draw(&mut self, font : &Font, shader : &Shader, gl : &gl::Gles2) {
        unsafe {
            // bind Texture
            font.bind_texture(gl);

            // render container
            shader.useProgram(gl);
            gl.BindVertexArray(self.VAO);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
            gl.BindBuffer(gl::ARRAY_BUFFER, self.VBO);
            gl.DrawElements(gl::TRIANGLES, (self.width * self.height * 6) as i32, gl::UNSIGNED_INT, ptr::null());
        }
        self.is_dirty = false;
    }

    fn at(&self, x:i32, y:i32) -> usize {
        (((self.height-1 - y as u32) * self.width) + x as u32) as usize
    }

    fn cls(&mut self) {
        self.is_dirty = true;
        for tile in self.tiles.iter_mut() {
            tile.glyph = 32;
            tile.fg = RGB::named(color::WHITE);
            tile.bg = RGB::named(color::BLACK);
        }
    }

    fn cls_bg(&mut self, background : RGB) {
        self.is_dirty = true;
        for tile in self.tiles.iter_mut() {
            tile.glyph = 32;
            tile.fg = RGB::named(color::WHITE);
            tile.bg = background;
        }
    }

    fn print(&mut self, x:i32, y:i32, output:&str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);
        let text = output.to_string();

        let bytes = text.as_bytes();
        for i in 0..bytes.len() {
            if idx < self.tiles.len() {
                self.tiles[idx].glyph = bytes[i];
                idx += 1;
            }
        }
    }

    fn print_color(&mut self, x:i32, y:i32, fg:RGB, bg:RGB, output:&str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);
        let text = output.to_string();

        let bytes = text.as_bytes();
        for i in 0..bytes.len() {
            if idx < self.tiles.len() {
                self.tiles[idx].glyph = bytes[i];
                self.tiles[idx].bg = bg;
                self.tiles[idx].fg = fg;
                idx += 1;
            }
        }
    }
}