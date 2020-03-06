# Welcome to bracket-lib

![](https://github.com/thebracket/rltk_rs/workflows/Rust/badge.svg)

> You can read a tutorial series on writing a Roguelike with this library at: [https://bfnightly.bracketproductions.com/rustbook/](https://bfnightly.bracketproductions.com/rustbook/)

## What happened to RLTK?

This *is* RLTK, renamed because it is increasingly finding usage outside of just Roguelikes. It's also been divided into a number of crates, to make it easy to pick-and-choose the features you need.

* `rltk` crate wraps `bracket-lib` and re-exports in the `rltk::` and `rltk::prelude` namespace. This preserves compatibility with all existing RLTK projects.
* `bracket-algorithm-traits` exposes the traits required for the various algorithm systems in other crates.
* `bracket-color` is my RGB/HSV color management system.
* `bracket-geometry` exposes various geometric primitives and helpers. Supports other crates.
* `bracket-noise` is a port of [Auburn's FastNoise](https://github.com/Auburns/FastNoise) to Rust.
* `bracket-pathfinding` provides a high-performance A* (A-Star) pathing system, as well as Dijkstra maps.
* `bracket-random` is a dice-oriented random number generator, including parsing of RPG-style dice strings such as `3d6+12`.

## Using `bracket-lib`

In your `Cargo.toml` file, include:

```toml
[dependencies]
bracket-lib = "0.7"
```

## Feature Flags

There are a few feature flags designed to aide integration with other systems:

* `specs` tells various `bracket-lib` sub-systems to export important primitives as having Specs' `Component` type applied.
* `serde` tells various `bracket-lib` sub-systems to support using `Serde` for serialization/de-serialization.

Performance:

* `threaded` enables multi-threading on some sub-systems.

Terminal mode:

By default, `bracket-lib` runs in OpenGL mode (or WebGL if it detects that you are compiling for `wasm32-unknown-unknown`). If you want to use other rendering back-ends, *disable default features* and apply *one* of the following feature flags:

* `amethyst_engine_vulkan` for Amethyst support, in `Vulkan` mode.
* `amethyst_engine_metal` for Amethyst support, in `Metal` mode.
* `crossterm` to use the excellent `Crossterm` terminal library.
* `curses` to use `pancurses` for `ncurses` or `pdcurses` support depending upon your platform.
