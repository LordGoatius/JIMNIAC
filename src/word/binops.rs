use super::*;
use std::ops::{Add, Mul, Sub};

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
