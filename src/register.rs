use std::ops::Add;

use crate::{tryte::{Tryte, TryteAddResult}, word::{Word, WordAddResult}};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Register(Word);

impl Add for Register {
    type Output = WordAddResult;
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Add<Tryte> for Register {
    type Output = TryteAddResult;
    fn add(self, rhs: Tryte) -> Self::Output {
        let tryte = Tryte(self.0[0..9].try_into().expect("Should always succeed"));
        tryte + rhs
    }
}


