pub use bracket_geometry::prelude::Point;
pub use crate::prelude::VirtualKeyCode;

#[derive(Clone, Debug, PartialEq)]
pub enum BEvent {
    Resized{new_size: Point},
    Moved{new_position: Point},
    CloseRequested,
    Character{c : char},
    Focused{focused : bool},
    CursorEntered,
    CursorLeft,
    CursorMoved{position: Point},
    MouseClick{button: usize},
    KeyboardInput{key: VirtualKeyCode, scan_code : u32},
    ScaleFactorChanged{new_size : Point},
}