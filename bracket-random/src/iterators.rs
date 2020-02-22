use crate::prelude::RandomNumberGenerator;
use std::convert::TryInto;
use core::iter::Iterator;

pub struct DiceIterator {
    die_type : i32,
    rng : *mut RandomNumberGenerator
}

impl DiceIterator {
    pub fn new<T>(die_type : T, rng : &mut RandomNumberGenerator) -> Self
    where T: TryInto<i32>
    {
        let dt = die_type.try_into().ok().unwrap();
        Self {
            die_type : dt,
            rng
        }
    }
}

impl Iterator for DiceIterator {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        unsafe {
            Some(self.rng.as_mut().unwrap().roll_dice(1, self.die_type))
        }
    }
}