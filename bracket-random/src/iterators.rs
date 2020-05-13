use crate::prelude::RandomNumberGenerator;
use core::iter::Iterator;
use std::convert::TryInto;

pub struct DiceIterator<'a> {
    die_type: i32,
    rng: &'a mut RandomNumberGenerator,
}

impl<'a> DiceIterator<'a> {
    pub fn new<T>(die_type: T, rng: &'a mut RandomNumberGenerator) -> Self
    where
        T: TryInto<i32>,
    {
        let dt = die_type.try_into().ok().unwrap();
        Self { die_type: dt, rng }
    }
}

impl<'a> Iterator for DiceIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        Some(self.rng.roll_dice(1, self.die_type))
    }
}
