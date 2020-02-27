use crate::prelude::{BTerm, VirtualKeyCode};
use std::collections::HashSet;

#[inline]
pub fn clear_input_state(term: &mut BTerm) {
    term.key = None;
    term.left_click = false;
    term.shift = false;
    term.control = false;
    term.alt = false;
    term.web_button = None;
    term.input.keys_down.clear();
}

#[derive(Clone, Debug)]
pub struct Input {
    keys_down : HashSet<VirtualKeyCode>
}

impl Input {
    pub(crate) fn new() -> Self {
        Self{
            keys_down : HashSet::new()
        }
    }

    pub fn is_key_pressed(&self, key : VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub(crate) fn on_key_down(&mut self, key : VirtualKeyCode) {
        self.keys_down.insert(key);
    }
}