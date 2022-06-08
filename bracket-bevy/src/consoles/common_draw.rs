use super::ConsoleFrontEnd;
use crate::prelude::{string_to_cp437, to_cp437};
use bevy::prelude::Color;

pub(crate) fn draw_box(
    terminal: &mut dyn ConsoleFrontEnd,
    sx: usize,
    sy: usize,
    width: usize,
    height: usize,
    fg: Color,
    bg: Color,
) {
    for y in sy..sy + height {
        for x in sx..sx + width {
            terminal.set(x, y, Color::WHITE, Color::BLACK, 32);
        }
    }

    terminal.set(sx, sy, fg, bg, to_cp437('┌'));
    terminal.set(sx + width, sy, fg, bg, to_cp437('┐'));
    terminal.set(sx, sy + height, fg, bg, to_cp437('└'));
    terminal.set(sx + width, sy + height, fg, bg, to_cp437('┘'));
    for x in sx + 1..sx + width {
        terminal.set(x, sy, fg, bg, to_cp437('─'));
        terminal.set(x, sy + height, fg, bg, to_cp437('─'));
    }
    for y in sy + 1..sy + height {
        terminal.set(sx, y, fg, bg, to_cp437('│'));
        terminal.set(sx + width, y, fg, bg, to_cp437('│'));
    }
}

pub(crate) fn print(terminal: &mut dyn ConsoleFrontEnd, mut x: usize, y: usize, text: &str) {
    let bytes = string_to_cp437(text);
    for glyph in bytes {
        terminal.set(x, y, Color::WHITE, Color::BLACK, glyph);
        x += 1;
    }
}

pub(crate) fn print_color(
    terminal: &mut dyn ConsoleFrontEnd,
    mut x: usize,
    y: usize,
    text: &str,
    foreground: Color,
    background: Color,
) {
    let bytes = string_to_cp437(text);
    for glyph in bytes {
        terminal.set(x, y, foreground, background, glyph);
        x += 1;
    }
}

pub(crate) fn draw_hollow_box(
    terminal: &mut dyn ConsoleFrontEnd,
    sx: usize,
    sy: usize,
    width: usize,
    height: usize,
    fg: Color,
    bg: Color,
) {
    terminal.set(sx, sy, fg, bg, to_cp437('┌'));
    terminal.set(sx + width, sy, fg, bg, to_cp437('┐'));
    terminal.set(sx, sy + height, fg, bg, to_cp437('└'));
    terminal.set(sx + width, sy + height, fg, bg, to_cp437('┘'));
    for x in sx + 1..sx + width {
        terminal.set(x, sy, fg, bg, to_cp437('─'));
        terminal.set(x, sy + height, fg, bg, to_cp437('─'));
    }
    for y in sy + 1..sy + height {
        terminal.set(sx, y, fg, bg, to_cp437('│'));
        terminal.set(sx + width, y, fg, bg, to_cp437('│'));
    }
}

pub(crate) fn draw_box_double(
    terminal: &mut dyn ConsoleFrontEnd,
    sx: usize,
    sy: usize,
    width: usize,
    height: usize,
    fg: Color,
    bg: Color,
) {
    for y in sy..sy + height {
        for x in sx..sx + width {
            terminal.set(x, y, Color::WHITE, Color::BLACK, 32);
        }
    }

    terminal.set(sx, sy, fg, bg, to_cp437('╔'));
    terminal.set(sx + width, sy, fg, bg, to_cp437('╗'));
    terminal.set(sx, sy + height, fg, bg, to_cp437('╚'));
    terminal.set(sx + width, sy + height, fg, bg, to_cp437('╝'));
    for x in sx + 1..sx + width {
        terminal.set(x, sy, fg, bg, to_cp437('═'));
        terminal.set(x, sy + height, fg, bg, to_cp437('═'));
    }
    for y in sy + 1..sy + height {
        terminal.set(sx, y, fg, bg, to_cp437('║'));
        terminal.set(sx + width, y, fg, bg, to_cp437('║'));
    }
}

/// Draws a box, starting at x/y with the extents width/height using CP437 line characters
pub(crate) fn draw_hollow_box_double(
    terminal: &mut dyn ConsoleFrontEnd,
    sx: usize,
    sy: usize,
    width: usize,
    height: usize,
    fg: Color,
    bg: Color,
) {
    terminal.set(sx, sy, fg, bg, to_cp437('╔'));
    terminal.set(sx + width, sy, fg, bg, to_cp437('╗'));
    terminal.set(sx, sy + height, fg, bg, to_cp437('╚'));
    terminal.set(sx + width, sy + height, fg, bg, to_cp437('╝'));
    for x in sx + 1..sx + width {
        terminal.set(x, sy, fg, bg, to_cp437('═'));
        terminal.set(x, sy + height, fg, bg, to_cp437('═'));
    }
    for y in sy + 1..sy + height {
        terminal.set(sx, y, fg, bg, to_cp437('║'));
        terminal.set(sx + width, y, fg, bg, to_cp437('║'));
    }
}
