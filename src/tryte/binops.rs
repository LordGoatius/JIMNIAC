use super::*;
use std::ops::{Add, Mul, Sub};

//== Binary Ops ==//

impl Add for Tryte {
    type Output = TryteAddResult;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Tryte::default();

        let mut result: TritAddResult = self[0] + rhs[0];
        output[0] = result.result;

        for i in 1..self.len() {
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

impl Sub for Tryte {
    type Output = TryteAddResult;
    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = -rhs;
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

impl Sub<Trit> for Tryte {
    type Output = TryteAddResult;

    fn sub(self, rhs: Trit) -> Self::Output {
        let mut output = Tryte::default();

        let mut result: TritAddResult = self[0] + -rhs;
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

impl Mul for Tryte {
    type Output = Tryte;

    fn mul(self, rhs: Self) -> Self::Output {
        rhs.iter()
            .enumerate()
            .map(|(i, trit)| (self * trit) << i)
            .reduce(|acc, right| (acc + right).result)
            .unwrap()
    }
}

impl Mul<Trit> for Tryte {
    type Output = Tryte;

    fn mul(self, rhs: Trit) -> Self::Output {
        self.map(|x| x * rhs).into()
    }
}

impl Mul<&Trit> for Tryte {
    type Output = Tryte;

    fn mul(self, rhs: &Trit) -> Self::Output {
        self.map(|x| x * (*rhs)).into()
    }
}

#[cfg(test)]
pub mod test {
    use super::{Trit, Tryte};

    #[test]
    fn test_mul() {
        let ones: Tryte = [Trit::POne; 9].into();
        let none: Tryte = ones * Trit::NOne;
        let zero: Tryte = none * Trit::Zero;

        let mut three: Tryte = zero;
        three[1] = Trit::POne;

        assert_eq!(ones << 1, ones * three);
        assert_eq!(ones << 2, ones * three * three);
        assert_eq!(ones << 3, ones * three * three * three);

        assert_eq!(none << 1, none * three);
        assert_eq!(none << 2, none * three * three);
        assert_eq!(none << 3, none * three * three * three);

        let mut two = three;
        two[0] = Trit::POne;

        assert_eq!(zero * two, zero);

        let seventy: Tryte = 
            [Trit::POne, Trit::NOne, Trit::NOne, 
             Trit::Zero, Trit::POne, Trit::Zero, 
             Trit::Zero, Trit::Zero, Trit::Zero].into();

        let four: Tryte =
            [Trit::POne, Trit::POne, Trit::Zero, 
             Trit::Zero, Trit::Zero, Trit::Zero, 
             Trit::Zero, Trit::Zero, Trit::Zero].into();

        let two_hundred_eighty: Tryte =
            [Trit::POne, Trit::Zero, Trit::POne, 
             Trit::POne, Trit::Zero, Trit::POne, 
             Trit::Zero, Trit::Zero, Trit::Zero].into();

        assert_eq!(seventy * four, two_hundred_eighty);
    }
}
