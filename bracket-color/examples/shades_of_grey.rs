use bracket_color::prelude::*;
use crossterm::queue;
use crossterm::style::{Color::Rgb, Print, SetForegroundColor};
use std::io::{stdout, Write};

fn print_color(color: RGB, text: &str) {
    queue!(
        stdout(),
        SetForegroundColor(Rgb {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        })
    )
    .expect("Command Fail");
    queue!(stdout(), Print(text)).expect("Command fail");
}
fn main() {
    for color in RgbLerp::new(RGB::named(RED), RGB::named(YELLOW), 20) {
        print_color(color.to_greyscale(), "You've got to lerp it, gray it.\n");
    }
    for color in RgbLerp::new(RGB::named(GREEN), RGB::named(MAGENTA), 20) {
        print_color(
            color.desaturate(),
            "You've got to lerp it, desaturate it and ruin the joke.\n",
        );
    }

    print_color(RGB::named(WHITE), "\nAnd back to white.\n");

    stdout().flush().expect("Flush Fail");
}
