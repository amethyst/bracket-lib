# Welcome to RLTK_RS, the Rust implementation of RLTK

You can find the original C++ project here: [https://github.com/thebracket/rltk](https://github.com/thebracket/rltk).

To use this, you will want to have a working `Rust` and `Cargo` setup. On Windows, [rustup](https://rustup.rs/) should get you going.

I recommend initially cloning this repo in its entirety. It is setup like this:

```
+---examples                            EXAMPLE FILES
|   \---ex01-helloworld                 EXAMPLE 01 - Hello World
|       \---src                         Source code for example 1
+---resources                           Assets used by the engine
+---src                                 Source code for RLTK itself
```

To build and run an example, `cd` to its directory and use `cargo` to build/run there. For example:

```
cd examples/ex01-helloworld
cargo build
cargo run
```

## Using RLTK in your project

In your `Cargo.toml` file, include the following:

```toml
[dependencies]
rltk = { git = "https://github.com/thebracket/rltk_rs" }
```

*Note: we don't do that in the example files, we use a relative path - to avoid having nested git repos.*

Copy all the files from the `resources` directory inside RLTK into your own `resources` folder. RLTK needs to be able to load the font file and OpenGL shaders.

For the simplest possible *Hello World*, your source code (`main.rs`) can look like this:

```rust
extern crate rltk;
use rltk::{Rltk, GameState, Console, RGB};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello RLTK World");
    }
}

fn main() {
    let mut context = Rltk::init_simple8x8(80, 50, "Hello RLTK World", "resources");
    let mut gs = State{ };
    context.main_loop(&mut gs);
}
```

## Examples

### Example 1: Bouncing Hello World

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE01.gif)

[Example 1 - Hello World](examples/ex01-helloworld) is a small example, showing off a simple 8x8 console, and the boilerplate required to make RLTK run.

### Example 2: Multiple console layers

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE02.gif)

[Example 2 - Sparse Layers](examples/ex02-sparse) is very similar to example 1, but it adds an additional layer - in a VGA 8x16 font, and renders the FPS and frame rate to it. This illustrates how easy it is to work with layers in RLTK.
