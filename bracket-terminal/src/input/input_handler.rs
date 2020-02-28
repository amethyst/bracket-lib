use crate::prelude::{BTerm, VirtualKeyCode};
use std::collections::{ HashSet, VecDeque };
use bracket_geometry::prelude::Point;
use super::{BEvent, INPUT};

/// Internal: clears the current frame's input state. Used by HAL backends to indicate the start of a new frame
/// for input.
pub(crate) fn clear_input_state(term: &mut BTerm) {
    term.key = None;
    term.left_click = false;
    term.shift = false;
    term.control = false;
    term.alt = false;
    term.web_button = None;
    let mut input = INPUT.lock().unwrap();
    input.keys_down.clear();
    input.scancodes.clear();
    input.mouse_buttons.clear();
}

/// Represents the current input state. The old key/mouse fields remain available for compatibility.
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
    /// Internal - instantiates a new Input object.
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

    /// Checks to see if a key is pressed. True if it is, false otherwise.
    pub fn is_key_pressed(&self, key : VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    /// Checks to see if a key is pressed by scancode. True if it is, false if it isn't.
    pub fn is_scancode_pressed(&self, scan_code : u32) -> bool {
        self.scancodes.contains(&scan_code)
    }

    /// Checks to see if a mouse button is pressed. 0 = left, 1 = right, 2 = middle, etc. is the convention for
    /// button numbering.
    pub fn is_mouse_button_pressed(&self, button_num: usize) -> bool {
        self.mouse_buttons.contains(&button_num)
    }

    /// Returns the current mouse position (0,0 if there isn't one yet), in TILE coordinates for the specified
    /// console layer.
    pub fn mouse_tile_pos(&self, console: usize) -> (i32, i32) {
        if console < self.mouse_tile.len() {
            self.mouse_tile[console]
        } else {
            (0,0)
        }
    }

    /// Returns the current mouse position (0,0 if there isn't one yet), in TILE coordinates for the specified
    /// console layer in Point format.
    pub fn mouse_tile(&self, console: usize) -> Point {
        if console < self.mouse_tile.len() {
            Point::from_tuple(self.mouse_tile[console])
        } else {
            Point::zero()
        }
    }

    /// Return the current mouse position in pixels.
    pub fn mouse_pixel_pos(&self) -> (f64, f64) {
        (self.mouse_pixel.0, self.mouse_pixel.1)
    }

    /// Call this to enable the event queue. Otherwise, events will not be tracked/stored outside of the
    /// HAL setup (to avoid continually filling a buffer that isn't being used).
    pub fn activate_event_queue(&mut self) {
        self.use_events = true;
    }

    /// Pop a single event from the event queue. Returns None if there aren't any events.
    pub fn pop(&mut self) -> Option<BEvent> {
        self.event_queue.pop_back()
    }

    /// Provides a for_each function for all messages in the queue.
    pub fn for_each_message<F>(&mut self, mut action : F) 
        where F : FnMut(BEvent)
    {
        loop {
            let e = self.event_queue.pop_back();
            if let Some(e) = e {
                action(e);
            } else {
                break;
            }
        }
    }

    /// Internal - do not use
    pub(crate) fn on_key_down(&mut self, key : VirtualKeyCode, scan_code : u32) {
        self.keys_down.insert(key);
        self.scancodes.insert(scan_code);
    }

    /// Internal - do not use
    pub(crate) fn on_mouse_button(&mut self, button_num: usize) {
        self.mouse_buttons.insert(button_num);
    }

    /// Internal - do not use
    pub(crate) fn on_mouse_pixel_position(&mut self, x:f64, y:f64) {
        self.mouse_pixel = (x,y);
        self.push_event(BEvent::CursorMoved{position: Point::new(x as i32, y as i32)});
    }

    /// Internal - do not use
    pub(crate) fn on_mouse_tile_position(&mut self, console: usize, x:i32, y:i32) {
        while self.mouse_tile.len() < console+1 {
            self.mouse_tile.push((0,0));
        }
        self.mouse_tile[console] = (x, y);
    }

    /// Internal - do not use
    pub(crate) fn push_event(&mut self, event : BEvent) {
        if self.use_events {
            self.event_queue.push_front(event);
        }
    }
}