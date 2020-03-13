use bracket_geometry::prelude::Rect;

#[derive(Copy, Clone, Debug)]
pub struct Sprite {
    pub sheet_location: Rect,
}

impl Sprite {
    pub fn new(sheet_location: Rect) -> Self {
        Sprite { sheet_location }
    }
}
