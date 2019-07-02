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

## Examples

* [Example 1 - Hello World](examples/ex01-helloworld) is a small example, showing off a simple 8x8 console, and the boilerplate required to make RLTK run.
