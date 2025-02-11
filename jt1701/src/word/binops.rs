use crate::cpu::errors::CpuError;

use super::*;
use std::ops::{Add, Div, Mul, Rem, Sub};

//== Euc Div Result ==//

#[derive(Debug)]
pub struct EuclideanDivisionResult {
    quotient: Word,
    remainder: Word,
}

//== Binary Ops ==//

impl Add for Word {
    type Output = WordAddResult;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Word::default();

        let mut result: TritAddResult = self[0] + rhs[0];
        output[0] = result.result;

        for i in 1..self.len() {
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

impl Add<Tryte> for Word {
    type Output = WordAddResult;

    fn add(self, rhs: Tryte) -> Self::Output {
        let word: Word = rhs.into();
        self + word
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

impl Mul for Word {
    type Output = Word;

    fn mul(self, rhs: Self) -> Self::Output {
        rhs.iter()
            .enumerate()
            .map(|(i, trit)| (self * trit) << i)
            .reduce(|acc, right| (acc + right).result)
            .unwrap()
    }
}

impl Mul<Trit> for Word {
    type Output = Word;

    fn mul(self, rhs: Trit) -> Self::Output {
        self.map(|x| x * rhs).into()
    }
}

impl Mul<&Trit> for Word {
    type Output = Word;

    fn mul(self, rhs: &Trit) -> Self::Output {
        self.map(|x| x * (*rhs)).into()
    }
}

impl Div for Word {
    type Output = Result<Word, CpuError>;

    fn div(self, rhs: Self) -> Self::Output {
        self.euclidean_division(rhs).map(|res| res.quotient)
    }
}

impl Rem for Word {
    type Output = Result<Word, CpuError>;
    fn rem(self, rhs: Self) -> Self::Output {
        self.euclidean_division(rhs).map(|res| res.remainder)
    }
}

impl Word {
    fn euclidean_division(self, rhs: Self) -> Result<EuclideanDivisionResult, CpuError> {
        if rhs == Word::default() {
            return Err(CpuError::DivByZero);
        }

        let len = self.len();
        let b = rhs;
        let b_sign = match b.cmp(&Word::default()) {
            std::cmp::Ordering::Less => Trit::NOne,
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Greater => Trit::POne,
        };

        let mut res = (1..=len).fold(
            EuclideanDivisionResult {
                quotient: Word::default(),
                remainder: self,
            },
            |acc: EuclideanDivisionResult, i| {
                let k_i = acc.remainder >> (len - i);
                let q_n = if Word::abs(b) > Word::abs(k_i) {
                    Trit::Zero
                } else {
                    let rem_sign = match acc.remainder.cmp(&Word::default()) {
                        std::cmp::Ordering::Less => Trit::NOne,
                        std::cmp::Ordering::Equal => Trit::Zero,
                        std::cmp::Ordering::Greater => Trit::POne,
                    };
                    match rem_sign {
                        Trit::NOne => -b_sign,
                        Trit::POne => b_sign,
                        Trit::Zero => Trit::Zero,
                    }
                };
                let l_n = (b << (len - i)) * q_n;
                let mut quotient = acc.quotient;
                quotient[len - i] = q_n;

                EuclideanDivisionResult {
                    quotient,
                    remainder: (acc.remainder - l_n).result,
                }
            },
        );

        let direction = b > Word::default();

        while res.remainder < Word::default() {
            if direction {
                res.quotient = (res.quotient - Trit::POne).result;
                res.remainder = (res.remainder + b).result;
            } else {
                res.quotient = (res.quotient + Trit::POne).result;
                res.remainder = (res.remainder - b).result;
            }
        }

        Ok(res)
    }
}
