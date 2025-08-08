use std::{fmt::{Binary, Display}, ops::{Deref, DerefMut}};

use crate::{trits::*, word::{consts::TWO_TRYTE, Word}};

pub mod binops;
pub mod tritops;
pub mod unops;
pub mod consts;
#[cfg(feature = "packed")]
pub mod packed;

#[repr(transparent)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Tryte(pub [Trit; 9]);

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

    pub const fn isize(self: Tryte) -> isize {
        let mut res = 0;
        let mut i = 0;
        while i < self.0.len() {
            res += match self.0[i] {
                Trit::NOne => -isize::pow(3, i as u32),
                Trit::Zero => 0,
                Trit::POne => isize::pow(3, i as u32),
            };
            i += 1;
        }
        res
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

impl From<Tryte> for [[Trit; 3]; 3] {
    fn from(value: Tryte) -> Self {
        unsafe { std::mem::transmute(value.0) }
    }
}

impl From<[[Trit; 3]; 3]> for Tryte {
    fn from(value: [[Trit; 3]; 3]) -> Self {
        Tryte([
            value[0][0],
            value[0][1],
            value[0][2],
            value[1][0],
            value[1][1],
            value[1][2],
            value[2][0],
            value[2][1],
            value[2][2],
        ])
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


impl From<Trit> for Tryte {
    fn from(value: Trit) -> Self {
        let mut def = Tryte::default();
        def[0] = value;
        def
    }
}

impl From<isize> for Tryte {
    fn from(mut value: isize) -> Self {
        let mut sum = Tryte::default();
        let neg = value.is_negative();

        if neg {
            value = -value;
        }

        for i in 0..(isize::BITS - 1) {
            let bit: Tryte = match (value >> i) & 1 {
                0 => Trit::Zero,
                1 => Trit::POne,
                _ => unreachable!()
            }.into();

            sum = (sum + (bit * Tryte::pow_isize(TWO_TRYTE, i as isize))).result;
        }

        if neg {
            sum = -sum;
        }

        sum
    }
}

impl From<Tryte> for Word {
    fn from(value: Tryte) -> Self {
        [value, Tryte::default(), Tryte::default()].into()
    }
}

impl PartialOrd for Tryte {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tryte {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_isize: isize = (*self).into();

        let other_isize: isize = (*other).into();

        self_isize.cmp(&other_isize)
    }
}

pub fn to_letter(arr: [Trit; 3]) -> char {
    use Trit::*;
    match arr {
        [NOne, NOne, NOne] => 'Z',
        [Zero, NOne, NOne] => 'Y',
        [POne, NOne, NOne] => 'X',
        [NOne, Zero, NOne] => 'W',
        [Zero, Zero, NOne] => 'V',
        [POne, Zero, NOne] => 'U',
        [NOne, POne, NOne] => 'T',
        [Zero, POne, NOne] => 'S',
        [POne, POne, NOne] => 'R',
        [NOne, NOne, Zero] => 'Q',
        [Zero, NOne, Zero] => 'P',
        [POne, NOne, Zero] => 'O',
        [NOne, Zero, Zero] => 'N',
        [Zero, Zero, Zero] => '0',
        [POne, Zero, Zero] => 'A',
        [NOne, POne, Zero] => 'B',
        [Zero, POne, Zero] => 'C',
        [POne, POne, Zero] => 'D',
        [NOne, NOne, POne] => 'E',
        [Zero, NOne, POne] => 'F',
        [POne, NOne, POne] => 'G',
        [NOne, Zero, POne] => 'H',
        [Zero, Zero, POne] => 'I',
        [POne, Zero, POne] => 'J',
        [NOne, POne, POne] => 'K',
        [Zero, POne, POne] => 'L',
        [POne, POne, POne] => 'M',
    }
}

impl Binary for Tryte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if f.alternate() {
            "0t"
        } else {
            ""
        };

        f.pad(&format!("{}{}", prefix, self.iter().map(|trit| <Trit as Into<char>>::into(*trit)).collect::<String>())[..])
    }
}

impl Display for Tryte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.pad(&format!("{}", <Tryte as Into<isize>>::into(*self))[..])
        } else {
            let arr: [[Trit; 3]; 3] = (*self).into();
            let arr = arr.map(to_letter);
            f.pad(&format!("[{}, {}, {}]", arr[0], arr[1], arr[2])[..])
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::tryte::TryteAddResult;

    use super::{Trit, Tryte};

    #[test]
    fn test_from_isize() {
        let tryte: Tryte = [Trit::NOne, Trit::POne, Trit::POne, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero].into();
        let neg_tryte = -tryte;
        assert_eq!(tryte, 11isize.into());
        assert_eq!(neg_tryte, (-11isize).into());
    }

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
        let zero = Trit::Zero;
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
        let zero = Trit::Zero;
        let p_one = Trit::POne;

        let n = Tryte([Trit::NOne; 9]);
        assert_eq!(Tryte::abs(n), -n);

        let big = Tryte([zero, zero, p_one, zero, p_one, zero, zero, n_one, zero]);
        assert_eq!(Tryte::abs(big), -big);
        assert_eq!(Tryte::abs(-big), -big);
    }
}
