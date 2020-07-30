# bracket-noise

[Auburn's FastNoise](https://github.com/Auburns/FastNoise) library is amazing - it's fast, covers all the commonly used types of noise, and has been ported to many systems. This crate ports much of that functionality (more is being ported in each version) to Rust. It is part of the `bracket-lib` family of crates.

## Examples

You may run examples with `cargo run --example <name>`. The examples use `crossterm` for easy terminal output.

* `simplex_fractal` uses Fractal Simplex Noise to make a heightmap, and outputs it to your terminal.
* `perlin_fractal` uses Fractal Perlin Noise to make a heightmap, and outputs it to your terminal.
* `white_noise` outputs a randomized white noise sample to your terminal.
* `value` outputs "value noise" to your terminal. This is a bit like white noise but smoother.
* `value_fractal` outputs "fractal value noise" to your terminal.
* `cellular` provides a dump of cellular noise. Until the functions to look up the base noise layer from another noise generator are implemented, this is of limited utility.
* `simplex_billow_quintic` and `simplex_rigid_hermite` demonstrate some noise tweaking options.
