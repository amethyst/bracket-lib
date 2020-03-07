use bracket_color::prelude::*;
use std::io::{stdout, Write};
use util::print_color;

fn main() {
    for color in RgbLerp::new(RGB::named(RED), RGB::named(YELLOW), 20) {
        print_color(color, "You've got to lerp it, lerp it.\n");
    }

    print_color(RGB::named(WHITE), "\nAnd back to white.\n");

    stdout().flush().expect("Flush Fail");
}

mod util;
