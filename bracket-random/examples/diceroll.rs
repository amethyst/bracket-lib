use bracket_random::prelude::*;

fn main() {
    let mut rng = RandomNumberGenerator::new();
    let mut total = 0;
    println!("Rolling 3d6, 10 times.");
    for _ in 0..10 {
        let d6roll = rng.roll_dice(3, 6);
        total += d6roll;
        println!("3d6 Roll: {}", d6roll);
    }
    println!("Total of rolls: {}", total);
}