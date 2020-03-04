use crate::prelude::{INPUT, BEvent};

/// Global variable to store mouse position changes
pub static mut GLOBAL_MOUSE_POS: (i32, i32) = (0, 0);

/// Event called via the web interface to indicate mouse movement
pub fn on_mouse_move(mouse: web_sys::MouseEvent) {
    let off_x = mouse.offset_x();
    let off_y = mouse.offset_y();
    INPUT.lock().unwrap().on_mouse_pixel_position(off_x as f64, off_y as f64);
    unsafe {
        GLOBAL_MOUSE_POS = (off_x, off_y);
    }
}

/// Global variable to indicate mouse clicking
pub static mut GLOBAL_LEFT_CLICK: bool = false;

/// Event called via the web interface to indicate mouse clicking
pub fn on_mouse_down(mouse: web_sys::MouseEvent) {
    INPUT.lock().unwrap().on_mouse_button_down(mouse.buttons() as usize);
    unsafe {
        GLOBAL_LEFT_CLICK = true;
    }
}
