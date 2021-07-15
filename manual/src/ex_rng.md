## Bracket-Random Examples

### diceroll

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/diceroll.rs)

Demonstrates using the `roll_dice` functionality of the RNG to roll dice in a familiar, RPG-style. `3d6` is represented as `roll_dice(3, 6)`.

```
Rolling 3d6, 10 times.
3d6 Roll: 10
3d6 Roll: 8
3d6 Roll: 11
3d6 Roll: 10
3d6 Roll: 14
3d6 Roll: 7
3d6 Roll: 13
3d6 Roll: 9
3d6 Roll: 13
3d6 Roll: 11
Total of rolls: 106
```

### dicestring

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/dicestring.rs)

Demonstrates using the `roll_str` functionality to allow you to enter dice types as strings. For example, `roll_str("3d6")`. Note that this is a lot slower due to parsing requirements.

```
Rolling 3d6, 10 times.
3d6 Roll: 13
3d6 Roll: 3
3d6 Roll: 11
3d6 Roll: 6
3d6 Roll: 8
3d6 Roll: 12
3d6 Roll: 5
3d6 Roll: 10
3d6 Roll: 12
3d6 Roll: 13
Total of rolls: 93
```

### die_iterator

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/die_iterator.rs)

Creates a dice iterator, each rolling a 6-sided die. Takes 10 dice rolls and prints them.

```
Rolled 3
Rolled 1
Rolled 3
Rolled 6
Rolled 2
Rolled 4
Rolled 1
Rolled 2
Rolled 1
Rolled 1
```

### distribution

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/distribution.rs)

Rolls a lot of dice and plots their relative distribution. This is intended to validate that the random number generator is behaving appropriately.

```
Rolling 3d6, 200000 times and counting distribution.
03 : ##
04 : #######
05 : ###############
06 : #########################
07 : ######################################
08 : #####################################################
09 : ################################################################
10 : ####################################################################
11 : ######################################################################
12 : #################################################################
13 : #####################################################
14 : ######################################
15 : ##########################
16 : ###############
17 : #######
18 : ##
```

### next

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/next.rs)

Demonstrates the `next_u64` function in the RNG, which passes straight through to the underlying RNG code.

```
Generating the next 10 u64 numbers
Roll: 18130881974873676332
Roll: 3148465433356529749
Roll: 44531299326498369
Roll: 9665219486649819621
Roll: 10520437451657888625
Roll: 12316016225926642867
Roll: 2116667603649678054
Roll: 11573604930291377796
Roll: 2541210746452578386
Roll: 17238031251593215327
```

### rand

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/rand.rs)

Demonstrates how to use `rand` to generate a random number of a specified type.

```
Generating the next 10 f64 numbers
Roll: 0.7195672608676137
Roll: 0.5348780904141426
Roll: 0.8706676996760022
Roll: 0.32794462603290664
Roll: 0.619775940285832
Roll: 0.4395722002981868
Roll: 0.37184757519241163
Roll: 0.9221657800105313
Roll: 0.35612926854806837
Roll: 0.17372920791278967
```

### range

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/range.rs)

Demonstrates the `rng.range(min, max)` function.

```
Generating the next 10 numbers in the range 100 - 200
Roll: 181
Roll: 179
Roll: 199
Roll: 180
Roll: 114
Roll: 117
Roll: 166
Roll: 162
Roll: 196
Roll: 113
```

### slice

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/slice.rs)

Demonstrates using the crate to randomly select an entry from a slice (collection) of entries.

```
Randomly chose a: Cat
Randomly chose a: Dragon
Randomly chose a: Hamster
Randomly chose a: Dragon
Randomly chose a: Cat
Randomly chose a: Dragon
Randomly chose a: Cat
Randomly chose a: Hamster
Randomly chose a: Cat
Randomly chose a: Cat
```

### slice_index

[Source Code](https://github.com/amethyst/bracket-lib/blob/master/bracket-random/examples/slice_index.rs)

Demonstrates using `slice_index` to randomly pull an index entry from a slice of data.

```
Randomly chose index: 2, which is a Gerbil
Randomly chose index: 0, which is a Cat
Randomly chose index: 0, which is a Cat
Randomly chose index: 1, which is a Dog
Randomly chose index: 0, which is a Cat
Randomly chose index: 2, which is a Gerbil
Randomly chose index: 4, which is a Dragon
Randomly chose index: 4, which is a Dragon
Randomly chose index: 4, which is a Dragon
Randomly chose index: 1, which is a Dog
```