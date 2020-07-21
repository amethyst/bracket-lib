# bracket-random

Part of the `bracket-lib` family, `bracket-random` is focused on providing dice-oriented random numbers. It also (optionally) includes parsing of RPG-style dice strings (e.g. `3d6+12`). It is targeted at games, particularly RPGs. It uses the high-performance `XorShift` algorithm for random number generation.

## Using bracket-random

To obtain `bracket-random`, include the following in your `Cargo.toml` file:

```toml
[dependencies]
bracket-random = "0.7.0"
```

It will be available as a crate very soon.

You can use `bracket-random` by including the `prelude` and instantiating `RandomNumberGenerator`:

```rust
use bracket_random::prelude::*;
let mut rng = RandomNumberGenerator::new();
```

You can also seed the RNG with `RandomNumberGenerator::seeded(1234)`.

## Obtaining Randomness

There are a number of random options available:

* `rng.roll_dice(1, 6)` rolls a six-sided die once.
* `rng.roll_dice(3, 6)` rolls three six-sided die, and adds them up.
* `rng.roll_str("3d6+12")` rolls three six-sided die, adds them up, and adds twelve to the result.
* `rng.next_u64` provides a random `u64`, anywhere within the `u64` range.
* `rng.rand::<TYPE>()` tries to provide a random number of type `TYPE`.
* `rng.range(min, max)` tries to provide a random number within the specified range.
* `rng.slice_index(&slice)` returns an `Option` with `None` if there are no options, or the index of a randomly selected slice entry. This is handy for treasure tables.
* `rng.slice(&slice)` returns an `Option` with `None` for empty slices, or the *contents* of a randomly selected slice entry.

## Parsing RPG-style dice strings

The `bracket-random` library includes a dice string parser. You can try to parse a string as follows:

```rust
use bracket_random::prelude::*;
let dice_type = parse_dice_string("3d6-4");
```

This returns a `Result`, which will either be `Ok` or a parsing error. If unwrapped, it provides a `DiceType` structure, breaking out the details of the requested die roll.

It supports `1d6`, `3d6+1`, and `5d6-1` formats. If you turn off the `parsing` feature flag, this feature is excluded - but your project won't be bloated by regular expression libraries and `lazy_static`.

## Feature Flags

* `parsing` enables parsing of dice types as strings.
* `serde` makes the `DiceType` structure serializable.
* If you are compiling for `wasm32-unknown-unknown`, it automatically includes `wasm-bindgen`.

## Examples

Execute examples with `cargo run --example <name>`.

* `diceroll` rolls 3d6 (specified as `roll_dice(1,6)`) 10 times and prints the results.
* `dicestring` rolls 3d6 (specified as `roll_str("3d6")`) 10 times and prints the results.
* `distribution` rolls 3d6, 200,000 times and plots the distribution of each cumulative result.
* `next` obtains the next 10 `u64` random numbers and prints them.
* `rand` obtains the next 10 `f64` random numbers and prints them. This demonstrates how the `rand` function can take any type that the underlying `random` library considers sufficiently numeric.
* `range` obtains the next 10 random numbers in the range `100.200` and prints them.
* `slice_index` randomly picks a slice index, prints it - and the contents of the array that was sliced.
* `slice` randomly picks a slice entry and prints it.
* `die_iterator` uses the (new and in need of work) `DiceIterator` function to roll 10d6 in a compact manner.
