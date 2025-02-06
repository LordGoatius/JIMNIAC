use super::*;
use std::ops::{Neg, Not, Shl, Shr};

//== Unary Ops ==//

impl Neg for Tryte {
    type Output = Tryte;
    fn neg(self) -> Self::Output {
        Tryte::new()
            .with_one(self.one().neg())
            .with_two(self.two().neg())
            .with_three(self.three().neg())
            .with_four(self.four().neg())
    }
}

impl Not for Tryte {
    type Output = Tryte;
    fn not(self) -> Self::Output {
        Tryte::new()
            .with_one(self.one().not())
            .with_two(self.two().not())
            .with_three(self.three().not())
            .with_four(self.four().not())
    }
}

impl Shl<usize> for Tryte {
    type Output = Tryte;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut copy = self;
        for _ in 0..rhs {
            copy = Tryte::new()
                .with_two(copy.one())
                .with_three(copy.two())
                .with_four(copy.three())
        }
        copy
    }
}

impl Shr<usize> for Tryte {
    type Output = Tryte;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut copy = self;
        for _ in 0..rhs {
            copy = Tryte::new()
                .with_one(copy.two())
                .with_two(copy.three())
                .with_three(copy.four())
        }
        copy
    }
}

#[cfg(test)]
pub mod test {}
