// Dummy platform to let it compile and do nothing. Only useful if you don't want a graphical backend.
use crate::{GameState, Rltk};

mod keycodes;
pub use keycodes::VirtualKeyCode;

use pancurses::{initscr, Window, noecho, resize_term};

mod main_loop;
pub use main_loop::main_loop;

pub struct PlatformGL {
    window : Window
}

pub mod shader {
    pub struct Shader{}
}

pub mod font {
    pub struct Font{
        pub tile_size: (u32, u32)
    }

    impl Font {
        pub fn load<S: ToString>(_filename: S, _tile_size: (u32, u32)) -> Font {
            Font{
                tile_size : (0, 0)
            }
        }

        pub fn setup_gl_texture(&mut self, _gl: &crate::hal::RltkPlatform) {

        }

        pub fn bind_texture(&self, _gl: &crate::hal::RltkPlatform) {

        }
    }
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
) -> crate::Rltk 
{
    let window = initscr();
    //println!("{}, {}", width_pixels, height_pixels);
    resize_term(height_pixels as i32/8, width_pixels as i32/8);
    noecho();
    //timeout(0);
    window.nodelay(true);

    crate::Rltk {
        backend: super::RltkPlatform { 
            platform: PlatformGL{
                window
            } 
        },
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders : Vec::new(),
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    }
}

pub struct SimpleConsoleBackend {
    tiles : Vec<crate::Tile>
}

impl SimpleConsoleBackend {
    pub fn new(_gl: &super::RltkPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend{
            tiles : Vec::new()
        }
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::RltkPlatform,
        _height: u32,
        _width: u32,
        tiles: &[crate::Tile],
        _offset_x: f32,
        _offset_y: f32,
    ) {
        self.tiles.clear();
        for t in tiles.iter() {
            self.tiles.push(*t);
        }
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        platform: &super::RltkPlatform,
        width: u32,
        height: u32,
    ) {
        let window = &platform.platform.window;
        let mut idx = 0;
        for y in 0..height {
            for x in 0..width {                
                window.mvaddch(height as i32 - y as i32, x as i32, self.tiles[idx].glyph as char);
                idx += 1;
            }            
        }
    }
}

pub struct SparseConsoleBackend {
}

impl SparseConsoleBackend {
    pub fn new(_gl: &super::RltkPlatform, _width: usize, _height: usize) -> SparseConsoleBackend {
        SparseConsoleBackend{}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::RltkPlatform,
        _height: u32,
        _width: u32,
        _offset_x: f32,
        _offset_y: f32,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        platform: &super::RltkPlatform,
        tiles: &[crate::sparse_console::SparseTile],
    ) {
        let window = &platform.platform.window;
        for t in tiles.iter() {
            // Do something
        }
    }
}