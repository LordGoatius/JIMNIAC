use super::*;
use std::ops::{BitAnd, BitOr};

//== TritWise ==//

impl<const N: usize> BitAnd for Varsize<N> {
    type Output = Varsize<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut count = 0;
        self.map(|x| {
            count += 1;
            x & rhs[count - 1]
        })
        .into()
    }
}

impl<const N: usize> BitOr for Varsize<N> {
    type Output = Varsize<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut count = 0;
        self.map(|x| {
            count += 1;
            x | rhs[count - 1]
        })
        .into()
    }
}
