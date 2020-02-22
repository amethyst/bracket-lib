use bracket_random::prelude::*;

fn main() {
    let mut rng = RandomNumberGenerator::new();
    println!("Generating the next 10 u64 numbers");
    for _ in 0..10 {
        println!("Roll: {}", rng.next_u64());
    }
}