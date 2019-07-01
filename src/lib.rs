mod color;
mod font;
mod shader;
mod rltk;

pub use self::rltk::Rltk;
pub use self::color::*;

pub trait GameState {
    fn tick(&mut self, ctx : &mut Rltk);
}
