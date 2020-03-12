use crate::prelude::Sprite;
use bracket_geometry::prelude::Rect;

#[derive(Clone, Debug)]
pub struct SpriteSheet {
    pub filename : String,
    pub sprites : Vec<Sprite>,
    pub loaded : bool
}

impl SpriteSheet {
    pub fn new<S: ToString>(filename : S) -> Self {
        Self{
            filename: filename.to_string(),
            sprites: Vec::new(),
            loaded: false
        }
    }

    pub fn add_sprite(mut self, location_pixel : Rect) -> Self {
        self.sprites.push( Sprite::new(location_pixel) );
        self
    }
}