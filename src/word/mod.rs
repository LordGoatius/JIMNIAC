use std::{hash::Hash, ops::{Deref, DerefMut}};

use crate::{trits::*, tryte::Tryte};

pub mod binops;
pub mod unops;
pub mod tritops;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Word(pub(crate) [Trit; 27]);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct WordAddResult {
    carry: Trit,
    pub(crate) result: Word,
}

//=== Impl Word ===//

impl Word {
    fn abs(value: Self) -> Self {
        if value < Word::default() {
            -value
        } else {
            value
        }
    }

    pub(crate) fn lowest_tryte(&self) -> Tryte {
        let [ret, _, _] = (*self).into();
        ret
    }

    pub(crate) fn zero_lowest_tryte(&self) -> Word {
        let [_, mid, high]: [Tryte; 3] = (*self).into();
        [[Trit::Zero; 9].into(), mid, high].into()
    }

    pub(crate) fn set_tryte(&mut self, tryte: Tryte) {
        *self = [tryte, Tryte::default(), Tryte::default()].into();
    }
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
        unsafe { std::mem::transmute::<Word, [Tryte; 3]>(value) }
    }
}

impl From<Word> for isize {
    fn from(value: Word) -> Self {
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

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_isize: isize = (*self).into();

        let other_isize: isize = (*other).into();
        self_isize.cmp(&other_isize)
    }
}

#[cfg(test)]
pub mod test {
    use crate::word::{Word, WordAddResult};
    use crate::tryte::Tryte;

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

    //#[test]
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

    #[test]
    fn sizes() {
        assert_eq!(std::mem::size_of::<Word>(), std::mem::size_of::<[Tryte; 3]>());
        assert_eq!(std::mem::size_of::<[[Trit; 9]; 3]>(), std::mem::size_of::<[Tryte; 3]>());
        assert_eq!(std::mem::size_of::<[[Trit; 9]; 3]>(), std::mem::size_of::<Word>());
    }
}