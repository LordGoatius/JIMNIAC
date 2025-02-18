use std::{
    fmt::Debug,
    ops::{Add, Deref, DerefMut, Index, IndexMut},
};

use crate::GetStatus;
use septivigntimal::*;
use ternary::{
    errors::DivByZeroError,
    trits::Trit,
    tryte::{Tryte, TryteAddResult},
    word::{Word, WordAddResult},
};

use itertools::{
    Either,
    Either::{Left, Right},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordOrTryte {
    Word,
    Tryte,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterNumber {
    RN13,
    RN12,
    RN11,
    RN10,
    RN9,
    RN8,
    RN7,
    RN6,
    RN5,
    RN4,
    RN3,
    RN2,
    RN1,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    /// R12
    SP, // => R12
    /// R13
    BP, // => R13
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register {
    pub(crate) num: RegisterNumber,
    pub(crate) size: WordOrTryte,
}

pub const BP_WORD: Register = Register {
    num: RegisterNumber::BP,
    size: WordOrTryte::Word,
};

pub const SP_WORD: Register = Register {
    num: RegisterNumber::SP,
    size: WordOrTryte::Word,
};

pub const BP_TRYTE: Register = Register {
    num: RegisterNumber::BP,
    size: WordOrTryte::Tryte,
};

pub const SP_TRYTE: Register = Register {
    num: RegisterNumber::SP,
    size: WordOrTryte::Tryte,
};

impl From<Register> for Trit {
    fn from(value: Register) -> Self {
        match value.size {
            WordOrTryte::Tryte => Trit::NOne,
            WordOrTryte::Word => Trit::Zero,
        }
    }
}

impl From<Register> for [Trit; 3] {
    fn from(value: Register) -> Self {
        match value.num {
            RegisterNumber::RN13 => Z,
            RegisterNumber::RN12 => Y,
            RegisterNumber::RN11 => X,
            RegisterNumber::RN10 => W,
            RegisterNumber::RN9 => V,
            RegisterNumber::RN8 => U,
            RegisterNumber::RN7 => T,
            RegisterNumber::RN6 => S,
            RegisterNumber::RN5 => R,
            RegisterNumber::RN4 => Q,
            RegisterNumber::RN3 => P,
            RegisterNumber::RN2 => O,
            RegisterNumber::RN1 => N,
            RegisterNumber::R0 => ZERO,
            RegisterNumber::R1 => A,
            RegisterNumber::R2 => B,
            RegisterNumber::R3 => C,
            RegisterNumber::R4 => D,
            RegisterNumber::R5 => E,
            RegisterNumber::R6 => F,
            RegisterNumber::R7 => G,
            RegisterNumber::R8 => H,
            RegisterNumber::R9 => I,
            RegisterNumber::R10 => J,
            RegisterNumber::R11 => K,
            RegisterNumber::R12 => L,
            RegisterNumber::R13 => M,
            RegisterNumber::SP => L,
            RegisterNumber::BP => M,
        }
    }
}

impl From<Register> for (Trit, [Trit; 3]) {
    fn from(value: Register) -> Self {
        let trit = match value.size {
            WordOrTryte::Tryte => Trit::NOne,
            WordOrTryte::Word => Trit::Zero,
        };

        let arr = match value.num {
            RegisterNumber::RN13 => Z,
            RegisterNumber::RN12 => Y,
            RegisterNumber::RN11 => X,
            RegisterNumber::RN10 => W,
            RegisterNumber::RN9 => V,
            RegisterNumber::RN8 => U,
            RegisterNumber::RN7 => T,
            RegisterNumber::RN6 => S,
            RegisterNumber::RN5 => R,
            RegisterNumber::RN4 => Q,
            RegisterNumber::RN3 => P,
            RegisterNumber::RN2 => O,
            RegisterNumber::RN1 => N,
            RegisterNumber::R0 => ZERO,
            RegisterNumber::R1 => A,
            RegisterNumber::R2 => B,
            RegisterNumber::R3 => C,
            RegisterNumber::R4 => D,
            RegisterNumber::R5 => E,
            RegisterNumber::R6 => F,
            RegisterNumber::R7 => G,
            RegisterNumber::R8 => H,
            RegisterNumber::R9 => I,
            RegisterNumber::R10 => J,
            RegisterNumber::R11 => K,
            RegisterNumber::R12 => L,
            RegisterNumber::R13 => M,
            RegisterNumber::SP => L,
            RegisterNumber::BP => M,
        };

        (trit, arr)
    }
}

impl From<(Trit, [Trit; 3])> for Register {
    /// size:
    ///     NOne => Tryte
    ///     Zero => Word
    fn from(value: (Trit, [Trit; 3])) -> Self {
        let size = match value.0 {
            Trit::NOne => WordOrTryte::Tryte,
            Trit::Zero => WordOrTryte::Word,
            _ => panic!(),
        };

        let num = match value.1 {
            Z => RegisterNumber::RN13,
            Y => RegisterNumber::RN12,
            X => RegisterNumber::RN11,
            W => RegisterNumber::RN10,
            V => RegisterNumber::RN9,
            U => RegisterNumber::RN8,
            T => RegisterNumber::RN7,
            S => RegisterNumber::RN6,
            R => RegisterNumber::RN5,
            Q => RegisterNumber::RN4,
            P => RegisterNumber::RN3,
            O => RegisterNumber::RN2,
            N => RegisterNumber::RN1,
            ZERO => RegisterNumber::R0,
            A => RegisterNumber::R1,
            B => RegisterNumber::R2,
            C => RegisterNumber::R3,
            D => RegisterNumber::R4,
            E => RegisterNumber::R5,
            F => RegisterNumber::R6,
            G => RegisterNumber::R7,
            H => RegisterNumber::R8,
            I => RegisterNumber::R9,
            J => RegisterNumber::R10,
            K => RegisterNumber::R11,
            L => RegisterNumber::R12,
            M => RegisterNumber::R13,
        };

        Register { num, size }
    }
}

#[derive(Debug, Default)]
pub(super) struct RegisterFile {
    rn15: Word,
    rn14: Word,
    rn13: Word,
    rn12: Word,
    rn11: Word,
    rn10: Word,
    rn9: Word,
    rn8: Word,
    rn7: Word,
    rn6: Word,
    rn5: Word,
    rn4: Word,
    rn3: Word,
    rn2: Word,
    rn1: Word,
    r0: Word,
    r1: Word,
    r2: Word,
    r3: Word,
    r4: Word,
    r5: Word,
    r6: Word,
    r7: Word,
    r8: Word,
    r9: Word,
    r10: Word,
    r11: Word,
    r12: Word,
    r13: Word,
    r14: Word,
    r15: Word,
}

impl Index<RegisterNumber> for RegisterFile {
    type Output = Word;

    fn index(&self, index: RegisterNumber) -> &Self::Output {
        match index {
            RegisterNumber::RN13 => &self.rn13,
            RegisterNumber::RN12 => &self.rn12,
            RegisterNumber::RN11 => &self.rn11,
            RegisterNumber::RN10 => &self.rn10,
            RegisterNumber::RN9 => &self.rn9,
            RegisterNumber::RN8 => &self.rn8,
            RegisterNumber::RN7 => &self.rn7,
            RegisterNumber::RN6 => &self.rn6,
            RegisterNumber::RN5 => &self.rn5,
            RegisterNumber::RN4 => &self.rn4,
            RegisterNumber::RN3 => &self.rn3,
            RegisterNumber::RN2 => &self.rn2,
            RegisterNumber::RN1 => &self.rn1,
            RegisterNumber::R0 => &self.r0,
            RegisterNumber::R1 => &self.r1,
            RegisterNumber::R2 => &self.r2,
            RegisterNumber::R3 => &self.r3,
            RegisterNumber::R4 => &self.r4,
            RegisterNumber::R5 => &self.r5,
            RegisterNumber::R6 => &self.r6,
            RegisterNumber::R7 => &self.r7,
            RegisterNumber::R8 => &self.r8,
            RegisterNumber::R9 => &self.r9,
            RegisterNumber::R10 => &self.r10,
            RegisterNumber::R11 => &self.r11,
            RegisterNumber::R12 => &self.r12,
            RegisterNumber::R13 => &self.r13,
            RegisterNumber::SP => &self.r12,
            RegisterNumber::BP => &self.r13,
        }
    }
}

impl IndexMut<RegisterNumber> for RegisterFile {
    fn index_mut(&mut self, index: RegisterNumber) -> &mut Self::Output {
        match index {
            RegisterNumber::RN13 => &mut self.rn13,
            RegisterNumber::RN12 => &mut self.rn12,
            RegisterNumber::RN11 => &mut self.rn11,
            RegisterNumber::RN10 => &mut self.rn10,
            RegisterNumber::RN9 => &mut self.rn9,
            RegisterNumber::RN8 => &mut self.rn8,
            RegisterNumber::RN7 => &mut self.rn7,
            RegisterNumber::RN6 => &mut self.rn6,
            RegisterNumber::RN5 => &mut self.rn5,
            RegisterNumber::RN4 => &mut self.rn4,
            RegisterNumber::RN3 => &mut self.rn3,
            RegisterNumber::RN2 => &mut self.rn2,
            RegisterNumber::RN1 => &mut self.rn1,
            RegisterNumber::R0 => &mut self.r0,
            RegisterNumber::R1 => &mut self.r1,
            RegisterNumber::R2 => &mut self.r2,
            RegisterNumber::R3 => &mut self.r3,
            RegisterNumber::R4 => &mut self.r4,
            RegisterNumber::R5 => &mut self.r5,
            RegisterNumber::R6 => &mut self.r6,
            RegisterNumber::R7 => &mut self.r7,
            RegisterNumber::R8 => &mut self.r8,
            RegisterNumber::R9 => &mut self.r9,
            RegisterNumber::R10 => &mut self.r10,
            RegisterNumber::R11 => &mut self.r11,
            RegisterNumber::R12 => &mut self.r12,
            RegisterNumber::R13 => &mut self.r13,
            RegisterNumber::SP => &mut self.r12,
            RegisterNumber::BP => &mut self.r13,
        }
    }
}

impl RegisterFile {
    pub(crate) fn set_value(&mut self, reg: Register, val: Word) {
        if reg.num == RegisterNumber::R0 {
            return;
        }

        match reg.size {
            WordOrTryte::Word => {
                self[reg.num] = val;
            }
            WordOrTryte::Tryte => {
                self[reg.num].set_tryte(<Word as Into<[Tryte; 3]>>::into(val)[0]);
            }
        }
    }

    pub(crate) fn set_value_either(&mut self, reg: Register, val: Either<Word, Tryte>) {
        if reg.num == RegisterNumber::R0 {
            return;
        }

        match reg.size {
            WordOrTryte::Word => match val {
                Left(word) => {
                    self[reg.num] = word;
                }
                Right(tryte) => {
                    self[reg.num] = tryte.into();
                }
            },
            WordOrTryte::Tryte => match val {
                Left(word) => {
                    self[reg.num] = <Word as Into<[Tryte; 3]>>::into(word)[0].into();
                }
                Right(tryte) => {
                    self[reg.num] = tryte.into();
                }
            },
        }
    }

    pub(crate) fn get_value(&self, reg: Register) -> Either<Word, Tryte> {
        if reg.num == RegisterNumber::R0 {
            return match reg.size {
                WordOrTryte::Word => Left(Word::default()),
                WordOrTryte::Tryte => Right(Tryte::default()),
            };
        }
        match reg.size {
            WordOrTryte::Word => Left(self[reg.num]),
            WordOrTryte::Tryte => Right(<Word as Into<[Tryte; 3]>>::into(self[reg.num])[0]),
        }
    }

    pub fn get_tryte(&self, reg: Register) -> Tryte {
        match self.get_value(reg) {
            Left(word) => <Word as Into<[Tryte; 3]>>::into(word)[0],
            Right(tryte) => tryte,
        }
    }

    pub fn get_word(&self, reg: Register) -> Word {
        match self.get_value(reg) {
            Left(word) => word,
            Right(tryte) => tryte.into(),
        }
    }
}

pub trait BimapEitherOps {
    fn bimap_add(self, rhs: Self) -> Either<WordAddResult, TryteAddResult>;
    fn bimap_add_tryte(self, rhs: Tryte) -> Either<WordAddResult, TryteAddResult>;
    fn bimap_sub(self, rhs: Self) -> Either<WordAddResult, TryteAddResult>;
    fn bimap_mul(self, rhs: Self) -> Either<Word, Tryte>;
    fn bimap_div(self, rhs: Self) -> Result<Either<Word, Tryte>, DivByZeroError>;
    fn bimap_mod(self, rhs: Self) -> Result<Either<Word, Tryte>, DivByZeroError>;
    fn bimap_and(self, rhs: Self) -> Either<Word, Tryte>;
    fn bimap_or(self, rhs: Self) -> Either<Word, Tryte>;
    fn as_word(self) -> Word;
}

impl GetStatus for Either<Word, Tryte> {
    fn get_sign(&self) -> Trit {
        self.either(|x| x.get_sign(), |x| x.get_sign())
    }

    fn get_parity(&self) -> Trit {
        self.either(|x| x.get_parity(), |x| x.get_parity())
    }
}

pub struct EitherAddResult {
    pub result: Either<Word, Tryte>,
    pub carry: Trit,
}

pub trait MapResult {
    fn mapres(self) -> Either<Word, Tryte>;
    fn bubbleres(self) -> EitherAddResult;
}

impl MapResult for Either<WordAddResult, TryteAddResult> {
    fn mapres(self) -> Either<Word, Tryte> {
        self.map_either(|r| r.result, |r| r.result)
    }

    fn bubbleres(self) -> EitherAddResult {
        match self {
            Left(res) => EitherAddResult {
                result: Left(res.result),
                carry: res.carry,
            },
            Right(res) => EitherAddResult {
                result: Right(res.result),
                carry: res.carry,
            },
        }
    }
}

impl BimapEitherOps for Either<Word, Tryte> {
    fn as_word(self) -> Word {
        match self {
            Left(word) => word,
            Right(tryte) => tryte.into(),
        }
    }
    fn bimap_and(self, rhs: Self) -> Either<Word, Tryte> {
        self.map_either(|r| r & rhs.unwrap_left(), |r| r & rhs.unwrap_right())
    }

    fn bimap_or(self, rhs: Self) -> Either<Word, Tryte> {
        self.map_either(|r| r | rhs.unwrap_left(), |r| r | rhs.unwrap_right())
    }

    fn bimap_add(self, rhs: Self) -> Either<WordAddResult, TryteAddResult> {
        self.map_either(|r| r + rhs.unwrap_left(), |r| r + rhs.unwrap_right())
    }

    fn bimap_add_tryte(self, rhs: Tryte) -> Either<WordAddResult, TryteAddResult> {
        self.map_either(|r| r + rhs, |r| r + rhs)
    }

    fn bimap_sub(self, rhs: Self) -> Either<WordAddResult, TryteAddResult> {
        self.map_either(|r| r - rhs.unwrap_left(), |r| r - rhs.unwrap_right())
    }

    fn bimap_mul(self, rhs: Self) -> Either<Word, Tryte> {
        self.map_either(|r| r * rhs.unwrap_left(), |r| r * rhs.unwrap_right())
    }

    fn bimap_div(self, rhs: Self) -> Result<Either<Word, Tryte>, DivByZeroError> {
        let temp = self.map_either(|r| (r / rhs.unwrap_left()), |r| (r / rhs.unwrap_right()));

        match temp {
            Left(val) => match val {
                Ok(word) => Ok(Left(word)),
                Err(err) => Err(err),
            },
            Right(val) => match val {
                Ok(tryte) => Ok(Right(tryte)),
                Err(err) => Err(err),
            },
        }
    }

    fn bimap_mod(self, rhs: Self) -> Result<Either<Word, Tryte>, DivByZeroError> {
        let temp = self.map_either(|r| (r % rhs.unwrap_left()), |r| (r % rhs.unwrap_right()));

        match temp {
            Left(val) => match val {
                Ok(word) => Ok(Left(word)),
                Err(err) => Err(err),
            },
            Right(val) => match val {
                Ok(tryte) => Ok(Right(tryte)),
                Err(err) => Err(err),
            },
        }
    }
}

#[cfg(test)]
pub mod test {
    use itertools::Either::{Left, Right};
    use ternary::{trits::Trit, tryte::Tryte, word::Word};

    use super::{Register, RegisterFile};

    #[test]
    fn reg_file_test() {
        let mut reg_file = RegisterFile::default();

        reg_file.set_value(
            (Trit::Zero, [Trit::POne, Trit::Zero, Trit::NOne]).into(),
            Word([Trit::POne; 27]),
        );

        assert_eq!(
            reg_file.get_value((Trit::Zero, [Trit::POne, Trit::Zero, Trit::NOne]).into()),
            Left(Word([Trit::POne; 27]))
        );

        reg_file.set_value(
            (Trit::NOne, [Trit::POne, Trit::Zero, Trit::NOne]).into(),
            Tryte([Trit::POne; 9]).into(),
        );

        assert_eq!(
            reg_file.get_value((Trit::NOne, [Trit::POne, Trit::Zero, Trit::NOne]).into()),
            Right(Tryte([Trit::POne; 9]).into())
        );
    }
}
