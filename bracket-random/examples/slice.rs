use bracket_random::prelude::*;

fn main() {
    let options = [ "Cat", "Dog", "Gerbil", "Hamster", "Dragon" ];

    let mut rng = RandomNumberGenerator::new();
    for _ in 0..10 {
        let option = rng.random_slice_entry(&options);
        println!("Randomly chose a: {}", option.unwrap());
    }
}