use super::*;
use std::ops::{Neg, Not, Shl, Shr};

//== Unary Ops ==//

impl Neg for Word {
    type Output = Word;
    fn neg(self) -> Self::Output {
        Word(self.map(|x| -x))
    }
}

impl Not for Word {
    type Output = Word;
    fn not(self) -> Self::Output {
        Word(self.map(|x| !x))
    }
}

impl Shl<usize> for Word {
    type Output = Word;

    fn shl(mut self, mut rhs: usize) -> Self::Output {
        while rhs != 0 {
            let copy = self;
            for i in 0..(self.len()-1) {
                self[i+1] = copy[i];
            }
            rhs -= 1;
            self[0] = Trit::Zero;
        }
        self
    }
}

impl Shr<usize> for Word {
    type Output = Word;

    fn shr(mut self, mut rhs: usize) -> Self::Output {
        while rhs != 0 {
            for i in 0..(self.len()-1) {
                self[i] = self[i+1];
            }
            rhs -= 1;
            let len = self.len();
            self[len - 1] = Trit::Zero;
        }
        let len = self.len();
        self[len - 1] = Trit::Zero;
        self
    }
}
