use std::ops::{Deref, DerefMut};

use crate::trits::*;

pub mod binops;
pub mod unops;
pub mod tritops;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Tryte(pub(crate) [Trit; 9]);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TryteAddResult {
    carry: Trit,
    result: Tryte,
}

//== Helper Traits ==//

impl Deref for Tryte {
    type Target = [Trit; 9];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tryte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[Trit; 9]> for Tryte {
    fn from(value: [Trit; 9]) -> Self {
        Tryte(value)
    }
}

impl From<Tryte> for isize {
    fn from(value: Tryte) -> Self {
        value
            .iter()
            .enumerate()
            .map(|(i, trit)| match trit {
                Trit::NOne => isize::pow(3, i as u32) * -1,
                Trit::Zero => 0,
                Trit::POne => isize::pow(3, i as u32) * 1,
            })
            .sum()
    }
}

impl PartialOrd for Tryte {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_isize:  isize = (*self).into();

        let other_isize: isize = (*other).into();

        return self_isize.partial_cmp(&other_isize);
    }
}

impl Ord for Tryte {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_isize:  isize = (*self).into();

        let other_isize: isize = (*other).into();

        return self_isize.cmp(&other_isize);
    }
}

#[cfg(test)]
pub mod test {
    use crate::tryte::TryteAddResult;

    use super::{Trit, Tryte};

    #[test]
    fn add_tryte() {
        let n_one = Trit::NOne;
        let zero = Trit::Zero;
        let p_one = Trit::POne;

        let thing_0 = Tryte([n_one; 9]);
        let thing_1 = Tryte([p_one; 9]);

        assert_eq!(
            TryteAddResult {
                carry: Trit::NOne,
                result: Tryte([p_one, zero, zero, zero, zero, zero, zero, zero, zero])
            },
            thing_0 + thing_0
        );
        
        assert_eq!(
            TryteAddResult {
                carry: Trit::POne,
                result: Tryte([n_one, zero, zero, zero, zero, zero, zero, zero, zero])
            },
            thing_1 + thing_1
        );
    }

    #[test]
    fn compare_tryte() {
        let n_one = Trit::NOne;
        let zero  = Trit::Zero;
        let p_one = Trit::POne;

        let mut start = Tryte([n_one; 9]);
        let one = Tryte([p_one, zero, zero, zero, zero, zero, zero, zero, zero]);

        // Fun fact this enumerates the bijection between + to - $\pm\frac{3^9 - 1}{2}$ $\in$
        // $\mathbb{Z}$
        while start != Tryte([p_one; 9]) {
            let temp = (start + one).result;
            assert!(temp > start);
            start = temp;
        }

        let big = Tryte([zero, zero, zero, zero, zero, zero, zero, zero, p_one]);
        assert!((big + big).carry == Trit::POne);

    }
}
