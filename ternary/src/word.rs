use std::{
    fmt::Display, hash::Hash, hint::unreachable_unchecked, ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Rem, Shl, Shr, Sub}, str::FromStr
};

use crate::{
    trits::{Trit, TritAddResult},
    tryte::Tryte,
    *,
};

/// I'm manually bit packing this because dependencies are annoying
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Word(pub(crate) u64);

impl Default for Word {
    fn default() -> Self {
        Word::ZERO
    }
}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.num());
    }
}

impl IntoIterator for Word {
    type Item = Trit;

    type IntoIter = <[Trit; 27] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let ret: [Trit; 27] = self.into();
        ret.into_iter()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .into_iter()
                .map(Trit::to_char)
                .rev()
                .collect::<String>(),
        )
    }
}

impl FromStr for Word {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 27 {
            return Err(());
        }
        match s.as_ascii() {
            None => Err(()),
            Some(s) => {
                for i in s.iter().map(|asciichar| asciichar.to_char()) {
                    if i != 'T' && i != '0' && i != '1' {
                        return Err(());
                    }
                }
                let mut ret: [Trit; 27] = Word::ZERO.into();
                for (i, char) in s.iter().map(|char| char.to_char()).rev().enumerate() {
                    ret[i] = match char {
                        'T' => Trit::NOne,
                        '0' => Trit::Zero,
                        '1' => Trit::POne,
                        _ => unreachable!(),
                    }
                }
                Ok(ret.into())
            }
        }
    }
}

impl From<[Trit; 27]> for Word {
    fn from(value: [Trit; 27]) -> Self {
        Word(
            value
                .into_iter()
                .enumerate()
                .map(|(i, trit)| -> u64 {
                    let val = trit as u64;
                    val << (2 * i)
                })
                .fold(0, std::ops::BitOr::bitor),
        )
    }
}

impl From<[Tryte; 3]> for Word {
    fn from(value: [Tryte; 3]) -> Self {
        Word(
            value
                .into_iter()
                .enumerate()
                .map(|(i, trit)| -> u64 {
                    let val = trit.num() as u64;
                    val << (18 * i)
                })
                .fold(0, std::ops::BitOr::bitor),
        )
    }
}

impl From<Word> for [Tryte; 3] {
    fn from(value: Word) -> Self {
        let mut zero = [Tryte::ZERO; 3];
        zero[0] = Tryte((value.0 as u32) & TRYTE_BIT_MASK);
        zero[1] = Tryte(((value.0 >> 18) as u32) & TRYTE_BIT_MASK);
        zero[2] = Tryte(((value.0 >> 36) as u32) & TRYTE_BIT_MASK);
        zero
    }
}

impl From<[[Trit; 3]; 9]> for Word {
    fn from(value: [[Trit; 3]; 9]) -> Self {
        unsafe { std::mem::transmute::<[[Trit; 3]; 9], [Trit; 27]>(value) }.into()
    }
}

impl From<Tryte> for Word {
    fn from(value: Tryte) -> Self {
        Word(value.num() as u64 | WORD_ZERO_TOP)
    }
}

impl From<Trit> for Word {
    fn from(value: Trit) -> Self {
        [
            value,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into()
    }
}

impl From<Word> for [Trit; 27] {
    fn from(value: Word) -> Self {
        let value = value.0;
        [
            0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26,
        ]
        .map(|i| unsafe {
            std::mem::transmute::<u8, Trit>(((value >> (2 * i)) & TRIT_BIT_MASK as u64) as u8)
        })
    }
}

impl From<&Word> for [Trit; 27] {
    fn from(value: &Word) -> Self {
        let value = value.0;
        [
            0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26,
        ]
        .map(|i| unsafe {
            std::mem::transmute::<u8, Trit>(((value >> (2 * i)) & TRIT_BIT_MASK as u64) as u8)
        })
    }
}

impl From<Word> for [[Trit; 3]; 9] {
    fn from(value: Word) -> Self {
        let value: [Trit; 27] = value.into();
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Word> for isize {
    fn from(value: Word) -> Self {
        let arr: [Trit; 27] = value.into();
        arr.map(<Trit as Into<isize>>::into)
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, trit)| acc + (3isize.pow(i as u32) * trit))
    }
}

impl From<isize> for Word {
    fn from(mut value: isize) -> Self {
        let mut sum: Word = [Trit::Zero; 27].into();
        let neg = value.is_negative();

        if neg {
            value = -value;
        }

        for i in 0..(isize::BITS - 1) {
            let bit: Word = match (value >> i) & 1 {
                0 => Trit::Zero,
                1 => Trit::POne,
                _ => unreachable!(),
            }
            .into();

            sum = sum + (bit * Word::pow_isize(Word::TWO, i as isize));
        }

        if neg {
            sum = -sum;
        }

        sum
    }
}

impl Add for Word {
    type Output = Word;

    fn add(self, rhs: Self) -> Self::Output {
        let mut val: [Trit; 27] = [Trit::Zero; 27];
        let mut carry = Trit::Zero;

        for (i, (l, r)) in self.into_iter().zip(rhs).enumerate() {
            let TritAddResult { carry: c, result } = (l + r) + carry;
            val[i] = result;
            carry = c;
        }
        let mut ret: Word = val.into();
        ret.0 = ret.0 | ((carry as u64) << WORD_BIT_LEN);
        ret
    }
}

impl Add<Trit> for Word {
    type Output = Word;

    fn add(self, rhs: Trit) -> Self::Output {
        let val: Word = [
            rhs,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();

        self + val
    }
}

impl Sub for Word {
    type Output = Word;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Neg for Word {
    type Output = Word;

    fn neg(self) -> Self::Output {
        let arr: [Trit; 27] = self.into();
        arr.map(std::ops::Neg::neg).into()
    }
}

impl Shl<usize> for Word {
    type Output = Word;

    fn shl(self, rhs: usize) -> Self::Output {
        Word(
            #[rustfmt::ignore]
            ((self.0 << (2 * rhs)) & WORD_BIT_MASK)
                | (Word::ZERO.0 >> (WORD_BIT_LEN - (2 * rhs))),
        )
    }
}

impl Shr<usize> for Word {
    type Output = Word;

    fn shr(self, rhs: usize) -> Self::Output {
        Word(
            ((self.num() >> (2 * rhs)) & WORD_BIT_MASK)
                | (Word::ZERO.0 << (WORD_BIT_LEN - (2 * rhs))),
        )
    }
}

impl Mul<Trit> for Word {
    type Output = Word;

    fn mul(self, rhs: Trit) -> Self::Output {
        let arr: [Trit; 27] = self.into();
        arr.map(|trit| trit * rhs).into()
    }
}

impl Mul for Word {
    type Output = Word;

    fn mul(self, rhs: Self) -> Self::Output {
        rhs.into_iter()
            .enumerate()
            .map(|(i, trit)| (self * trit) << i)
            .fold(Word::ZERO, |acc, right| acc + right)
    }
}

impl Div for Word {
    type Output = Option<Word>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == Word::ZERO {
            None
        } else {
            let lhs: isize = self.into();
            let rhs: isize = rhs.into();
            Some(lhs.div_euclid(rhs).into())
        }
    }
}

impl Rem for Word {
    type Output = Option<Word>;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs == Word::ZERO {
            None
        } else {
            let lhs: isize = self.into();
            let rhs: isize = rhs.into();
            Some(lhs.rem_euclid(rhs).into())
        }
    }
}

impl BitAnd for Word {
    type Output = Word;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut ret: [Trit; 27] = Word::ZERO.into();
        let arr0: [Trit; 27] = self.into();
        let arr1: [Trit; 27] = rhs.into();
        for i in 0..27 {
            ret[i] = arr0[i] & arr1[i];
        }
        ret.into()
    }
}

impl BitOr for Word {
    type Output = Word;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut ret: [Trit; 27] = Word::ZERO.into();
        let arr0: [Trit; 27] = self.into();
        let arr1: [Trit; 27] = rhs.into();
        for i in 0..27 {
            ret[i] = arr0[i] | arr1[i];
        }
        ret.into()
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.num() == other.num()
    }
}

impl Eq for Word {}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs: isize = (*self).into();
        let rhs: isize = (*other).into();
        lhs.cmp(&rhs)
    }
}

impl Word {
    pub const WORD_BIT_MASK: u64 = 0b111111111111111111111111111111111111111111111111111111;
    pub const PONE: Word = Word(0b101010101010101010101010101010101010101010101010101011);
    pub const ZERO: Word = Word(0b101010101010101010101010101010101010101010101010101010);
    pub const NONE: Word = Word(0b101010101010101010101010101010101010101010101010101001);
    pub const TWO: Word = Word(0b101010101010101010101010101010101010101010101010101101);
    pub const MIN: Word = Word(0b010101010101010101010101010101010101010101010101010101);
    pub const MAX: Word = Word(0b111111111111111111111111111111111111111111111111111111);
    pub const WORD_SIZE: usize = 3usize.pow(27);

    pub const unsafe fn from_u64(num: u64) -> Word {
        Word(num)
    }

    pub fn rot(&self, val: isize) -> Word {
        let mut word: [Trit; 27] = self.into();
        let func = match val.signum() {
            -1 => <[Trit]>::rotate_right,
            1 => <[Trit]>::rotate_left,
            _ => unsafe { unreachable_unchecked() }
        };
        func(&mut word, val.abs() as usize);
        word.into()
    }

    pub const fn get(&self, idx: usize) -> Option<Trit> {
        if idx < 27 {
            unsafe { std::mem::transmute((self.0 >> (2 * idx)) as u8 & TRIT_BIT_MASK) }
        } else {
            None
        }
    }

    pub fn pow_isize(lhs: Word, rhs: isize) -> Word {
        if rhs < 0 {
            [Trit::Zero; 27].into()
        } else if rhs == 1 || lhs == Self::PONE {
            lhs
        } else {
            let mut ret = Self::PONE;
            let mut count = rhs;
            while count > 0 {
                ret = ret * lhs;
                count -= 1;
            }
            ret
        }
    }

    pub fn get_carry(&self) -> Trit {
        unsafe { std::mem::transmute((self.0 >> WORD_BIT_LEN) as u8 & TRIT_BIT_MASK) }
    }

    pub fn get_sign(&self) -> Trit {
        let nums: [Trit; 27] = self.into();
        for &trit in nums.iter().rev() {
            if trit != Trit::Zero {
                return trit;
            }
        }
        return Trit::Zero;
    }

    pub fn get_parity(&self) -> Trit {
        unsafe {
            std::mem::transmute(self.num() as u8 & Trit::TRIT_BIT_MASK)
        }
    }

    pub fn num(&self) -> u64 {
        (self.0) & WORD_BIT_MASK
    }
}

#[cfg(test)]
pub mod test {
    use crate::{trits::Trit, tryte::Tryte, word::Word};

    const PONE: Trit = Trit::POne;
    const NONE: Trit = Trit::NOne;
    const ZERO: Trit = Trit::Zero;

    #[test]
    fn test_convert() {
        let arr = [
            ZERO, PONE, PONE, PONE, NONE, NONE, NONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
            ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
        ];
        let tryte: Word = arr.into();
        let test_eq: [Trit; 27] = tryte.into();
        assert_eq!(arr, test_eq);
    }

    #[test]
    fn test_add() {
        let ones: Word = [PONE; 27].into();
        let one: Word = [
            PONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
            ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
        ]
        .into();
        let res = ones + one;
        let carry = res.get_carry();
        let num = res.num();
        let cmp = res.0;
        let res: [Trit; 27] = res.into();
        let exp: [Trit; 27] = [NONE; 27];
        assert_eq!(res, exp);
        assert_eq!(carry, Trit::POne);
        assert_ne!(cmp, num);

        let nones: Word = [NONE; 27].into();
        let none: Word = [
            NONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
            ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
        ]
        .into();
        let res = nones + none;
        let carry = res.get_carry();
        let num = res.num();
        let cmp = res.0;
        let res: [Trit; 27] = res.into();
        let exp: [Trit; 27] = [PONE; 27];
        assert_eq!(res, exp);
        assert_eq!(carry, Trit::NOne);
        assert_ne!(cmp, num);

        let zeros: Word = [ZERO; 27].into();
        let res = zeros + one;
        let carry = res.get_carry();
        let res: [Trit; 27] = res.into();
        let exp: [Trit; 27] = one.into();
        assert_eq!(res, exp);
        assert_eq!(carry, Trit::Zero);
    }

    #[test]
    fn test_ord() {
        let min: Tryte = [NONE; 9].into();
        let val1: isize = min.into();
        let mut min: Word = min.into();
        let val2: isize = min.into();
        assert_eq!(val1, val2);
        for _ in 0..9841 {
            let add = min + PONE;
            assert!(add > min, "{add:?}, {min:?}");
            min = add;
        }
    }

    #[test]
    fn consts() {
        let zero: Word = [Trit::Zero; 27].into();
        assert_eq!(zero, Word::ZERO);
        let pone: Word = [
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();
        assert_eq!(pone, Word::PONE);

        let none: Word = [
            Trit::NOne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();
        assert_eq!(none, Word::NONE);
    }

    #[test]
    fn test_mul() {
        let one = Word::PONE;
        let three = Word::PONE << 1;
        let three_arr: Word = [
            Trit::Zero,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();
        assert_eq!(three, three_arr);
        assert_eq!(three, one * three);
        let nine_arr: Word = [
            Trit::Zero,
            Trit::Zero,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();
        assert_eq!(nine_arr, three * three);
    }

    #[test]
    fn test_convertion() {
        for i in -9841..=-9839 {
            let tryte: Word = i.into();
            let isize: isize = tryte.into();
            assert_eq!(i, isize);
        }
    }

    extern crate test;

    #[bench]
    fn test_div(b: &mut test::Bencher) {
        let i: isize = -38127987;
        let r: isize = 18;
        let div = i.div_euclid(r);
        let rem = i.rem_euclid(r);
        let i_word: Word = (-38127987).into();
        let r_word: Word = 18.into();
        b.iter(|| {
            assert_eq!(div, (i_word / r_word).unwrap().into());
            assert_eq!(rem, (i_word % r_word).unwrap().into());
        });
    }
}
