use std::ops::{Add, BitAnd, BitOr, Mul, Neg, Not};

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Trit {
    NOne = 0b00,
    #[default]
    Zero = 0b01,
    POne = 0b10,
}

impl Trit {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(val: u8) -> Trit {
        match val {
            0b00 => Trit::NOne,
            0b10 => Trit::POne,
            _    => Trit::Zero,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct TritAddResult {
    pub carry: Trit,
    pub result: Trit,
}

impl BitAnd<Trit> for Trit {
    type Output = Trit;
    fn bitand(self, rhs: Trit) -> Self::Output {
        Trit::min(self, rhs)
    }
}

impl BitOr<Trit> for Trit {
    type Output = Trit;
    fn bitor(self, rhs: Trit) -> Self::Output {
        Trit::max(self, rhs)
    }
}

impl Add<Trit> for Trit {
    type Output = TritAddResult;
    fn add(self, rhs: Trit) -> Self::Output {
        let result = self.tritwise_add(rhs);
        let carry  = self.tritwise_add_carry(rhs);
        TritAddResult { carry, result }
    }
}

/// ONLY TO MAKE ADDING 3 TRITS ERGONOMICAL ONLY
impl Add<Trit> for TritAddResult {
    type Output = TritAddResult;
    fn add(self, rhs: Trit) -> Self::Output {
        let tmp = self.result + rhs;

        TritAddResult {
            result: tmp.result,
            carry:  (tmp.carry + self.carry).result
        }
    }
}

impl Neg for Trit {
    type Output = Trit;
    fn neg(self) -> Self::Output {
        match self {
            Trit::Zero => Trit::Zero,
            Trit::NOne => Trit::POne,
            Trit::POne => Trit::NOne,
        }
    }
}

impl Not for Trit {
    type Output = Trit;
    fn not(self) -> Self::Output {
        -self
    }
}

impl Mul<Trit> for Trit {
    type Output = Trit;
    fn mul(self, rhs: Trit) -> Self::Output {
        match self {
            Trit::Zero => Trit::Zero,
            Trit::NOne => match rhs {
                Trit::Zero => Trit::Zero,
                Trit::NOne => Trit::POne,
                Trit::POne => Trit::NOne,
            },
            Trit::POne => match rhs {
                Trit::Zero => Trit::Zero,
                Trit::NOne => Trit::NOne,
                Trit::POne => Trit::POne,
            },
        }
    }
}

impl Trit {
    fn tritwise_add(self, rhs: Trit) -> Trit {
        match self {
            Trit::Zero => rhs,
            Trit::POne => match rhs {
                Trit::NOne => Trit::Zero,
                Trit::Zero => Trit::POne,
                Trit::POne => Trit::NOne,
            },
            Trit::NOne => match rhs {
                Trit::NOne => Trit::POne,
                Trit::Zero => Trit::NOne,
                Trit::POne => Trit::Zero,
            },
        }
    }

    fn tritwise_add_carry(self, rhs: Trit) -> Trit {
        match self {
            Trit::Zero => Trit::Zero,
            Trit::NOne => match rhs {
                Trit::Zero => Trit::Zero,
                Trit::NOne => Trit::NOne,
                Trit::POne => Trit::Zero,
            },
            Trit::POne => match rhs {
                Trit::Zero => Trit::Zero,
                Trit::NOne => Trit::Zero,
                Trit::POne => Trit::POne,
            },
        }
    }

    fn cycle_1_0(self) -> Trit {
        match self {
            Trit::NOne => Trit::NOne,
            Trit::Zero => Trit::POne,
            Trit::POne => Trit::Zero,
        }
    }

    fn cycle_n1_0(self) -> Trit {
        match self {
            Trit::NOne => Trit::Zero,
            Trit::Zero => Trit::NOne,
            Trit::POne => Trit::POne,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::trits::{Trit, TritAddResult};

    #[test]
    fn order_min_max_eq_test() {
        assert_eq!(Trit::POne > Trit::NOne, Trit::POne > Trit::NOne);
        assert_eq!(Trit::POne < Trit::NOne, Trit::POne < Trit::NOne);
        assert_eq!(Trit::POne > Trit::Zero, Trit::POne > Trit::Zero);
        assert_eq!(Trit::POne < Trit::Zero, Trit::POne < Trit::Zero);
        assert_eq!(Trit::NOne > Trit::Zero, Trit::NOne > Trit::Zero);
        assert_eq!(Trit::NOne < Trit::Zero, Trit::NOne < Trit::Zero);

        assert_eq!(Trit::NOne == Trit::Zero, Trit::NOne == Trit::Zero);
        assert_eq!(Trit::Zero == Trit::Zero, Trit::Zero == Trit::Zero);
        assert_eq!(Trit::POne == Trit::Zero, Trit::POne == Trit::Zero);

        assert_eq!(Trit::NOne == Trit::POne, Trit::NOne == Trit::POne);
        assert_eq!(Trit::Zero == Trit::POne, Trit::Zero == Trit::POne);
        assert_eq!(Trit::POne == Trit::POne, Trit::POne == Trit::POne);

        assert_eq!(Trit::NOne == Trit::NOne, Trit::NOne == Trit::NOne);
        assert_eq!(Trit::Zero == Trit::NOne, Trit::Zero == Trit::NOne);
        assert_eq!(Trit::POne == Trit::NOne, Trit::POne == Trit::NOne);

        assert_eq!(Trit::max(Trit::NOne, Trit::NOne), Trit::NOne);
        assert_eq!(Trit::max(Trit::NOne, Trit::Zero), Trit::Zero);
        assert_eq!(Trit::max(Trit::Zero, Trit::POne), Trit::POne);

        assert_eq!(Trit::min(Trit::NOne, Trit::Zero), Trit::NOne);
        assert_eq!(Trit::min(Trit::Zero, Trit::POne), Trit::Zero);
        assert_eq!(Trit::min(Trit::POne, Trit::POne), Trit::POne);
    }

    #[test]
    fn add_mul_test() {
        let n_one = Trit::NOne;
        let zero  = Trit::Zero;
        let p_one = Trit::POne;

        assert_eq!((zero + n_one).result, n_one);
        assert_eq!((zero + zero ).result,  zero);
        assert_eq!((zero + p_one).result, p_one);

        assert_eq!((zero + n_one).carry, zero);
        assert_eq!((zero + zero ).carry, zero);
        assert_eq!((zero + p_one).carry, zero);

        assert_eq!((p_one + p_one), TritAddResult { carry: Trit::POne, result: Trit::NOne });
        assert_eq!((n_one + n_one), TritAddResult { carry: Trit::NOne, result: Trit::POne });

        assert_eq!(zero * n_one, zero);
        assert_eq!(zero * zero , zero);
        assert_eq!(zero * p_one, zero);

        assert_eq!(n_one * n_one, p_one);
        assert_eq!(n_one * zero ,  zero);
        assert_eq!(n_one * p_one, n_one);

        assert_eq!(p_one * p_one, p_one);
    }

    #[test]
    fn add_3_trits() {
        let n_one = Trit::NOne;
        let zero  = Trit::Zero;
        let p_one = Trit::POne;

        let thing_0 = p_one + p_one + p_one;
        let thing_1 = n_one + zero  + n_one;
        println!("{:?}", thing_0);
        println!("{:?}", thing_1);
    }
}
