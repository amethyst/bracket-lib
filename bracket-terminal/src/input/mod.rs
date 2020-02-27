use crate::prelude::{BTerm, VirtualKeyCode};
use std::collections::HashSet;
use bracket_geometry::prelude::Point;

#[inline]
pub fn clear_input_state(term: &mut BTerm) {
    term.key = None;
    term.left_click = false;
    term.shift = false;
    term.control = false;
    term.alt = false;
    term.web_button = None;
    term.input.keys_down.clear();
    term.input.scancodes.clear();
    term.input.mouse_buttons.clear();
}

#[derive(Clone, Debug)]
pub struct Input {
    keys_down : HashSet<VirtualKeyCode>,
    scancodes : HashSet<u32>,
    mouse_buttons : HashSet<usize>,
    mouse_pixel : (f64, f64)
}

impl Input {
    pub(crate) fn new() -> Self {
        Self{
            keys_down : HashSet::new(),
            scancodes : HashSet::new(),
            mouse_buttons : HashSet::new(),
            mouse_pixel : (0.0, 0.0)
        }
    }

    pub fn is_key_pressed(&self, key : VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_scancode_pressed(&self, scan_code : u32) -> bool {
        self.scancodes.contains(&scan_code)
    }

    pub fn is_mouse_button_pressed(&self, button_num: usize) -> bool {
        self.mouse_buttons.contains(&button_num)
    }

    pub fn mouse_tile_pos(&self, console: usize) -> (i32, i32) {
        (0,0)
    }

    pub fn mouse_tile(&self, console: usize) -> Point {
        Point::new(0,0)
    }

    // NOTE: Do these really need to be 64-bit? That's slow on some platforms, and it's unlikely
    // that you have more than 3.4E+38 pixels.
    pub fn mouse_pixel_pos(&self) -> (f64, f64) {
        (self.mouse_pixel.0, self.mouse_pixel.1)
    }

    pub(crate) fn on_key_down(&mut self, key : VirtualKeyCode, scan_code : u32) {
        self.keys_down.insert(key);
        self.scancodes.insert(scan_code);
    }

    pub(crate) fn on_mouse_button(&mut self, button_num: usize) {
        self.mouse_buttons.insert(button_num);
    }

    pub(crate) fn on_mouse_pixel_position(&mut self, x:f64, y:f64) {
        self.mouse_pixel = (x,y);
    }
}