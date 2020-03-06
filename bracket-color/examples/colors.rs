use bracket_color::prelude::*;
use std::io::{stdout, Write};
use util::print_color;

fn main() {
    print_color(RGB::named(RED), "Hello RED\n");
    print_color(RGB::from_f32(0.0, 1.0, 0.0), "Hello GREEN\n");
    print_color(RGB::from_u8(0, 0, 255), "Hello BLUE\n");
    print_color(
        RGB::from_hex("#FF00FF").expect("Bad hex!"),
        "Hello MAGENTA\n",
    );
    print_color(RGB::named(WHITE), "And back to white.\n");

    stdout().flush().expect("Flush Fail");
}

mod util;
