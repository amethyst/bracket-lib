use bevy::prelude::Color;
use crate::{fonts::FontStore, consoles::{ConsoleFrontEnd}};
use parking_lot::Mutex;

pub struct BracketContext {
    pub(crate) fonts: Vec<FontStore>,
    pub(crate) terminals: Mutex<Vec<Box<dyn ConsoleFrontEnd>>>,
    pub(crate) current_layer: usize,
}

impl BracketContext {
    pub(crate) fn new() -> Self {
        Self { fonts: Vec::new(), terminals: Mutex::new(Vec::new()), current_layer: 0 }
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.current_layer = layer;
    }

    pub fn cls(&self) {
        self.terminals.lock()[self.current_layer].cls();
    }

    pub fn print<S: ToString>(&self, x: usize, y: usize, text: S) {
        self.terminals.lock()[self.current_layer].print(x, y, &text.to_string());
    }

    pub fn print_color<S: ToString>(&self, x: usize, y: usize, text: S, foreground: Color, background: Color) {
        self.terminals.lock()[self.current_layer].print_color(x, y, &text.to_string(), foreground, background)
    }
}