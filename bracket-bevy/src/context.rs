use bevy::prelude::Color;

use crate::{fonts::FontStore, consoles::SimpleConsole};

pub struct BracketContext {
    pub(crate) fonts: Vec<FontStore>,
    pub(crate) terminals: Vec<SimpleConsole>,
    pub(crate) current_layer: usize,
}

impl BracketContext {
    pub(crate) fn new() -> Self {
        Self { fonts: Vec::new(), terminals: Vec::new(), current_layer: 0 }
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.current_layer = layer;
    }

    pub fn cls(&mut self) {
        self.terminals[self.current_layer].cls();
    }

    pub fn print<S: ToString>(&mut self, x: usize, y: usize, text: S) {
        self.terminals[self.current_layer].print(x, y, text);
    }

    pub fn print_color<S: ToString>(&mut self, x: usize, y: usize, text: S, foreground: Color) {
        self.terminals[self.current_layer].print_color(x, y, text, foreground)
    }
}