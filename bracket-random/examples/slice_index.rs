use bracket_random::prelude::*;

fn main() {
    let options = [ "Cat", "Dog", "Gerbil", "Hamster", "Dragon" ];

    let mut rng = RandomNumberGenerator::new();
    for _ in 0..10 {
        let option = rng.random_slice_index(&options).unwrap();
        println!("Randomly chose index: {}, which is a {}", option, options[option]);
    }
}