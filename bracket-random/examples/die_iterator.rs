use bracket_random::prelude::*;

fn main() {
    let mut rng = RandomNumberGenerator::new();
    DiceIterator::new(6, &mut rng).take(10).for_each(|n| println!("Rolled {}", n));
}