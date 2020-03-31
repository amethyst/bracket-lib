pub use crate::prelude::VirtualKeyCode;
pub use bracket_geometry::prelude::Point;

/// Available device events
#[derive(Clone, Debug, PartialEq)]
pub enum BEvent {
    /// The window was resized
    Resized {
        new_size: Point,
        dpi_scale_factor: f32,
    },

    /// The window was moved
    Moved { new_position: Point },

    /// The window has requested that it be closed
    CloseRequested,

    /// A character was input
    Character { c: char },

    /// The window gained or lost focus
    Focused { focused: bool },

    /// The mouse cursor entered the window
    CursorEntered,

    /// The mouse cursor left the window
    CursorLeft,

    /// The mouse cursor moved
    CursorMoved { position: Point },

    /// A mouse button was pressed or released
    MouseClick { button: usize, pressed: bool },

    /// Mouse button is down
    MouseButtonDown { button: usize },

    /// Mouse button is up
    MouseButtonUp { button: usize },

    /// A key on the keyboard was pressed or released.
    KeyboardInput {
        key: VirtualKeyCode,
        scan_code: u32,
        pressed: bool,
    },

    /// The window's scale factor was changed. You generally don't need to do anything for this, unless you are working with
    /// pixel coordinates.
    ScaleFactorChanged {
        new_size: Point,
        dpi_scale_factor: f32,
    },
}
