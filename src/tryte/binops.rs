use super::*;
use std::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};

//== Euc Div Result ==//

pub struct EuclideanDivisionResult {
    quotient: Tryte,
    remainder: Tryte,
}

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

impl Div for Tryte {
    type Output = Tryte;

    fn div(self, rhs: Self) -> Self::Output {
        self.euclidean_division(rhs).quotient
    }
}

impl Rem for Tryte {
    type Output = Tryte;
    fn rem(self, rhs: Self) -> Self::Output {
        self.euclidean_division(rhs).remainder
    }
}

impl Tryte {
    fn euclidean_division(self, rhs: Self) -> EuclideanDivisionResult {
        if rhs == Tryte::default() {
            // FIXME: change to result to unwrap so cpu can div by zero interrupt handle
            panic!("Division by zero is not allowed");
        }

        // Self is a, rhs is b in a / b, a = qb + r

        let len = self.len();
        let b = rhs;

        (1..=len).fold(
            EuclideanDivisionResult {
                quotient: Tryte::default(),
                remainder: self,
            },
            |acc: EuclideanDivisionResult, i| {
                let k_i = acc.remainder >> len - i;
                let q_n = if Tryte::abs(b) > Tryte::abs(k_i) {
                    Trit::Zero
                } else {
                    // TODO: fix this
                    if b > k_i {
                        Trit::NOne
                    } else {
                        Trit::POne
                    }
                };
                let l_n = (b << len - i) * q_n;
                let mut quotient = acc.quotient;
                quotient[len - i] = q_n;

                EuclideanDivisionResult {
                    quotient,
                    remainder: (acc.remainder - l_n).result,
                }
            },
        )
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

        let seventy: Tryte = [
            Trit::POne,
            Trit::NOne,
            Trit::NOne,
            Trit::Zero,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();

        let four: Tryte = [
            Trit::POne,
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

        let two_hundred_eighty: Tryte = [
            Trit::POne,
            Trit::Zero,
            Trit::POne,
            Trit::POne,
            Trit::Zero,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();
    }

    #[test]
    fn test_euc_div() {
        let six = Tryte::from([
            Trit::Zero,
            Trit::NOne,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]);

        let mut three = Tryte::default();
        three[1] = Trit::POne;

        let mut four = Tryte::default();
        four[1] = Trit::POne;
        four[0] = Trit::POne;

        let nineteen = Tryte::from([
            Trit::POne,
            Trit::Zero,
            Trit::NOne,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]);

        let quod = nineteen / four;
        let rem = nineteen % four;
        assert_eq!(quod, four);
        assert_eq!(rem, three);
        assert_eq!(((four * quod) + rem).result, nineteen);

        let nsix = -six;
        let sixty_two = Tryte::from([
            Trit::NOne,
            Trit::Zero,
            Trit::POne,
            Trit::NOne,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]);

        let nten = Tryte::from([
            Trit::NOne,
            Trit::Zero,
            Trit::NOne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]);

        let two = Tryte::from([
            Trit::NOne,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]);

        let quod = sixty_two / nsix;
        let rem = sixty_two % nsix;
        println!("{quod:?}");
        println!("{rem:?}");
        assert_eq!(quod, nten);
        assert_eq!(rem, two);
        assert_eq!(((nsix * quod) + rem).result, sixty_two);

    }
}
