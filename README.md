# Welcome to RLTK_RS, the Rust implementation of RLTK

You can find the original C++ project here: [https://github.com/thebracket/rltk](https://github.com/thebracket/rltk).

To use this, you will want to have a working `Rust` and `Cargo` setup. On Windows, [rustup](https://rustup.rs/) should get you going.

If you'd like to see a functional roguelike that uses `rltk_rs`, check out [Rusty Roguelike](https://github.com/thebracket/rustyroguelike). It was built for the 2019 */r/roguelikedev does the complete roguelike tutorial* event, and implements the [TCOD Tutorial](http://rogueliketutorials.com/tutorials/tcod/) - but using RLTK. It started out as the home of this project, which was then spun off into a separate crate.

## Running the examples

The examples use Cargo's built-in support for example code. E.g. To run example 1, enter: `cargo run --example ex01-helloworld`.

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

[Example 1 - Hello World](examples/ex01-helloworld.rs) is a small example, showing off a simple 8x8 console, and the boilerplate required to make RLTK run.

Run this example with `cargo run --example ex01-helloworld` from the root of the cloned repository.

### Example 2: Multiple console layers

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE02.gif)

[Example 2 - Sparse Layers](examples/ex02-sparse.rs) is very similar to example 1, but it adds an additional layer - in a VGA 8x16 font, and renders the FPS and frame rate to it. This illustrates how easy it is to work with layers in RLTK.

Run this example with `cargo run --example ex02-sparse` from the root of the cloned repository.

### Example 3: Walking around

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE03.gif)

[Example 3 - Walking Around](examples/ex03-walking.rs) is the first step for a lot of roguelikes: we generate a random map (very random in this case), render the player as an `@`, and move him/her/it around with the cursor keys or numpad. This illustrates the simple keyboard input mechanism, and also how to handle basic game state.

Run this example with `cargo run --example ex03-walking` from the root of the cloned repository.

### Example 4: Field of view

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE04.gif)

[Example 4 - Field of View/FOV](examples/ex04-fov.rs) takes example 3, and adds field-of-view. To do this, it implements some traits from the RLTK library that allow it to provide helpers such as this.

Run this example with `cargo run --example ex04-fov` from the root of the cloned repository.

### Example 5: Auto-explore with Dijkstra Flow Maps

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE05-2.gif)

[Example 5 - Auto-explore with Dijkstra Flow Maps](examples/ex05-dijkstra.rs) creates a random map, with a lot more walls. It uses RLTK's Dijkstra Flow Maps (see [this article](http://www.roguebasin.com/index.php?title=The_Incredible_Power_of_Dijkstra_Maps)) to solve an auto-explore problem for the map. I recommend compiling this one with `cargo run --release` - debug mode lacks a lot of optimizations and runs really slowly. (RLTK's Dijkstra implementation automatically uses a parallel algorithm for large numbers of targets).

Run this example with `cargo run --example ex05-dijkstra --release` from the root of the cloned repository. (The `--release` tells it to optimize the build; it's pretty slow without optimizations)

### Example 6: A-Star pathing and mouse control

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE06.gif)

[Example 6 - A Star with the Mouse](examples/ex06-astar-mouse.rs) lets you use A-Star navigation to traverse a random map. Mouse over a destination, and your path is highlighted. Click, and the little @ runs there.

Run this example with `cargo run --example ex06-astar-mouse` from the root of the cloned repository.

### Example 7: Layered Tiles

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE07.gif)

[Example 7 - Tiles](examples/ex07-tiles.rs) shows how you can use the font system for tiles. It creates two layers with the same font; the base layer is used for the map, and the second layer is drawn in "no background" mode to seamlessly overlay sprites atop the map without ugly borders. It uses the FOV code from example 4 to show how to recolor sprites, and uses a distance function to give a lighting effect.

Run this example with `cargo run --example ex07-tiles` from the root of the cloned repository.

### Example 8: REX Paint Support

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE08.jpg)

[Example 8 - REX Paint Support](examples/ex08-rex.rs) demonstrates loading a `.xp` file (created with
GridSageGames' excellent [REX Paint](https://www.gridsagegames.com/rexpaint/)), and displaying it on the console. Thanks to Mara "Alkahest"'s hard work (see original repo [here](https://gitlab.com/medusacle/rs-rexpaint)), you can load and save `.xp` files as you need them.

Run this example with `cargo run --example ex08-rex` from the root of the cloned repository.

### Example 9: Layer Offsets

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE09.jpg)

[Example 9 - Offsets](examples/ex09-offsets.rs) demonstrates the ability to call `set_offset` on a console layer, to nudge it in a direction. In this case, it is used to render thin walls *between* the major tiles.

Run this example with `cargo run --example ex09-offsets` from the root of the cloned repository.

### Example 10: Post Process Effects

![Animated GIF](/screenshots/RLTK_RS_EXAMPLE10.gif)

[Example 10 - Post Process](examples/ex09-postprocess.rs) shows off the post-processing system built into RLTK. Scanlines, and optional screen-burn can be enabled for the truly retro experience.

Run this example with `cargo run --example ex10-postprocess` from the root of the cloned repository.
