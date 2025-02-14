use crate::errors::DivByZeroError;

use super::*;
use std::ops::{Add, Div, Mul, Rem, Sub};

//== Euc Div Result ==//

#[derive(Debug)]
pub struct VarEuclideanDivisionResult<const N: usize> {
    quotient: Varsize<N>,
    remainder: Varsize<N>,
}

//== Binary Ops ==//

impl<const N: usize> Add for Varsize<N> {
    type Output = VarAddResult<N>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Varsize::default();

        let mut result: TritAddResult = self[0] + rhs[0];
        output[0] = result.result;

        for i in 1..self.len() {
            result = self[i] + rhs[i] + result.carry;
            output[i] = result.result;
        }

        VarAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl<const N: usize> Add<Trit> for Varsize<N> {
    type Output = VarAddResult<N>;

    fn add(self, rhs: Trit) -> Self::Output {
        let mut output = Varsize::default();

        let mut result: TritAddResult = self[0] + rhs;
        output[0] = result.result;

        for i in 1..27 {
            result = self[i] + result.carry;
            output[i] = result.result;
        }

        VarAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl<const N: usize> Sub for Varsize<N> {
    type Output = VarAddResult<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = -rhs;
        let mut output = Varsize::default();

        let mut result: TritAddResult = self[0] + rhs[0];
        output[0] = result.result;

        for i in 1..27 {
            result = self[i] + rhs[i] + result.carry;
            output[i] = result.result;
        }

        VarAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl<const N: usize> Sub<Trit> for Varsize<N> {
    type Output = VarAddResult<N>;

    fn sub(self, rhs: Trit) -> Self::Output {
        let mut output = Varsize::default();

        let mut result: TritAddResult = self[0] + -rhs;
        output[0] = result.result;

        for i in 1..27 {
            result = self[i] + result.carry;
            output[i] = result.result;
        }

        VarAddResult {
            result: output,
            carry: result.carry,
        }
    }
}

impl<const N: usize> Mul for Varsize<N> {
    type Output = Varsize<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        rhs.iter()
            .enumerate()
            .map(|(i, trit)| (self * trit) << i)
            .reduce(|acc, right| (acc + right).result)
            .unwrap()
    }
}

impl<const N: usize> Mul<Trit> for Varsize<N> {
    type Output = Varsize<N>;

    fn mul(self, rhs: Trit) -> Self::Output {
        self.map(|x| x * rhs).into()
    }
}

impl<const N: usize> Mul<&Trit> for Varsize<N> {
    type Output = Varsize<N>;

    fn mul(self, rhs: &Trit) -> Self::Output {
        self.map(|x| x * (*rhs)).into()
    }
}

impl<const N: usize> Div for Varsize<N> {
    type Output = Result<Varsize<N>, DivByZeroError>;

    fn div(self, rhs: Self) -> Self::Output {
        self.euclidean_division(rhs).map(|res| res.quotient)
    }
}

impl<const N: usize> Rem for Varsize<N> {
    type Output = Result<Varsize<N>, DivByZeroError>;
    fn rem(self, rhs: Self) -> Self::Output {
        self.euclidean_division(rhs).map(|res| res.remainder)
    }
}

impl<const N: usize> Varsize<N> {
    fn euclidean_division(self, rhs: Self) -> Result<VarEuclideanDivisionResult<N>, DivByZeroError> {
        if rhs == Varsize::default() {
            return Err(DivByZeroError);
        }

        let len = self.len();
        let b = rhs;
        let b_sign = match b.cmp(&Varsize::default()) {
            std::cmp::Ordering::Less => Trit::NOne,
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Greater => Trit::POne,
        };

        let mut res = (1..=len).fold(
            VarEuclideanDivisionResult {
                quotient: Varsize::default(),
                remainder: self,
            },
            |acc: VarEuclideanDivisionResult<N>, i| {
                let k_i = acc.remainder >> (len - i);
                let q_n = if Varsize::abs(b) > Varsize::abs(k_i) {
                    Trit::Zero
                } else {
                    let rem_sign = match acc.remainder.cmp(&Varsize::default()) {
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

                VarEuclideanDivisionResult {
                    quotient,
                    remainder: (acc.remainder - l_n).result,
                }
            },
        );

        let direction = b > Varsize::default();

        while res.remainder < Varsize::default() {
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

#[cfg(test)]
pub mod test {
    use super::{Trit, Varsize};

    #[test]
    fn test_mul() {
        let ones: Varsize<8> = [Trit::POne; 8].into();
        let none: Varsize<8> = ones * Trit::NOne;
        let zero: Varsize<8> = none * Trit::Zero;

        let mut three: Varsize<8> = zero;
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

        let seventy: Varsize<8> = [
            Trit::POne,
            Trit::NOne,
            Trit::NOne,
            Trit::Zero,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();

        let four: Varsize<8> = [
            Trit::POne,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();

        let two_hundred_eighty: Varsize<8> = [
            Trit::POne,
            Trit::Zero,
            Trit::POne,
            Trit::POne,
            Trit::Zero,
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
        ]
        .into();

        assert_eq!(four * seventy, two_hundred_eighty);
    }

    #[test]
    fn test_euc_div() {
        let six = Varsize::from([
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

        let mut three = Varsize::default();
        three[1] = Trit::POne;

        let mut four = Varsize::default();
        four[1] = Trit::POne;
        four[0] = Trit::POne;

        let nineteen = Varsize::from([
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

        let quod = (nineteen / four).unwrap();
        let rem = (nineteen % four).unwrap();
        assert_eq!(quod, four);
        assert_eq!(rem, three);
        assert_eq!(((four * quod) + rem).result, nineteen);

        let nsix = -six;
        let sixty_two = Varsize::from([
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

        let nten = Varsize::from([
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

        let neleven = Varsize::from([
            Trit::POne,
            Trit::NOne,
            Trit::NOne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]);

        let one = Varsize::from([
            Trit::POne,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
            Trit::Zero,
        ]);

        let two = Varsize::from([
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

        let quod = (sixty_two / nsix).unwrap();
        let rem = (sixty_two % nsix).unwrap();
        assert_eq!(quod, nten);
        assert_eq!(rem, two);
        assert_eq!(((nsix * quod) + rem).result, sixty_two);

        let quod = ((-sixty_two) / six).unwrap();
        let rem  = ((-sixty_two) % six).unwrap();
        assert_eq!(quod, neleven);
        assert_eq!(rem, four);
        assert_eq!(((six * quod) + rem).result, -sixty_two);

        let quod = (-nineteen / -four).unwrap();
        let rem = (-nineteen % -four).unwrap();
        assert_eq!(quod, (four + Trit::POne).result);
        assert_eq!(rem, one);
        assert_eq!(((-four * quod) + rem).result, -nineteen);
    }
}
