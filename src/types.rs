use crate::errors::TritParseErr;

/// A Trit represents a balanced ternary trit, which
/// is 0, 1, or -1. The default is 0
#[derive(Debug, Default, Clone, Copy)]
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
    pub fn from_arr<T: Into<i64>>(input: [T; 3]) -> Result<Self, TritParseErr> {
        Ok(Tryte { 
            value: input.try_map(|i| Trit::from_num(i))? 
        })
    }
}


