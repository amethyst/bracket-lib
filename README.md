# Welcome to bracket-lib
Bracket-lib is a versatile Rust library designed to facilitate the creation of roguelike games and other graphical applications. This toolkit is a user-friendly teaching library. It provide virtual consoles on a variety of platforms from the web to major Operating Systems, a number of extensions to allow you to do layering, sprites, and more advanced rendering, and a good "on ramp" for moving onto high-performance libraries once you've mastered the basics.
Read more about the toolkit here: (https://github.com/thebracket/bracket-lib/workflows/Rust/badge.svg)

# Getting started
**Create a new project**: Find the directory in which you want to start developing, and type cargo init my_project to create a new project.

**Linking to Bracket-lib**
The quickest and easiest way to use bracket-lib in your program is to include the entire library in your project. Open your project's Cargo.toml and include:
[dependencies]
bracket-lib = "0.8"
You now have the whole bracket-lib project linked to your program.
Although not recommended, you can link to the Github version. In your project's Cargo.toml file, add the following:
[dependencies]
bracket-lib = { git = "https://github.com/amethyst/bracket-lib.git" }
The main reason to do this is to try out new features. The Github version isn't always as stable as the bracket - lib version, so proceed with caution.

# Example Usage
**Hello Minimal Terminal**
The following code prints "Hello Bracket World" in a new simple console:

```toml
use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let gs: State = State {};
    main_loop(context, gs)
}
```
This provides what you need for a minimal start:
Importing the prelude from bracket_lib makes the various types and functions available.
You have to create a State object. This is where your ongoing game state is stored.
Implementing GameState provides bracket-lib with a tick function to call on every frame.
Your main function constructs a terminal---in this case an 80x50 text console.
You create a State object, even though it doesn't have much state to store.
Launching main_loop hands control over to bracket-lib, and runs the tick function on every rendered frame.

# Resources
**Tutorial Series**: If you are new to rougelike game development, checkout this trtorial series with this library: [https://bfnightly.bracketproductions.com/rustbook/](https://bfnightly.bracketproductions.com/rustbook/)
**Hands-on Rust Book**: Bracket-lib serves as the primary support library for the "Hands-on Rust" book. For a deeper understanding and practical application of Rust programming, consider checking out the book:[Hands-on Rust](https://hands-on-rust.com/).
**Manual**: Early work has begun on crafting a comprehensive manual for Bracket-lib. You can access it in the manual folder of the repository or read it online for detailed guidance. [read it online](https://bfnightly.bracketproductions.com/bracket-lib/what_is_it.html).

**BREAKING CHANGE ALERT**: The `crossterm` feature is now `cross_term` if you are using `bracket-terminal` directly. It's still `crossterm` for `bracket-lib` and `rltk`.

**IMPORTANT**: If you are running the `webgpu` backend, you need to add `resolver = 2` to your `Cargo.toml` file. WGPU requires it for platform selection.

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
bracket-lib = "~0.8"
```

## Feature Flags

There are a few feature flags designed to aide integration with other systems:

* `specs` tells various `bracket-lib` sub-systems to export important primitives as having Specs' `Component` type applied.
* `serde` tells various `bracket-lib` sub-systems to support using `Serde` for serialization/de-serialization.

Performance:

* `threaded` enables multi-threading on some sub-systems.

Terminal mode:

By default, `bracket-lib` runs in OpenGL mode (or WebGL if it detects that you are compiling for `wasm32-unknown-unknown`). If you want to use other rendering back-ends, *disable default features* and apply *one* of the following feature flags:

* `webgpu` to use the `wgpu` system as a back-end, supporting Vulkan, Metal and WebGPU.
* `crossterm` to use the excellent `Crossterm` terminal library.
* `curses` to use `pancurses` for `ncurses` or `pdcurses` support depending upon your platform.

## Sample Projects

- https://github.com/Micutio/innit
- https://github.com/amethyst/shotcaller
- https://github.com/bofh69/rouge
- https://github.com/carsin/miners
- https://github.com/baszalmstra/my-little-robots
- https://github.com/Havegum/Terrain-Generator
- https://github.com/Bobox214/rs-gliphus
- https://github.com/Maxgy/blademaster
- https://github.com/Maxgy/text-rts


# Credit
This is an open source project and I'd like to acknowledge and thank the contributors. This includes me fellow teammates at Amethyst and active contributors such as @thebracket. Thanks all of you for working on this project. 