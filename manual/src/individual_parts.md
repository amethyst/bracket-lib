# Using Individual Bracket-Lib Components

`bracket-lib` is a collection of libraries. You can depend upon them individually, if you only want part of the `bracket-lib` functionality. The parts are:

* `bracket-algorithm-traits` defines some traits that are used in other parts of the program.
* `bracket-color` defines how the library handles color, and includes functions for grayscale, RGB/RGBA/HSV conversion, a lot of named colors, and general color management support.
* `bracket-geometry` provides points, lines, rectangles and circle support.
* `bracket-noise` provides Perlin, Simplex, White and other noise functions useful for randomly generating things.
* `bracket-pathfinding` provides an A-Star and a Dijkstra mapping solution.
* `bracket-random` provides an easy-to-use wrapper to a random number generator.
* `bracket-terminal` provides console rendering and support.

When you link directly to a dependency, the namespace is on longer `bracket_lib::prelude`. Instead, it will be the crate's name, e.g. `bracket_random::prelude`.

This option is partly provided to help keep development efforts separated, and partly because sometimes you just want a small portion of what the library has to offer---and there's no point in wasting space (and mostly compile time) on the bits you don't need.