use bracket_color::prelude::*;
use std::io::{stdout, Write};
use util::print_color;

fn main() {
    for color in HsvLerp::new(
        RGB::named(GREEN).to_hsv(),
        RGB::named(CHOCOLATE).to_hsv(),
        20,
    ) {
        print_color(color.to_rgb(), "You've got to lerp it, lerp it.\n");
    }

    print_color(RGB::named(WHITE), "\nAnd back to white.\n");

    stdout().flush().expect("Flush Fail");
}

mod util;
