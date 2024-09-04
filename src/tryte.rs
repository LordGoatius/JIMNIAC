use std::ops::{Add, Deref, DerefMut, Neg, Not};

use crate::trits::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Tryte([Trit; 9]);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TryteAddResult {
    carry: Trit,
    result: Tryte,
}

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

impl Add for Tryte {
    type Output = TryteAddResult;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Tryte::default();

        let mut result: TritAddResult = self[0] + rhs[0];
        output[0] = result.result;

        for i in 1..9 {
            result = self[i] + rhs[i] + result.carry;
            output[i] = result.result;
        }

        TryteAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl Add<Trit> for Tryte {
    type Output = TryteAddResult;

    fn add(self, rhs: Trit) -> Self::Output {
        let mut output = Tryte::default();

        let mut result: TritAddResult = self[0] + rhs;
        output[0] = result.result;

        for i in 1..9 {
            result = self[i] + result.carry;
            output[i] = result.result;
        }

        TryteAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl PartialOrd for Tryte {
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

impl Ord for Tryte {
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

#[cfg(test)]
pub mod test {
    use crate::tryte::TryteAddResult;

    use super::{Trit, Tryte};

    #[test]
    fn add_tryte() {
        let n_one = Trit::NOne;
        let zero = Trit::Zero;
        let p_one = Trit::POne;

        let thing = Tryte([n_one; 9]);

        assert_eq!(
            TryteAddResult {
                carry: Trit::NOne,
                result: Tryte([p_one, zero, zero, zero, zero, zero, zero, zero, zero])
            },
            thing + thing
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
