use crate::prelude::{BTerm, VirtualKeyCode};
use std::collections::{ HashSet, VecDeque };
use bracket_geometry::prelude::Point;
use super::BEvent;

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
    mouse_pixel : (f64, f64),
    mouse_tile : Vec<(i32, i32)>,
    pub(crate) use_events : bool,
    event_queue : VecDeque<BEvent>
}

impl Input {
    pub(crate) fn new() -> Self {
        Self{
            keys_down : HashSet::new(),
            scancodes : HashSet::new(),
            mouse_buttons : HashSet::new(),
            mouse_pixel : (0.0, 0.0),
            mouse_tile : Vec::new(),
            event_queue : VecDeque::new(),
            use_events : false // Not enabled by default so that systems not using it don't fill up RAM for no reason
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
        if console < self.mouse_tile.len() {
            self.mouse_tile[console]
        } else {
            (0,0)
        }
    }

    pub fn mouse_tile(&self, console: usize) -> Point {
        if console < self.mouse_tile.len() {
            Point::from_tuple(self.mouse_tile[console])
        } else {
            Point::zero()
        }
    }

    pub fn mouse_pixel_pos(&self) -> (f64, f64) {
        (self.mouse_pixel.0, self.mouse_pixel.1)
    }

    pub fn activate_event_queue(&mut self) {
        self.use_events = true;
    }

    pub fn pop(&mut self) -> Option<BEvent> {
        self.event_queue.pop_back()
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

    pub(crate) fn on_mouse_tile_position(&mut self, console: usize, x:i32, y:i32) {
        while self.mouse_tile.len() < console+1 {
            self.mouse_tile.push((0,0));
        }
        self.mouse_tile[console] = (x, y);
    }

    pub(crate) fn push_event(&mut self, event : BEvent) {
        if self.use_events {
            self.event_queue.push_front(event);
        }
    }
}