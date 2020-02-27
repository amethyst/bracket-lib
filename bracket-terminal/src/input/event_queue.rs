pub use bracket_geometry::prelude::Point;

#[derive(Clone, Debug, PartialEq)]
pub enum BEvent {
    Resized{new_size: Point},
    Moved{new_position: Point},
    CloseRequested,
    Character{c : char},
    Focused{focused : bool},
    CursorEntered,
    CursorLeft
}