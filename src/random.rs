use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

pub struct RandomNumberGenerator {
    rng: XorShiftRng,
}

impl RandomNumberGenerator {
    /// Creates a new RNG from a randomly generated seed
    pub fn new() -> RandomNumberGenerator {
        let rng: XorShiftRng = SeedableRng::from_entropy();
        RandomNumberGenerator { rng }
    }

    /// Creates a new RNG from a specific seed
    pub fn seeded(seed: u64) -> RandomNumberGenerator {
        let rng: XorShiftRng = SeedableRng::seed_from_u64(seed);
        RandomNumberGenerator { rng }
    }

    /// Returns a random value of whatever type you specify
    pub fn rand<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.rng.gen::<T>()
    }

    /// Returns a random value in the specified range, of type specified at the call site.
    /// This is INCLUSIVE of the first parameter, and EXCLUSIVE of the second.
    /// So range(1,6) will give you numbers from 1 to 5.
    pub fn range<T>(&mut self, min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
    {
        self.rng.gen_range(min, max)
    }

    /// Rolls dice, using the classic 3d6 type of format: n is the number of dice, die_type is the size of the dice.
    pub fn roll_dice(&mut self, n: i32, die_type: i32) -> i32 {
        let mut total = 0;
        for _ in 0..n {
            total += self.range(1, die_type + 1);
        }
        total
    }

    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }
}
