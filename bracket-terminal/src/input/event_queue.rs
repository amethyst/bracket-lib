pub use bracket_geometry::prelude::Point;
pub use crate::prelude::VirtualKeyCode;

/// Available device events
#[derive(Clone, Debug, PartialEq)]
pub enum BEvent {
    /// The window was resized
    Resized{new_size: Point},

    /// The window was moved
    Moved{new_position: Point},

    /// The window has requested that it be closed
    CloseRequested,

    /// A character was input
    Character{c : char},

    /// The window gained or lost focus
    Focused{focused : bool},

    /// The mouse cursor entered the window
    CursorEntered,

    /// The mouse cursor left the window
    CursorLeft,

    /// The mouse cursor moved
    CursorMoved{position: Point},

    /// A mouse button was clicked
    MouseClick{button: usize},

    /// Keyboard input was received.
    KeyboardInput{key: VirtualKeyCode, scan_code : u32},

    /// The window's scale factor was changed. You generally don't need to do anything for this, unless you are working with
    /// pixel coordinates.
    ScaleFactorChanged{new_size : Point},
}