use std::ops::{Deref, DerefMut};

use crate::{trits::*, tryte::Tryte};

pub mod binops;
pub mod unops;
pub mod tritops;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Word([Trit; 27]);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct WordAddResult {
    carry: Trit,
    result: Word,
}

//== Helper Traits ==//

impl Deref for Word {
    type Target = [Trit; 27];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Word {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[Trit; 27]> for Word {
    fn from(value: [Trit; 27]) -> Self {
        Word(value)
    }
}

impl From<[Tryte; 3]> for Word {
    fn from(value: [Tryte; 3]) -> Self {
        let value = unsafe { std::mem::transmute::<[[Trit; 9]; 3], [Trit; 27]>(value.map(|tryte| *tryte)) };
        Word(value)
    }
}

impl From<Word> for [Tryte; 3] {
    fn from(value: Word) -> Self {
        let value = unsafe { std::mem::transmute::<Word, [Tryte; 3]>(value) };
        value
    }
}

impl From<Word> for isize {
    fn from(value: Word) -> Self {
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

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_isize: isize = self
            .iter()
            .enumerate()
            .map(|(i, trit)| match trit {
                Trit::NOne => isize::pow(3, i as u32) * -1,
                Trit::Zero => 0,
                Trit::POne => isize::pow(3, i as u32) * 1,
            })
            .sum();

        let other_isize: isize = other
            .iter()
            .enumerate()
            .map(|(i, trit)| match trit {
                Trit::NOne => isize::pow(3, i as u32) * -1,
                Trit::Zero => 0,
                Trit::POne => isize::pow(3, i as u32) * 1,
            })
            .sum();

        return self_isize.partial_cmp(&other_isize);
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_isize: isize = self
            .iter()
            .enumerate()
            .map(|(i, trit)| match trit {
                Trit::NOne => isize::pow(i as isize, 3) * -1,
                Trit::Zero => 0,
                Trit::POne => isize::pow(i as isize, 3) * 1,
            })
            .sum();

        let other_isize: isize = other
            .iter()
            .enumerate()
            .map(|(i, trit)| match trit {
                Trit::NOne => isize::pow(i as isize, 3) * -1,
                Trit::Zero => 0,
                Trit::POne => isize::pow(i as isize, 3) * 1,
            })
            .sum();

        return self_isize.cmp(&other_isize);
    }
}

#[cfg(test)]
pub mod test {
    use crate::word::{Word, WordAddResult};

    use super::Trit;

    #[test]
    fn add_word() {
        let n_one = Trit::NOne;
        let zero = Trit::Zero;
        let p_one = Trit::POne;

        let thing = Word([n_one; 27]);

        assert_eq!(
            WordAddResult {
                carry: Trit::NOne,
                result: Word([p_one, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero])
            },
            thing + thing
        );
    }

    #[test]
    fn compare_word() {
        let n_one = Trit::NOne;
        let zero  = Trit::Zero;
        let p_one = Trit::POne;

        let mut start = Word([n_one; 27]);
        let one = Word([p_one, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero]);

        // Fun fact this enumerates the bijection between + to - $\pm\frac{3^27 - 1}{2}$ $\in$
        // $\mathbb{Z}$
        while start != Word([p_one; 27]) {
            let temp = (start + one).result;
            assert!(temp > start);
            start = temp;
        }

        let big = Word([zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, zero, p_one]);
        assert!((big + big).carry == Trit::POne);

    }
}
