use super::*;
use std::ops::{Neg, Not, Shl, Shr};

//== Unary Ops ==//

impl<const N: usize> Neg for Varsize<N> {
    type Output = Varsize<N>;
    fn neg(self) -> Self::Output {
        Varsize(self.map(|x| -x))
    }
}

impl<const N: usize> Not for Varsize<N> {
    type Output = Varsize<N>;
    fn not(self) -> Self::Output {
        Varsize(self.map(|x| !x))
    }
}

impl<const N: usize> Shl<usize> for Varsize<N> {
    type Output = Varsize<N>;

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

impl<const N: usize> Shr<usize> for Varsize<N> {
    type Output = Varsize<N>;

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
