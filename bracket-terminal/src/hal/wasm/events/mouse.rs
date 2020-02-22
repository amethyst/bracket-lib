/// Global variable to store mouse position changes
pub static mut GLOBAL_MOUSE_POS: (i32, i32) = (0, 0);

/// Event called via the web interface to indicate mouse movement
pub fn on_mouse_move(mouse: web_sys::MouseEvent) {
    unsafe {
        GLOBAL_MOUSE_POS = (mouse.offset_x(), mouse.offset_y());
    }
}

/// Global variable to indicate mouse clicking
pub static mut GLOBAL_LEFT_CLICK: bool = false;

/// Event called via the web interface to indicate mouse clicking
pub fn on_mouse_down(_mouse: web_sys::MouseEvent) {
    unsafe {
        GLOBAL_LEFT_CLICK = true;
    }
}
