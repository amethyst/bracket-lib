use bracket_random::prelude::*;

fn main() {
    let mut rng = RandomNumberGenerator::new();
    println!("Generating the next 10 numbers in the range 100 - 200");
    for _ in 0..10 {
        println!("Roll: {}", rng.range(100, 200));
    }
}