use super::*;
use std::ops::{BitAnd, BitOr};

//== TritWise ==//

impl BitAnd for Word {
    type Output = Word;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut count = 0;
        self.map(|x| {
            count += 1;
            x & rhs[count - 1]
        })
        .into()
    }
}

impl BitOr for Word {
    type Output = Word;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut count = 0;
        self.map(|x| {
            count += 1;
            x | rhs[count - 1]
        })
        .into()
    }
}
