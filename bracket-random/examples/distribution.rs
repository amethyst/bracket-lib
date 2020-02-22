use bracket_random::prelude::*;

fn main() {
    let mut rng = RandomNumberGenerator::new();
    let mut rolls : Vec<i32> = Vec::new();
    for _ in 0..=18 {
        rolls.push(0);
    }

    const N_ROLLS : i32 = 200000;

    println!("Rolling 3d6, {} times and counting distribution.", N_ROLLS);
    for _ in 0..N_ROLLS {
        let d6roll = rng.roll_str("3d6").expect("Parse fail");
        rolls[d6roll as usize] += 1;
    }

    let max = rolls.iter().max().unwrap();
    let scale = 70.0 / *max as f32;

    for i in 3..=18 {
        //println!("{:02} was rolled {} times.", i, rolls[i]);
        print!("{:02} : ", i);
        for _ in 0.. (rolls[i] as f32 * scale) as i32 {
            print!("#");
        }
        println!("");
    }
}