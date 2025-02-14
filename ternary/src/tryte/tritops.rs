use super::*;
use std::ops::{BitAnd, BitOr};

//== TritWise ==//

impl BitAnd for Tryte {
    type Output = Tryte;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut count = 0;
        self.map(|x| {
            count += 1;
            x & rhs[count - 1]
        })
        .into()
    }
}

impl BitOr for Tryte {
    type Output = Tryte;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut count = 0;
        self.map(|x| {
            count += 1;
            x | rhs[count - 1]
        })
        .into()
    }
}
