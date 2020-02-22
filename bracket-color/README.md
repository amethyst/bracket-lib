# bracket-color

This crate provides a color system for use in the `bracket-terminal` system. It is part of the overall `bracket-lib` system.

## Using bracket-color

To obtain `bracket-color`, include the following in your `Cargo.toml` file:

```toml
[dependencies]
bracket-color = "0.7.0"
```

## RGB

The basic structure is `RGB`, which represents a color as red/green/blue components. You can construct a color in a number of ways:

* `new()` provides a black (all zeroes) entry.
* `from_f32` takes three floats, from `0.0` to `1.0` as a starting point.
* `from_u8` takes there bytes, from `0` to `255`.
* `named` takes a tuple of floats, with a LOT of predefined colors available. (e.g. `RGB::named(RED)`).
* `from_hex` takes an HTML/CSS style hex number and converts it (e.g. `RGB::from_hex("#aabbcc"))`).

You can also convert RGB structures:

* Add, subtract, multiply and divide operations are supported both on a single float and against another RGB structure.
* `to_hsv` makes a Hue-Saturation-Value color.
* `to_greyscale` uses a standard grayscale operation to make a greyscale approximation of a color.
* `desaturate` makes a better greyscale conversion by converting to HSV and lowering the saturation.
* `lerp` lets you smoothly transition between two colors, in RGB space.

## HSV

The HSV system provides color support in the HSV space. You can construct an HSV color as follows:

* `new()` makes an all-zero HSV color.
* `from_f32` lets you specify HSV as floats.
* `RGB::to_hsv` converts an RGB color into an HSV color.

You can also go back to RGB with `to_rgb`.

## ColorPair

A `ColorPair` is simply a helper structure holding both a foreground and a background.

## Exports

Everything is exported via the `bracket_color::prelude` namespace.

## Feature Flags

* If you enable the `serde` feature flag, the RGB, HSV and ColorPair structures are derived as `Serde` serializable/de-serializable.
* The `rex` feature flag enables [RexPaint](https://www.gridsagegames.com/rexpaint/) support.

## Examples

There are a few examples to help get you going. They use `crossterm` for terminal output. You may run the examples with `cargo run --example <name>`.

* `lerp` is a simple color lerp.
* `lerpit` is an iterator-based lerp.
* `lerpit_hsv` is an HSV lerp.
* `named_colors` demonstrates how to access named colors.
* `shades_of_grey` demonstrates greyscale and desaturate functions.
* `colors` demonstrates various ways to acquire colors.
