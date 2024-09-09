use std::ops::{Add, Deref, DerefMut, Neg, Not, Sub};

use crate::trits::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Word([Trit; 27]);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct WordAddResult {
    carry: Trit,
    result: Word,
}

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

impl Add for Word {
    type Output = WordAddResult;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Word::default();

        let mut result: TritAddResult = self[0] + rhs[0];
        output[0] = result.result;

        for i in 1..27 {
            result = self[i] + rhs[i] + result.carry;
            output[i] = result.result;
        }

        WordAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl Add<Trit> for Word {
    type Output = WordAddResult;

    fn add(self, rhs: Trit) -> Self::Output {
        let mut output = Word::default();

        let mut result: TritAddResult = self[0] + rhs;
        output[0] = result.result;

        for i in 1..27 {
            result = self[i] + result.carry;
            output[i] = result.result;
        }

        WordAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl Sub for Word {
    type Output = WordAddResult;
    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = -rhs;
        let mut output = Word::default();

        let mut result: TritAddResult = self[0] + rhs[0];
        output[0] = result.result;

        for i in 1..27 {
            result = self[i] + rhs[i] + result.carry;
            output[i] = result.result;
        }

        WordAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl Sub<Trit> for Word {
    type Output = WordAddResult;

    fn sub(self, rhs: Trit) -> Self::Output {
        let mut output = Word::default();

        let mut result: TritAddResult = self[0] + -rhs;
        output[0] = result.result;

        for i in 1..27 {
            result = self[i] + result.carry;
            output[i] = result.result;
        }

        WordAddResult {
            result: output,
            carry: result.carry,
        }
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
