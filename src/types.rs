use std::ops::{Index, IndexMut};

use crate::errors::TritParseErr;

/// A Trit represents a balanced ternary trit, which
/// is 0, 1, or -1. The default is 0
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Trit {
    NOne,
    #[default]
    Zero,
    POne,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct TritAddResult {
    carry:  Trit,
    result: Trit,
}

/// A tryte is the smallest possible addressable unit.
/// It's made up of 3 trits (eventuallly may make it generic over size, for 
/// different machine architectures). 
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tryte {
    pub value: [Trit; 3]
}

impl PartialOrd for Tryte {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self[2], other[2]) {
            (Trit::POne, Trit::Zero) => return Some(std::cmp::Ordering::Greater),
            (Trit::POne, Trit::NOne) => return Some(std::cmp::Ordering::Greater),
            (Trit::Zero, Trit::NOne) => return Some(std::cmp::Ordering::Greater),
            (Trit::Zero, Trit::POne) => return Some(std::cmp::Ordering::Less),
            (Trit::NOne, Trit::POne) => return Some(std::cmp::Ordering::Less),
            (Trit::NOne, Trit::Zero) => return Some(std::cmp::Ordering::Less),
            _ => (),
        }
        match (self[1], other[1]) {
            (Trit::POne, Trit::Zero) => return Some(std::cmp::Ordering::Greater),
            (Trit::POne, Trit::NOne) => return Some(std::cmp::Ordering::Greater),
            (Trit::Zero, Trit::NOne) => return Some(std::cmp::Ordering::Greater),
            (Trit::Zero, Trit::POne) => return Some(std::cmp::Ordering::Less),
            (Trit::NOne, Trit::POne) => return Some(std::cmp::Ordering::Less),
            (Trit::NOne, Trit::Zero) => return Some(std::cmp::Ordering::Less),
            _ => (),
        }
        match (self[0], other[0]) {
            (Trit::POne, Trit::Zero) => return Some(std::cmp::Ordering::Greater),
            (Trit::POne, Trit::NOne) => return Some(std::cmp::Ordering::Greater),
            (Trit::Zero, Trit::NOne) => return Some(std::cmp::Ordering::Greater),
            (Trit::Zero, Trit::POne) => return Some(std::cmp::Ordering::Less),
            (Trit::NOne, Trit::POne) => return Some(std::cmp::Ordering::Less),
            (Trit::NOne, Trit::Zero) => return Some(std::cmp::Ordering::Less),
            _ => (),
        }
        return Some(std::cmp::Ordering::Equal);
    }
}

impl Ord for Tryte {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self[2], other[2]) {
            (Trit::POne, Trit::Zero) => return std::cmp::Ordering::Greater,
            (Trit::POne, Trit::NOne) => return std::cmp::Ordering::Greater,
            (Trit::Zero, Trit::NOne) => return std::cmp::Ordering::Greater,
            (Trit::Zero, Trit::POne) => return std::cmp::Ordering::Less,
            (Trit::NOne, Trit::POne) => return std::cmp::Ordering::Less,
            (Trit::NOne, Trit::Zero) => return std::cmp::Ordering::Less,
            _ => (),
        }
        match (self[1], other[1]) {
            (Trit::POne, Trit::Zero) => return std::cmp::Ordering::Greater,
            (Trit::POne, Trit::NOne) => return std::cmp::Ordering::Greater,
            (Trit::Zero, Trit::NOne) => return std::cmp::Ordering::Greater,
            (Trit::Zero, Trit::POne) => return std::cmp::Ordering::Less,
            (Trit::NOne, Trit::POne) => return std::cmp::Ordering::Less,
            (Trit::NOne, Trit::Zero) => return std::cmp::Ordering::Less,
            _ => (),
        }
        match (self[0], other[0]) {
            (Trit::POne, Trit::Zero) => return std::cmp::Ordering::Greater,
            (Trit::POne, Trit::NOne) => return std::cmp::Ordering::Greater,
            (Trit::Zero, Trit::NOne) => return std::cmp::Ordering::Greater,
            (Trit::Zero, Trit::POne) => return std::cmp::Ordering::Less,
            (Trit::NOne, Trit::POne) => return std::cmp::Ordering::Less,
            (Trit::NOne, Trit::Zero) => return std::cmp::Ordering::Less,
            _ => (),
        }
        return std::cmp::Ordering::Equal;
    }
}

impl Index<usize> for Tryte {
    type Output = Trit;
    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Tryte {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl Trit {
    /// Returns a zero trit
    const fn zero() -> Self {
        Trit::Zero
    }

    /// Returns a one trit
    const fn p_one() -> Self {
        Trit::POne
    }

    /// Returns a negative one trit
    const fn n_one() -> Self {
        Trit::NOne
    }

    /// Taken in a 0, 1, or -1, (or 2, as shorthand for -1), and returns
    /// a TritParseErr or a Trit
    fn from_num<T: Into<i64>>(input: T) -> Result<Trit, TritParseErr> {
        match input.into() {
            0  => Ok(Trit::Zero),
            1  => Ok(Trit::POne),
            -1 => Ok(Trit::NOne),
            2  => Ok(Trit::NOne),
            v  => Err(TritParseErr(v))
        }
    }

    /// Bitwise not/unary negation
    fn neg(self) -> Self {
        match self {
            Trit::Zero => Trit::Zero,
            Trit::POne => Trit::NOne,
            Trit::NOne => Trit::POne,
        }
    }

    /// Binary bitwise and
    const fn and(l: Trit, r: Trit) -> Trit {
        match l {
            Trit::NOne => {
                match r {
                    Trit::NOne => Trit::POne,
                    Trit::POne => Trit::NOne,
                    Trit::Zero => Trit::Zero,
                }
            },
            Trit::POne => {
                match r {
                    Trit::NOne => Trit::NOne,
                    Trit::POne => Trit::POne,
                    Trit::Zero => Trit::Zero,
                }
            },
            Trit::Zero => {
                Trit::Zero
            }
        }
    }

    /// Binary bitwise or
    const fn or(l: Trit, r: Trit) -> Trit {
        match l {
            Trit::NOne => {
                match r {
                    Trit::NOne => Trit::POne,
                    Trit::Zero => Trit::NOne,
                    Trit::POne => Trit::Zero,
                }
            },
            Trit::Zero => {
                match r {
                    Trit::NOne => Trit::NOne,
                    Trit::Zero => Trit::Zero,
                    Trit::POne => Trit::POne,
                }
            },
            Trit::POne => {
                match r {
                    Trit::NOne => Trit::Zero,
                    Trit::Zero => Trit::POne,
                    Trit::POne => Trit::NOne,
                }
            },
        }
    }

    /// Binary add 
    pub fn add(l: Trit, r: Trit) -> TritAddResult {
        let mut ret = TritAddResult {
            carry: Trit::Zero,
            result: Trit::or(l, r)
        };
        if (l == Trit::NOne) && (r == Trit::NOne) {
            ret.carry = Trit::NOne;
        } else if (l == Trit::POne) && (r == Trit::POne) {
            ret.carry = Trit::POne;
        }
        ret
    }

    /// Trinary add
    pub fn tri_add(l: Trit, m: Trit, r: Trit) -> TritAddResult {
        let TritAddResult { 
            carry, 
            result: first_digit,
        } = Trit::add(l, r);
        let TritAddResult {
            carry: second_digit,
            result: first_ret,
        } = Trit::add(first_digit, m);
        let second_ret = Trit::add(carry, second_digit).result;
        return TritAddResult{
            carry: second_ret,
            result: first_ret
        };
    }
    
}

impl Tryte {
    /// Returns a tryte with only 0's
    pub const fn zero() -> Self {
        Tryte { value: [Trit::Zero; 3] }
    }

    /// Returns a tryte with only 1's
    pub const fn p_one() -> Self {
        Tryte { value: [Trit::POne; 3] }
    }

    /// Returns a tryte with only -1's
    pub const fn n_one() -> Self {
        Tryte { value: [Trit::NOne; 3] }
    }

    /// Takes in an array of 3 numbers that implement `Into<u64>` and outputs a TritParseErr 
    /// or a valid array of trits
    pub fn from_arr_num<T: Into<i64>>(input: [T; 3]) -> Result<Self, TritParseErr> {
        Ok(Tryte { 
            value: input.try_map(|i| Trit::from_num(i))? 
        })
    }

    /// Returns a Tryte from an array of Trits
    pub fn from_arr(input: [Trit; 3]) -> Result<Self, TritParseErr> {
        Ok(Tryte { 
            value: input 
        })
    }

    /// Unary negation/bitwise negation
    pub fn neg(self) -> Self {
        Tryte {
            value: self.value.map(|val| val.neg())
        }
    }

    /// And w/ trit
    pub fn and_trit(val: Tryte, mask: Trit) -> Tryte {
        let value = [
            Trit::and(val.value[0], mask), 
            Trit::and(val.value[1], mask), 
            Trit::and(val.value[2], mask)];

        Tryte {
            value
        }
    }

    /// And 3 trytes w/ trit
    pub fn and_3_trit(val: [Tryte; 3], mask: Trit) -> [Tryte; 3] {
        let value_0 = [
            Trit::and(val[0][0], mask), 
            Trit::and(val[0][1], mask), 
            Trit::and(val[0][2], mask)];

        let value_1 = [
            Trit::and(val[1][0], mask), 
            Trit::and(val[1][1], mask), 
            Trit::and(val[1][2], mask)];

        let value_2 = [
            Trit::and(val[2][0], mask), 
            Trit::and(val[2][1], mask), 
            Trit::and(val[2][2], mask)];

        return
            [ Tryte { value: value_0 }
            , Tryte { value: value_1 }
            , Tryte { value: value_2 }
            ];
    }

    /// And 9 trytes w/ trit
    pub fn and_9_trit(val: [Tryte; 9], mask: Trit) -> [Tryte; 9] {
        let value_0 = [
            Trit::and(val[0][0], mask), 
            Trit::and(val[0][1], mask), 
            Trit::and(val[0][2], mask)];

        let value_1 = [
            Trit::and(val[1][0], mask), 
            Trit::and(val[1][1], mask), 
            Trit::and(val[1][2], mask)];

        let value_2 = [
            Trit::and(val[2][0], mask), 
            Trit::and(val[2][1], mask), 
            Trit::and(val[2][2], mask)];

        let value_3 = [
            Trit::and(val[3][0], mask), 
            Trit::and(val[3][1], mask), 
            Trit::and(val[3][2], mask)];

        let value_4 = [
            Trit::and(val[4][0], mask), 
            Trit::and(val[4][1], mask), 
            Trit::and(val[4][2], mask)];

        let value_5 = [
            Trit::and(val[5][0], mask), 
            Trit::and(val[5][1], mask), 
            Trit::and(val[5][2], mask)];

        let value_6 = [
            Trit::and(val[6][0], mask), 
            Trit::and(val[6][1], mask), 
            Trit::and(val[6][2], mask)];

        let value_7 = [
            Trit::and(val[7][0], mask), 
            Trit::and(val[7][1], mask), 
            Trit::and(val[7][2], mask)];

        let value_8 = [
            Trit::and(val[8][0], mask), 
            Trit::and(val[8][1], mask), 
            Trit::and(val[8][2], mask)];

        return
            [ Tryte { value: value_0 }
            , Tryte { value: value_1 }
            , Tryte { value: value_2 }
            , Tryte { value: value_3 }
            , Tryte { value: value_4 }
            , Tryte { value: value_5 }
            , Tryte { value: value_6 }
            , Tryte { value: value_7 }
            , Tryte { value: value_8 }
            ];
    }


    /// Binary bitwise and
    pub fn and(l: Tryte, r: Tryte) -> Tryte {
        let value = [
            Trit::and(l.value[0], r.value[0]), 
            Trit::and(l.value[0], r.value[0]), 
            Trit::and(l.value[0], r.value[0])];

        Tryte {
            value
        }
    }

    /// Binary bitwise or
    pub fn or(l: Tryte, r: Tryte) -> Tryte {
        let value = [
            Trit::or(l.value[0], r.value[0]), 
            Trit::or(l.value[0], r.value[0]), 
            Trit::or(l.value[0], r.value[0])];

        Tryte {
            value
        }
    }

    /// Unary left shift 
    pub fn lsh(t: Tryte) -> (Trit, Tryte) {
        return (t.value[2], Tryte::from_arr([Trit::Zero, t.value[0], t.value[1]]).unwrap());
    }

    /// Unary left shift 3 trytes
    pub fn lsh_3(t: [Tryte; 3]) -> (Trit, [Tryte; 3]) {
        let (carry_0, tryte_0) = Tryte::lsh(t[0]);
        let (carry_1, mut tryte_1) = Tryte::lsh(t[1]);
        tryte_1[0] = carry_0;
        let (carry_2, mut tryte_2) = Tryte::lsh(t[2]);
        tryte_2[0] = carry_1;

        return (carry_2, 
            [ tryte_0
            , tryte_1
            , tryte_2
            ]);
    }

    /// Unary left shift 9 trytes
    pub fn lsh_9(t: [Tryte; 9]) -> (Trit, [Tryte; 9]) {
        let (carry_0, tryte_0) = Tryte::lsh(t[0]);
        let (carry_1, mut tryte_1) = Tryte::lsh(t[1]);
        tryte_1[0] = carry_0;
        let (carry_2, mut tryte_2) = Tryte::lsh(t[2]);
        tryte_2[0] = carry_1;
        let (carry_3, mut tryte_3) = Tryte::lsh(t[1]);
        tryte_3[0] = carry_2;
        let (carry_4, mut tryte_4) = Tryte::lsh(t[2]);
        tryte_4[0] = carry_3;
        let (carry_5, mut tryte_5) = Tryte::lsh(t[1]);
        tryte_5[0] = carry_4;
        let (carry_6, mut tryte_6) = Tryte::lsh(t[2]);
        tryte_6[0] = carry_5;
        let (carry_7, mut tryte_7) = Tryte::lsh(t[1]);
        tryte_7[0] = carry_6;
        let (carry_8, mut tryte_8) = Tryte::lsh(t[2]);
        tryte_8[0] = carry_7;

        return (carry_8, 
            [ tryte_0
            , tryte_1
            , tryte_2
            , tryte_3
            , tryte_4
            , tryte_5
            , tryte_6
            , tryte_7
            , tryte_8
            ]);
    }

    /// Unary left shift n times
    pub fn lsh_n(t: Tryte, mut n: usize) -> (Trit, Tryte) {
        if n == 0 { return (Trit::Zero, t) }
        n -= 1;
        let (mut trit, mut tryte) = Tryte::lsh(t);
        for _ in 0..n {
            let (next_trit, next_tryte) = Tryte::lsh(tryte);
            trit = next_trit;
            tryte = next_tryte;
        }
        return (trit, tryte);
    }

    /// Unary left shift 3 trytes n times
    pub fn lsh_3_n(t: [Tryte; 3], mut n: usize) -> (Trit, [Tryte; 3]) {
        if n == 0 { return (Trit::Zero, t) }
        n -= 1;
        let (mut trit, mut trytes) = Tryte::lsh_3(t);
        for _ in 0..n {
            let (next_trit, next_tryte) = Tryte::lsh_3(trytes);
            trit = next_trit;
            trytes = next_tryte;
        }
        return (trit, trytes);
    }

    /// Unary left shift 9 trytes n times
    pub fn lsh_9_n(t: [Tryte; 9], mut n: usize) -> (Trit, [Tryte; 9]) {
        if n == 0 { return (Trit::Zero, t) }
        n -= 1;
        let (mut trit, mut trytes) = Tryte::lsh_9(t);
        for _ in 0..n {
            let (next_trit, next_tryte) = Tryte::lsh_9(trytes);
            trit = next_trit;
            trytes = next_tryte;
        }
        return (trit, trytes);
    }

    /// Unary right shift 
    pub fn rsh(t: Tryte) -> Tryte {
        return Tryte::from_arr([t.value[1], t.value[2], Trit::Zero]).unwrap();
    }

    /// Binary tryte add
    pub fn add(l: Tryte, r: Tryte) -> (Trit, Tryte) {
        let TritAddResult { carry, result: zero_trit } = Trit::add(l[0], r[0]);
        let TritAddResult { carry, result:  one_trit } = Trit::tri_add(carry, l[1], r[1]);
        let TritAddResult { carry, result:  two_trit } = Trit::tri_add(carry, l[2], r[2]);
        return (carry, Tryte::from_arr([zero_trit, one_trit, two_trit]).expect("Should be impossible"));
    }

    /// Binary 3 tryte add
    pub fn add_3(l: [Tryte; 3], r: [Tryte; 3]) -> (Trit, [Tryte; 3]) {
        let TritAddResult { carry, result: zero_trit } = Trit::add(l[0][0], r[0][0]);
        let TritAddResult { carry, result:  one_trit } = Trit::tri_add(carry, l[0][1], r[0][1]);
        let TritAddResult { carry, result:  two_trit } = Trit::tri_add(carry, l[0][2], r[0][2]);

        let TritAddResult { carry, result: thre_trit } = Trit::tri_add(carry, l[1][0], r[1][0]);
        let TritAddResult { carry, result: four_trit } = Trit::tri_add(carry, l[1][1], r[1][1]);
        let TritAddResult { carry, result: five_trit } = Trit::tri_add(carry, l[1][2], r[1][2]);

        let TritAddResult { carry, result:  six_trit } = Trit::tri_add(carry, l[2][0], r[2][0]);
        let TritAddResult { carry, result: sven_trit } = Trit::tri_add(carry, l[2][1], r[2][1]);
        let TritAddResult { carry, result: eigh_trit } = Trit::tri_add(carry, l[2][2], r[2][2]);
        return (carry, 
            [Tryte::from_arr([zero_trit, one_trit,   two_trit]).expect("Should be impossible"),
             Tryte::from_arr([thre_trit, four_trit, five_trit]).expect("Should be impossible"),
             Tryte::from_arr([six_trit,  sven_trit, eigh_trit]).expect("Should be impossible")
            ]);
    }

    /// Binary 9 tryte add
    pub fn add_9(l: [Tryte; 9], r: [Tryte; 9]) -> (Trit, [Tryte; 9]) {
        let TritAddResult { carry, result: _00_trit } = Trit::add(l[0][0], r[0][0]);
        let TritAddResult { carry, result: _01_trit } = Trit::tri_add(carry, l[0][1], r[0][1]);
        let TritAddResult { carry, result: _02_trit } = Trit::tri_add(carry, l[0][2], r[0][2]);
        let TritAddResult { carry, result: _10_trit } = Trit::tri_add(carry, l[1][0], r[1][0]);
        let TritAddResult { carry, result: _11_trit } = Trit::tri_add(carry, l[1][1], r[1][1]);
        let TritAddResult { carry, result: _12_trit } = Trit::tri_add(carry, l[1][2], r[1][2]);
        let TritAddResult { carry, result: _20_trit } = Trit::tri_add(carry, l[2][0], r[2][0]);
        let TritAddResult { carry, result: _21_trit } = Trit::tri_add(carry, l[2][1], r[2][1]);
        let TritAddResult { carry, result: _22_trit } = Trit::tri_add(carry, l[2][2], r[2][2]);

        let TritAddResult { carry, result: _30_trit } = Trit::tri_add(carry, l[3][0], r[3][0]);
        let TritAddResult { carry, result: _31_trit } = Trit::tri_add(carry, l[3][1], r[3][1]);
        let TritAddResult { carry, result: _32_trit } = Trit::tri_add(carry, l[3][2], r[3][2]);
        let TritAddResult { carry, result: _40_trit } = Trit::tri_add(carry, l[4][0], r[4][0]);
        let TritAddResult { carry, result: _41_trit } = Trit::tri_add(carry, l[4][1], r[4][1]);
        let TritAddResult { carry, result: _42_trit } = Trit::tri_add(carry, l[4][2], r[4][2]);
        let TritAddResult { carry, result: _50_trit } = Trit::tri_add(carry, l[5][0], r[5][0]);
        let TritAddResult { carry, result: _51_trit } = Trit::tri_add(carry, l[5][1], r[5][1]);
        let TritAddResult { carry, result: _52_trit } = Trit::tri_add(carry, l[5][2], r[5][2]);
                                                                                         
        let TritAddResult { carry, result: _60_trit } = Trit::tri_add(carry, l[6][0], r[6][0]);
        let TritAddResult { carry, result: _61_trit } = Trit::tri_add(carry, l[6][1], r[6][1]);
        let TritAddResult { carry, result: _62_trit } = Trit::tri_add(carry, l[6][2], r[6][2]);
        let TritAddResult { carry, result: _70_trit } = Trit::tri_add(carry, l[7][0], r[7][0]);
        let TritAddResult { carry, result: _71_trit } = Trit::tri_add(carry, l[7][1], r[7][1]);
        let TritAddResult { carry, result: _72_trit } = Trit::tri_add(carry, l[7][2], r[7][2]);
        let TritAddResult { carry, result: _80_trit } = Trit::tri_add(carry, l[8][0], r[8][0]);
        let TritAddResult { carry, result: _81_trit } = Trit::tri_add(carry, l[8][1], r[8][1]);
        let TritAddResult { carry, result: _82_trit } = Trit::tri_add(carry, l[8][2], r[8][2]);

        return (carry, 
            [Tryte::from_arr([_00_trit, _01_trit, _02_trit]).expect("Should be impossible"),
             Tryte::from_arr([_10_trit, _11_trit, _12_trit]).expect("Should be impossible"),
             Tryte::from_arr([_20_trit, _21_trit, _22_trit]).expect("Should be impossible"),
             Tryte::from_arr([_30_trit, _31_trit, _32_trit]).expect("Should be impossible"),
             Tryte::from_arr([_40_trit, _41_trit, _42_trit]).expect("Should be impossible"),
             Tryte::from_arr([_50_trit, _51_trit, _52_trit]).expect("Should be impossible"),
             Tryte::from_arr([_60_trit, _61_trit, _62_trit]).expect("Should be impossible"),
             Tryte::from_arr([_70_trit, _71_trit, _72_trit]).expect("Should be impossible"),
             Tryte::from_arr([_80_trit, _81_trit, _82_trit]).expect("Should be impossible")
            ]);
    }

    /// Binary tryte multiply
    pub fn mul(l: Tryte, r: Tryte) -> Tryte {
        let r0 = Tryte::and_trit(l, r[0]);
        let r1 = Tryte::and_trit(Tryte::lsh_n(l, 1).1, r[1]);
        let r2 = Tryte::and_trit(Tryte::lsh_n(l, 2).1, r[2]);

        let tmp = Tryte::add(r0, r1).1;
        let ret = Tryte::add(tmp, r2).1;
        return ret;
    }

    /// Binary 3 tryte multiply
    pub fn mul_3(l: [Tryte; 3], r: [Tryte; 3]) -> [Tryte; 3] {
        let r0 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 0).1, r[0][0]);
        let r1 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 1).1, r[0][1]);
        let r2 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 2).1, r[0][2]);
        let r3 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 3).1, r[1][0]);
        let r4 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 4).1, r[1][1]);
        let r5 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 5).1, r[1][2]);
        let r6 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 6).1, r[2][0]);
        let r7 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 7).1, r[2][1]);
        let r8 = Tryte::and_3_trit(Tryte::lsh_3_n(l, 8).1, r[2][2]);

        let (_, temp) = Tryte::add_3(r0,   r1);
        let (_, temp) = Tryte::add_3(temp, r2);
        let (_, temp) = Tryte::add_3(temp, r3);
        let (_, temp) = Tryte::add_3(temp, r4);
        let (_, temp) = Tryte::add_3(temp, r5);
        let (_, temp) = Tryte::add_3(temp, r6);
        let (_, temp) = Tryte::add_3(temp, r7);
        let (_,  res) = Tryte::add_3(temp, r8);

        return res;
    }

    /// Binary 9 tryte multiply
    pub fn mul_9(l: [Tryte; 9], r: [Tryte; 9]) -> [Tryte; 9] {
        let r00 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  0).1, r[0][0]);
        let r01 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  1).1, r[0][1]);
        let r02 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  2).1, r[0][2]);
        let r03 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  3).1, r[1][0]);
        let r04 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  4).1, r[1][1]);
        let r05 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  5).1, r[1][2]);
        let r06 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  6).1, r[2][0]);
        let r07 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  7).1, r[2][1]);
        let r08 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  8).1, r[2][2]);
              
        let r09 = Tryte::and_9_trit(Tryte::lsh_9_n(l,  9).1, r[3][0]);
        let r10 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 10).1, r[3][1]);
        let r11 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 11).1, r[3][2]);
        let r12 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 12).1, r[4][0]);
        let r13 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 13).1, r[4][1]);
        let r14 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 14).1, r[4][2]);
        let r15 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 15).1, r[5][0]);
        let r16 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 16).1, r[5][1]);
        let r17 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 17).1, r[5][2]);
               
        let r18 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 18).1, r[6][0]);
        let r19 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 19).1, r[6][1]);
        let r20 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 20).1, r[6][2]);
        let r21 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 21).1, r[7][0]);
        let r22 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 22).1, r[7][1]);
        let r23 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 23).1, r[7][2]);
        let r24 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 24).1, r[8][0]);
        let r25 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 25).1, r[8][1]);
        let r26 = Tryte::and_9_trit(Tryte::lsh_9_n(l, 26).1, r[8][2]);

        let (_, temp) = Tryte::add_9(r00,  r01);
        let (_, temp) = Tryte::add_9(temp, r02);
        let (_, temp) = Tryte::add_9(temp, r03);
        let (_, temp) = Tryte::add_9(temp, r04);
        let (_, temp) = Tryte::add_9(temp, r05);
        let (_, temp) = Tryte::add_9(temp, r06);
        let (_, temp) = Tryte::add_9(temp, r07);
        let (_, temp) = Tryte::add_9(temp, r08);

        let (_, temp) = Tryte::add_9(temp, r09);
        let (_, temp) = Tryte::add_9(temp, r10);
        let (_, temp) = Tryte::add_9(temp, r11);
        let (_, temp) = Tryte::add_9(temp, r12);
        let (_, temp) = Tryte::add_9(temp, r13);
        let (_, temp) = Tryte::add_9(temp, r14);
        let (_, temp) = Tryte::add_9(temp, r15);
        let (_, temp) = Tryte::add_9(temp, r16);
        let (_, temp) = Tryte::add_9(temp, r17);
                                              
        let (_, temp) = Tryte::add_9(temp, r18);
        let (_, temp) = Tryte::add_9(temp, r19);
        let (_, temp) = Tryte::add_9(temp, r20);
        let (_, temp) = Tryte::add_9(temp, r21);
        let (_, temp) = Tryte::add_9(temp, r22);
        let (_, temp) = Tryte::add_9(temp, r23);
        let (_, temp) = Tryte::add_9(temp, r24);
        let (_, temp) = Tryte::add_9(temp, r25);
        let (_,  res) = Tryte::add_9(temp, r26);

        return res;
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Trit, Tryte};

    #[test]
    fn test_trit_add() {
        let one_0 = Trit::POne;
        let one_1 = Trit::POne;
        println!("First {:?}", Trit::add(one_1, one_0));
    }

    #[test]
    fn test_triple_trit_add() {
        let pone = Trit::POne;
        let zero = Trit::Zero;
        let none = Trit::NOne;

        println!("{:?}", Trit::tri_add(pone, pone, pone));
        println!("{:?}", Trit::tri_add(pone, pone, zero));
        println!("{:?}", Trit::tri_add(none, none, zero));
        println!("{:?}", Trit::tri_add(none, none, none));
        println!("{:?}", Trit::tri_add(pone, none, zero));
    }

    #[test]
    fn mul() {
        let three = Tryte::from_arr([Trit::Zero, Trit::POne, Trit::Zero]).unwrap();

        println!("Mul {:?}", Tryte::mul(three, three));
    }
}
