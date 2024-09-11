use super::*;
use std::ops::{Neg, Not, Shl, Shr};

//== Unary Ops ==//

impl Neg for Tryte {
    type Output = Tryte;
    fn neg(self) -> Self::Output {
        Tryte(self.map(|x| -x))
    }
}

impl Not for Tryte {
    type Output = Tryte;
    fn not(self) -> Self::Output {
        Tryte(self.map(|x| !x))
    }
}

impl Shl<usize> for Tryte {
    type Output = Tryte;

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

impl Shr<usize> for Tryte {
    type Output = Tryte;

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

#[cfg(test)]
pub mod test {
    use super::{Trit, Tryte};

    #[test]
    fn test_shift() {
        let trit_left: Tryte = [Trit::POne; 9].into();
        let mut trit_left_check: Tryte = [Trit::POne; 9].into();
        trit_left_check[0] = Trit::Zero;
        assert_eq!(trit_left << 1usize, trit_left_check);
        trit_left_check[1] = Trit::Zero;
        assert_eq!(trit_left << 2usize, trit_left_check);

        let trit_right: Tryte = [Trit::POne; 9].into();
        let mut trit_right_check: Tryte = [Trit::POne; 9].into();
        trit_right_check[8] = Trit::Zero;
        assert_eq!(trit_right >> 1usize, trit_right_check);
        trit_right_check[7] = Trit::Zero;
        assert_eq!(trit_right >> 2usize, trit_right_check);
    }
}
