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

/// A tryte is the smallest possible addressable unit.
/// It's made up of 3 trits (eventuallly may make it generic over size, for 
/// different machine architectures). 
#[derive(Debug, Default, Clone, Copy)]
pub struct Tryte {
    pub value: [Trit; 3]
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
    pub fn add(l: Trit, r: Trit) -> [Trit; 2] {
        let mut ret = [Trit::Zero, Trit::and(l, r)];
        if (l == Trit::NOne) && (r == Trit::NOne) {
            ret[0] = Trit::NOne;
        } else if (l == Trit::POne) && (r == Trit::POne) {
            ret[0] = Trit::POne;
        }
        ret
    }

    /// Trinary add
    pub fn tri_add(l: Trit, m: Trit, r: Trit) -> [Trit; 2] {
        if l == Trit::Zero {
            return Trit::add(m, r);
        } else if m == Trit::Zero {
            return Trit::add(l, r);
        } else if r == Trit::Zero {
            return Trit::add(l, m);
        }

        todo!()
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

    /// Binary tryte add
    pub fn add(l: Tryte, r: Tryte) -> (Trit, Tryte) {
        let [carry, first]   = Trit::add(l.value[0], r.value[0]);
        let [carry, second]  = Trit()
        todo!()
    }
}
