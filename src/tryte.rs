use std::ops::{Deref, DerefMut};

use crate::trits::*;

pub mod binops;
pub mod unops;
pub mod tritops;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Tryte(pub(crate) [Trit; 9]);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TryteAddResult {
    pub carry: Trit,
    pub result: Tryte,
}

//=== Impl Tryte ===//

impl Tryte {
    fn abs(value: Self) -> Self {
        if value < Tryte::default() {
            -value
        } else {
            value
        }
    }
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
                Trit::NOne => -isize::pow(3, i as u32),
                Trit::Zero => 0,
                Trit::POne => isize::pow(3, i as u32),
            })
            .sum()
    }
}

impl PartialOrd for Tryte {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tryte {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_isize:  isize = (*self).into();

        let other_isize: isize = (*other).into();

        self_isize.cmp(&other_isize)
    }
}

#[cfg(test)]
pub mod test {
    use crate::tryte::TryteAddResult;

    use super::{Trit, Tryte};

    #[test]
    fn iter_tribble() {
        let n_one = Trit::NOne;
        let zero = Trit::Zero;
        let p_one = Trit::POne;

        let mut tryte = Tryte([n_one, n_one, n_one, zero, zero, zero, zero, zero, zero]);

        for _ in 0..27 {
            println!("{tryte:?}");
            tryte = (tryte + p_one).result;
        }
    }

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

    #[test]
    fn abs_val() {
        let n_one = Trit::NOne;
        let zero  = Trit::Zero;
        let p_one = Trit::POne;

        let n = Tryte([Trit::NOne; 9]);
        assert_eq!(Tryte::abs(n), -n);

        let big = Tryte([zero, zero, p_one, zero, p_one, zero, zero, n_one, zero]);
        assert_eq!(Tryte::abs(big), -big);
        assert_eq!(Tryte::abs(-big), -big);
    }
}
