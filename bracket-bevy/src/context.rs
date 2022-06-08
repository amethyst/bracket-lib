use crate::{
    consoles::{ConsoleFrontEnd, Rect},
    fonts::FontStore,
};
use bevy::prelude::Color;
use parking_lot::Mutex;

pub struct BracketContext {
    pub(crate) fonts: Vec<FontStore>,
    pub(crate) terminals: Mutex<Vec<Box<dyn ConsoleFrontEnd>>>,
    pub(crate) current_layer: usize,
}

impl BracketContext {
    pub(crate) fn new() -> Self {
        Self {
            fonts: Vec::new(),
            terminals: Mutex::new(Vec::new()),
            current_layer: 0,
        }
    }

    pub fn get_char_size(&self) -> (usize, usize) {
        self.terminals.lock()[self.current_layer].get_char_size()
    }

    pub fn at(&self, x: usize, y: usize) -> usize {
        self.terminals.lock()[self.current_layer].at(x, y)
    }

    pub fn try_at(&self, x: usize, y: usize) -> Option<usize> {
        self.terminals.lock()[self.current_layer].try_at(x, y)
    }

    pub fn get_clipping(&self) -> Option<Rect> {
        self.terminals.lock()[self.current_layer].get_clipping()
    }

    pub fn set_clipping(&self, clipping: Option<Rect>) {
        self.terminals.lock()[self.current_layer].set_clipping(clipping);
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.current_layer = layer;
    }

    pub fn cls(&self) {
        self.terminals.lock()[self.current_layer].cls();
    }

    pub fn cls_bg(&self, color: Color) {
        self.terminals.lock()[self.current_layer].cls_bg(color);
    }

    pub fn set(&mut self, x: usize, y: usize, fg: Color, bg: Color, glyph: u16) {
        self.terminals.lock()[self.current_layer].set(x, y, fg, bg, glyph);
    }

    pub fn print<S: ToString>(&self, x: usize, y: usize, text: S) {
        self.terminals.lock()[self.current_layer].print(x, y, &text.to_string());
    }

    pub fn print_centered(&mut self, y: usize, text: &str) {
        self.terminals.lock()[self.current_layer].print_centered(y, text);
    }

    pub fn print_color<S: ToString>(
        &self,
        x: usize,
        y: usize,
        text: S,
        foreground: Color,
        background: Color,
    ) {
        self.terminals.lock()[self.current_layer].print_color(
            x,
            y,
            &text.to_string(),
            foreground,
            background,
        )
    }

    pub fn draw_box(&self, x: usize, y: usize, width: usize, height: usize, fg: Color, bg: Color) {
        self.terminals.lock()[self.current_layer].draw_box(x, y, width, height, fg, bg);
    }

    pub fn draw_hollow_box(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) {
        self.terminals.lock()[self.current_layer].draw_hollow_box(x, y, width, height, fg, bg);
    }

    pub fn draw_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) {
        self.terminals.lock()[self.current_layer].draw_box_double(x, y, width, height, fg, bg);
    }

    pub fn draw_hollow_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    ) {
        self.terminals.lock()[self.current_layer]
            .draw_hollow_box_double(x, y, width, height, fg, bg);
    }

    pub fn fill_region(&mut self, target: Rect, glyph: u16, fg: Color, bg: Color) {
        self.terminals.lock()[self.current_layer].fill_region(target, glyph, fg, bg);
    }
}
