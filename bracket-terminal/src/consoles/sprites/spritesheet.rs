use crate::prelude::{Font, Sprite};
use bracket_geometry::prelude::Rect;
use std::rc::Rc;

#[derive(Clone)]
pub struct SpriteSheet {
    pub filename: String,
    pub sprites: Vec<Sprite>,
    pub backing: Option<Rc<Box<Font>>>,
}

impl SpriteSheet {
    pub fn new<S: ToString>(filename: S) -> Self {
        Self {
            filename: filename.to_string(),
            sprites: Vec::new(),
            backing: None,
        }
    }

    pub fn add_sprite(mut self, location_pixel: Rect) -> Self {
        self.sprites.push(Sprite::new(location_pixel));
        self
    }
}
