use bracket_noise::prelude::*;
use bracket_color::prelude::*;
use bracket_random::prelude::*;

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
    let mut rng = RandomNumberGenerator::new();
    let mut noise = FastNoise::seeded(rng.next_u64());
    noise.set_noise_type(NoiseType::Cellular);
    noise.set_frequency(0.25);
    noise.set_fractal_gain(0.6);
    noise.set_cellular_distance_function(CellularDistanceFunction::Natural);
    noise.set_cellular_return_type(CellularReturnType::CellValue);

    for y in 0..50 {
        for x in 0..80 {
            let n = noise.get_noise(x as f32, y as f32);
            let col = (n + 1.0) * 0.5;
            print_color(RGB::from_f32(col, col, col), "▒");
        }
        print_color(RGB::named(WHITE), "\n");
    }

    print_color(RGB::named(WHITE), "\n");
    stdout().flush().expect("Flush Fail");
}