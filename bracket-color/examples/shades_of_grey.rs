use bracket_color::prelude::*;
use std::io::{stdout, Write};
use util::print_color;

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

mod util;
