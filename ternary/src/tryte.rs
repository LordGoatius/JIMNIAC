use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Rem, Shl, Shr, Sub},
    str::FromStr,
};

use crate::{
    trits::{Trit, TritAddResult},
    word::Word,
    *,
};

/// I'm manually bit packing this because dependencies are annoying
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Tryte(pub(crate) u32);

impl Hash for Tryte {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.num());
    }
}

impl Debug for Tryte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .into_iter()
                .map(<Trit as Into<char>>::into)
                .collect::<String>(),
        )
    }
}

impl FromStr for Tryte {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 9 {
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
                let mut ret: [Trit; 9] = Tryte::ZERO.into();
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

impl IntoIterator for Tryte {
    type Item = Trit;

    type IntoIter = <[Trit; 9] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let ret: [Trit; 9] = self.into();
        ret.into_iter()
    }
}

impl From<[Trit; 9]> for Tryte {
    fn from(value: [Trit; 9]) -> Self {
        Tryte(
            value
                .into_iter()
                .enumerate()
                .map(|(i, trit)| -> u32 {
                    let val = trit as u32;
                    val << (2 * i)
                })
                .fold(0, std::ops::BitOr::bitor),
        )
    }
}

impl From<[[Trit; 3]; 3]> for Tryte {
    fn from(value: [[Trit; 3]; 3]) -> Self {
        unsafe { std::mem::transmute::<[[Trit; 3]; 3], [Trit; 9]>(value) }.into()
    }
}

impl From<Trit> for Tryte {
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
        ]
        .into()
    }
}

impl From<Tryte> for [Trit; 9] {
    fn from(value: Tryte) -> Self {
        let value = value.0;
        [0u32, 1, 2, 3, 4, 5, 6, 7, 8].map(|i| unsafe {
            std::mem::transmute::<u8, Trit>(((value >> (2 * i)) & TRIT_BIT_MASK as u32) as u8)
        })
    }
}

impl From<Tryte> for [[Trit; 3]; 3] {
    fn from(value: Tryte) -> Self {
        let value: [Trit; 9] = value.into();
        unsafe {
            std::mem::transmute(value)
        }
    }
}

impl From<Word> for Tryte {
    fn from(value: Word) -> Self {
        Tryte(value.0 as u32 & TRYTE_BIT_MASK)
    }
}

impl From<Tryte> for isize {
    fn from(value: Tryte) -> Self {
        let arr: [Trit; 9] = value.into();
        arr.map(<Trit as Into<isize>>::into)
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, trit)| acc + (3isize.pow(i as u32) * trit))
    }
}

impl From<isize> for Tryte {
    fn from(mut value: isize) -> Self {
        let mut sum: Tryte = [Trit::Zero; 9].into();
        let neg = value.is_negative();

        if neg {
            value = -value;
        }

        for i in 0..(isize::BITS - 1) {
            let bit: Tryte = match (value >> i) & 1 {
                0 => Trit::Zero,
                1 => Trit::POne,
                _ => unreachable!(),
            }
            .into();

            sum = sum + (bit * Tryte::pow_isize(Tryte::TWO, i as isize));
        }

        if neg {
            sum = -sum;
        }

        sum
    }
}

impl Add for Tryte {
    type Output = Tryte;

    fn add(self, rhs: Self) -> Self::Output {
        let mut val: [Trit; 9] = [Trit::Zero; 9];
        let mut carry = Trit::Zero;

        for (i, (l, r)) in self.into_iter().zip(rhs).enumerate() {
            let TritAddResult { carry: c, result } = (l + r) + carry;
            val[i] = result;
            carry = c;
        }
        let mut ret: Tryte = val.into();
        ret.0 = ret.0 | ((carry as u32) << TRYTE_BIT_LEN);
        ret
    }
}

impl Add<Trit> for Tryte {
    type Output = Tryte;

    fn add(self, rhs: Trit) -> Self::Output {
        let val: Tryte = [
            rhs,
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

impl Sub for Tryte {
    type Output = Tryte;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Neg for Tryte {
    type Output = Tryte;

    fn neg(self) -> Self::Output {
        let arr: [Trit; 9] = self.into();
        arr.map(std::ops::Neg::neg).into()
    }
}

impl Shl<usize> for Tryte {
    type Output = Tryte;

    fn shl(self, rhs: usize) -> Self::Output {
        Tryte(
            #[rustfmt::ignore]
            ((self.0 << (2 * rhs)) & TRYTE_BIT_MASK)
                | (Tryte::ZERO.0 >> (TRYTE_BIT_LEN - (2 * rhs))),
        )
    }
}

impl Shr<usize> for Tryte {
    type Output = Tryte;

    fn shr(self, rhs: usize) -> Self::Output {
        Tryte(
            ((self.num() >> (2 * rhs)) & TRYTE_BIT_MASK)
                | (Tryte::ZERO.0 << (TRYTE_BIT_LEN - (2 * rhs))),
        )
    }
}

impl Mul<Trit> for Tryte {
    type Output = Tryte;

    fn mul(self, rhs: Trit) -> Self::Output {
        let arr: [Trit; 9] = self.into();
        arr.map(|trit| trit * rhs).into()
    }
}

impl Mul for Tryte {
    type Output = Tryte;

    fn mul(self, rhs: Self) -> Self::Output {
        rhs.into_iter()
            .enumerate()
            .map(|(i, trit)| (self * trit) << i)
            .fold(Tryte::ZERO, |acc, right| acc + right)
    }
}

impl Div for Tryte {
    type Output = Option<Tryte>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == Tryte::ZERO {
            None
        } else {
            let lhs: isize = self.into();
            let rhs: isize = rhs.into();
            Some(lhs.div_euclid(rhs).into())
        }
    }
}

impl Rem for Tryte {
    type Output = Option<Tryte>;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs == Tryte::ZERO {
            None
        } else {
            let lhs: isize = self.into();
            let rhs: isize = rhs.into();
            Some(lhs.rem_euclid(rhs).into())
        }
    }
}

impl BitAnd for Tryte {
    type Output = Tryte;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut ret: [Trit; 9] = Tryte::ZERO.into();
        let arr0: [Trit; 9] = self.into();
        let arr1: [Trit; 9] = rhs.into();
        for i in 0..9 {
            ret[i] = arr0[i] & arr1[i];
        }
        ret.into()
    }
}

impl BitOr for Tryte {
    type Output = Tryte;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut ret: [Trit; 9] = Tryte::ZERO.into();
        let arr0: [Trit; 9] = self.into();
        let arr1: [Trit; 9] = rhs.into();
        for i in 0..9 {
            ret[i] = arr0[i] | arr1[i];
        }
        ret.into()
    }
}

impl PartialEq for Tryte {
    fn eq(&self, other: &Self) -> bool {
        self.num() == other.num()
    }
}

impl Eq for Tryte {}

impl PartialOrd for Tryte {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tryte {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs: isize = (*self).into();
        let rhs: isize = (*other).into();
        lhs.cmp(&rhs)
    }
}

impl Tryte {
    pub const PONE: Tryte = Tryte(0b101010101010101011);
    pub const ZERO: Tryte = Tryte(0b101010101010101010);
    pub const NONE: Tryte = Tryte(0b101010101010101001);
    pub const TWO: Tryte = Tryte(0b101010101010101101);
    pub const MIN: Tryte = Tryte(0b010101010101010101);
    pub const MAX: Tryte = Tryte(0b111111111111111111);
    pub const TRYTE_BIT_MASK: u32 = 0b111111111111111111;
    pub const TRYTE_SIZE: usize = 3usize.pow(9);

    pub unsafe fn from_num(num: u32) -> Tryte {
        unsafe {
            std::mem::transmute(num)
        }
    }

    pub const fn get(&self, idx: usize) -> Option<Trit> {
        if idx < 9 {
            unsafe { std::mem::transmute((self.0 >> (2 * idx)) as u8 & TRIT_BIT_MASK) }
        } else {
            None
        }
    }

    pub const fn isize(&self) -> isize {
        let value = self.0;
        let mut arr = [Trit::Zero; 9];
        let mut count = 0;
        while count < 9 {
            arr[count] = unsafe {
                std::mem::transmute::<u8, Trit>(
                    ((value >> (2 * count)) & TRIT_BIT_MASK as u32) as u8,
                )
            };
            count += 1;
        }
        let mut count = 0;
        let mut sum = 0;
        while count < 9 {
            let trit = match arr[count] {
                Trit::NOne => -1,
                Trit::Zero => 0,
                Trit::POne => 1,
            };
            sum += 3isize.pow(count as u32) * trit;
            count += 1;
        }
        sum
    }

    pub const fn from_arr_2d(value: [[Trit; 3]; 3]) -> Tryte {
        let val = [
            value[0][0],
            value[0][1],
            value[0][2],
            value[1][0],
            value[1][1],
            value[1][2],
            value[2][0],
            value[2][1],
            value[2][2],
        ];
        let mut inner = 0;
        let mut count = 0;
        while count < 9 {
            let val = val[count] as u32;
            inner |= val << (2 * count);
            count += 1;
        }
        Tryte(inner)
    }

    pub fn pow_isize(lhs: Tryte, rhs: isize) -> Tryte {
        if rhs < 0 {
            [Trit::Zero; 9].into()
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

    pub fn carry(&self) -> Trit {
        unsafe { std::mem::transmute((self.0 >> TRYTE_BIT_LEN) as u8 & TRIT_BIT_MASK) }
    }

    pub fn num(&self) -> u32 {
        (self.0) & TRYTE_BIT_MASK
    }
}

#[cfg(test)]
pub mod test {
    use crate::{trits::Trit, tryte::Tryte};

    const PONE: Trit = Trit::POne;
    const NONE: Trit = Trit::NOne;
    const ZERO: Trit = Trit::Zero;

    #[test]
    fn test_convert() {
        let arr = [ZERO, PONE, PONE, PONE, NONE, NONE, NONE, ZERO, ZERO];
        let tryte: Tryte = arr.into();
        let test_eq: [Trit; 9] = tryte.into();
        assert_eq!(arr, test_eq);
    }

    #[test]
    fn test_add() {
        let ones: Tryte = [PONE; 9].into();
        let one: Tryte = [PONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into();
        let res = ones + one;
        let carry = res.carry();
        let num = res.num();
        let cmp = res.0;
        let res: [Trit; 9] = res.into();
        let exp: [Trit; 9] = [NONE; 9];
        assert_eq!(res, exp);
        assert_eq!(carry, Trit::POne);
        assert_ne!(cmp, num);

        let nones: Tryte = [NONE; 9].into();
        let none: Tryte = [NONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into();
        let res = nones + none;
        let carry = res.carry();
        let num = res.num();
        let cmp = res.0;
        let res: [Trit; 9] = res.into();
        let exp: [Trit; 9] = [PONE; 9];
        assert_eq!(res, exp);
        assert_eq!(carry, Trit::NOne);
        assert_ne!(cmp, num);

        let zeros: Tryte = [ZERO; 9].into();
        let one: Tryte = [NONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into();
        let res = zeros + one;
        let carry = res.carry();
        let res: [Trit; 9] = res.into();
        let exp: [Trit; 9] = one.into();
        assert_eq!(res, exp);
        assert_eq!(carry, Trit::Zero);
    }

    #[test]
    fn test_ord() {
        let mut min: Tryte = [NONE; 9].into();
        for _ in 0..9841 {
            let add = min + PONE;
            assert!(add > min);
            min = add;
        }
    }

    #[test]
    fn consts() {
        let zero: Tryte = [Trit::Zero; 9].into();
        assert_eq!(zero, Tryte::ZERO);
        let pone: Tryte = [
            Trit::POne,
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
        assert_eq!(pone, Tryte::PONE);

        let none: Tryte = [
            Trit::NOne,
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
        assert_eq!(none, Tryte::NONE);
    }

    #[test]
    fn test_mul() {
        let one = Tryte::PONE;
        let three = Tryte::PONE << 1;
        let three_arr: Tryte = [
            Trit::Zero,
            Trit::POne,
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
        let nine_arr: Tryte = [
            Trit::Zero,
            Trit::Zero,
            Trit::POne,
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
    fn test_mul_neg() {
        let one = Tryte::NONE;
        assert_eq!(Tryte::NONE * Tryte::NONE, Tryte::PONE);
        let three = Tryte::NONE << 1;
        let three_arr: Tryte = [
            Trit::Zero,
            Trit::NOne,
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
        assert_eq!(-three, one * three);
        let nine_arr: Tryte = [
            Trit::Zero,
            Trit::Zero,
            Trit::POne,
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
        for i in -9840..=-9838 {
            let tryte: Tryte = i.into();
            let isize: isize = tryte.into();
            assert_eq!(i, isize);
        }
    }

    #[test]
    fn test_shr() {
        let tryte = Tryte::PONE << 8;
        for i in 0..9 {
            assert_eq!(tryte >> i, 3isize.pow((8 - i) as u32).into());
        }
    }

    #[test]
    fn test_shl() {
        let tryte = Tryte::PONE;
        for i in 0..9 {
            assert_eq!(tryte << i, 3isize.pow(i as u32).into());
        }
    }

    #[test]
    fn from_str() {
        assert_eq!(Tryte::ZERO, "0".parse().unwrap());
        assert_eq!(Tryte::NONE, "T".parse().unwrap());
        assert_eq!(Tryte::PONE, "1".parse().unwrap());
        use Trit::*;
        let tryte: Tryte = [NOne, POne, Zero, Zero, Zero, POne, NOne, POne, Zero].into();
        assert_eq!(tryte, "01T10001T".parse().unwrap());
    }

    extern crate test;

    #[bench]
    fn test_div(b: &mut test::Bencher) {
        let i: isize = -9840;
        let r: isize = 18;
        let div = i.div_euclid(r);
        let rem = i.rem_euclid(r);
        let i_tryte: Tryte = (-9840).into();
        let r_tryte: Tryte = 18.into();
        b.iter(|| {
            assert_eq!(div, (i_tryte / r_tryte).unwrap().into());
            assert_eq!(rem, (i_tryte % r_tryte).unwrap().into());
        });
    }
}
