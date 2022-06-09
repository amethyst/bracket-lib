use bracket_random::prelude::RandomNumberGenerator;
use parking_lot::Mutex;

pub struct RandomNumbers {
    rng: Mutex<RandomNumberGenerator>,
}

impl RandomNumbers {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(RandomNumberGenerator::new()),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: Mutex::new(RandomNumberGenerator::seeded(seed)),
        }
    }

    /// Returns a random value of whatever type you specify
    pub fn rand<T>(&self) -> T
    where
        bracket_random::rand::distributions::Standard:
            bracket_random::rand::distributions::Distribution<T>,
    {
        self.rng.lock().rand::<T>()
    }

    /// Returns a random value in the specified range, of type specified at the call site.
    /// This is INCLUSIVE of the first parameter, and EXCLUSIVE of the second.
    /// So range(1,6) will give you numbers from 1 to 5.
    pub fn range<T>(&self, min: T, max: T) -> T
    where
        T: bracket_random::rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.lock().range(min, max)
    }

    /// Rolls dice, using the classic 3d6 type of format: n is the number of dice, die_type is the size of the dice.
    pub fn roll_dice(&self, n: i32, die_type: i32) -> i32 {
        self.rng.lock().roll_dice(n, die_type)
    }

    /// Returns the RNG's next unsigned-64 type
    pub fn next_u64(&self) -> u64 {
        self.rng.lock().next_u64()
    }

    /// Rolls dice based on a DiceType struct, including application of the bonus
    #[cfg(feature = "parsing")]
    pub fn roll(&self, dice: DiceType) -> i32 {
        self.rng.lock().roll_dice(dice)
    }

    /// Rolls dice based on passing in a string, such as roll_str("1d12")
    #[cfg(feature = "parsing")]
    pub fn roll_str<S: ToString>(&self, dice: S) -> Result<i32, DiceParseError> {
        self.rng.lock().roll_str(dice)
    }

    /// Returns a random index into a slice
    pub fn random_slice_index<T>(&self, slice: &[T]) -> Option<usize> {
        self.rng.lock().random_slice_index(slice)
    }

    /// Returns a random entry in a slice (or none if empty)
    pub fn random_slice_entry<'a, T>(&self, slice: &'a [T]) -> Option<&'a T> {
        self.rng.lock().random_slice_entry(slice)
    }
}

impl Default for RandomNumbers {
    fn default() -> Self {
        Self::new()
    }
}
