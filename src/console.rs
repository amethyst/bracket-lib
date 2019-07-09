use super::{RGB, Font, Shader};
use super::gl;

pub struct Tile {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB
}

pub trait Console {
    fn rebuild_if_dirty(&mut self, gl : &gl::Gles2);
    fn gl_draw(&mut self, font : &Font, shader : &Shader, gl : &gl::Gles2);

    fn at(&self, x:i32, y:i32) -> usize;
    fn cls(&mut self);
    fn cls_bg(&mut self, background : RGB);
    fn print(&mut self, x:i32, y:i32, output:&str);
    fn print_color(&mut self, x:i32, y:i32, fg:RGB, bg:RGB, output:&str);
}