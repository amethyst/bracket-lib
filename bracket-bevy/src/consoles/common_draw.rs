use super::{ColoredTextSpans, ConsoleFrontEnd, TextAlign};
use crate::{
    prelude::{string_to_cp437, to_cp437},
    BracketContext, CharacterTranslationMode, FontCharType,
};
use bracket_color::prelude::*;

pub(crate) fn draw_box(
    terminal: &mut dyn ConsoleFrontEnd,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGBA,
    bg: RGBA,
) {
    for y in sy..sy + height {
        for x in sx..sx + width {
            terminal.set(x, y, WHITE.into(), BLACK.into(), 32);
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

pub(crate) fn print(terminal: &mut dyn ConsoleFrontEnd, mut x: i32, y: i32, text: &str) {
    let bytes = match terminal.get_translation_mode() {
        CharacterTranslationMode::Codepage437 => string_to_cp437(text),
        CharacterTranslationMode::Unicode => text.chars().map(|c| c as FontCharType).collect(),
    };

    for glyph in bytes {
        terminal.set(x, y, WHITE.into(), BLACK.into(), glyph);
        x += 1;
    }
}

pub(crate) fn print_color(
    terminal: &mut dyn ConsoleFrontEnd,
    mut x: i32,
    y: i32,
    text: &str,
    foreground: RGBA,
    background: RGBA,
) {
    let bytes = string_to_cp437(text);
    for glyph in bytes {
        terminal.set(x, y, foreground, background, glyph);
        x += 1;
    }
}

pub(crate) fn draw_hollow_box(
    terminal: &mut dyn ConsoleFrontEnd,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGBA,
    bg: RGBA,
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
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGBA,
    bg: RGBA,
) {
    for y in sy..sy + height {
        for x in sx..sx + width {
            terminal.set(x, y, WHITE.into(), BLACK.into(), 32);
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
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGBA,
    bg: RGBA,
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

pub(crate) fn printer(
    terminal: &mut dyn ConsoleFrontEnd,
    context: &BracketContext,
    x: i32,
    y: i32,
    output: &str,
    align: TextAlign,
    background: Option<RGBA>,
) {
    let bg = if let Some(bg) = background {
        bg
    } else {
        RGBA::from_u8(0, 0, 0, 255)
    };

    let split_text = ColoredTextSpans::new(context, output);

    let mut tx = match align {
        TextAlign::Left => x,
        TextAlign::Center => x - (split_text.length as i32 / 2),
        TextAlign::Right => x - split_text.length as i32,
    };
    for span in split_text.spans.iter() {
        let fg = span.0;
        for ch in span.1.chars() {
            terminal.set(tx, y, fg, bg, to_cp437(ch));
            tx += 1;
        }
    }
}

/// Draws a horizontal progress bar
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_bar_horizontal(
    console: &mut dyn ConsoleFrontEnd,
    sx: i32,
    sy: i32,
    width: i32,
    n: i32,
    max: i32,
    fg: RGBA,
    bg: RGBA,
) {
    let percent = n as f32 / max as f32;
    let fill_width = (percent * width as f32) as i32;
    for x in 0..width {
        if x <= fill_width {
            console.set(sx + x, sy, fg, bg, to_cp437('▓'));
        } else {
            console.set(sx + x, sy, fg, bg, to_cp437('░'));
        }
    }
}

/// Draws a vertical progress bar
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_bar_vertical(
    console: &mut dyn ConsoleFrontEnd,
    sx: i32,
    sy: i32,
    height: i32,
    n: i32,
    max: i32,
    fg: RGBA,
    bg: RGBA,
) {
    let percent = n as f32 / max as f32;
    let fill_height = height - ((percent * height as f32) as i32);
    for y in 0..height {
        if y >= fill_height {
            console.set(sx, sy + y, fg, bg, to_cp437('▓'));
        } else {
            console.set(sx, sy + y, fg, bg, to_cp437('░'));
        }
    }
}
