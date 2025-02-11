use std::{
    fmt::Debug,
    ops::{Add, Deref, DerefMut, Index, IndexMut},
};

use crate::{tryte::Tryte, GetStatus};
use crate::word::Word;
use crate::{trits::Trit, tryte::TryteAddResult, word::WordAddResult};

use itertools::{
    Either,
    Either::{Left, Right},
};

use super::errors::CpuError;

pub trait BimapEitherOps {
    fn bimap_add(self, rhs: Self) -> Either<WordAddResult, TryteAddResult>;
    fn bimap_add_tryte(self, rhs: Tryte) -> Either<WordAddResult, TryteAddResult>;
    fn bimap_sub(self, rhs: Self) -> Either<WordAddResult, TryteAddResult>;
    fn bimap_mul(self, rhs: Self) -> Either<Word, Tryte>;
    fn bimap_div(self, rhs: Self) -> Result<Either<Word, Tryte>, CpuError>;
    fn bimap_mod(self, rhs: Self) -> Result<Either<Word, Tryte>, CpuError>;
    fn bimap_and(self, rhs: Self) -> Either<Word, Tryte>;
    fn bimap_or(self, rhs: Self) -> Either<Word, Tryte>;
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
    pub carry: Trit
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
            Left(res) => EitherAddResult { result: Left(res.result), carry: res.carry },
            Right(res) => EitherAddResult { result: Right(res.result), carry: res.carry },
        }
    }
}

impl BimapEitherOps for Either<Word, Tryte> {
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

    fn bimap_div(self, rhs: Self) -> Result<Either<Word, Tryte>, CpuError> {
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

    fn bimap_mod(self, rhs: Self) -> Result<Either<Word, Tryte>, CpuError> {
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum WordOrTryte {
    Word,
    Tryte,
}

#[derive(Debug, Clone, Copy)]
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
    SP, // => R12
    BP, // => R13
}

#[derive(Debug, Clone, Copy)]
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

impl Register {
    /// size:
    ///     NOne => Tryte
    ///     Zero => Word
    pub(super) fn to_register(size: Trit, operands: [Trit; 3]) -> Register {
        let size = match size {
            Trit::NOne => WordOrTryte::Tryte,
            Trit::Zero => WordOrTryte::Word,
            _ => panic!(),
        };

        let num = match operands {
            [Trit::NOne, Trit::NOne, Trit::NOne] => RegisterNumber::RN13,
            [Trit::NOne, Trit::NOne, Trit::Zero] => RegisterNumber::RN12,
            [Trit::NOne, Trit::NOne, Trit::POne] => RegisterNumber::RN11,
            [Trit::NOne, Trit::Zero, Trit::NOne] => RegisterNumber::RN10,
            [Trit::NOne, Trit::Zero, Trit::Zero] => RegisterNumber::RN9,
            [Trit::NOne, Trit::Zero, Trit::POne] => RegisterNumber::RN8,
            [Trit::NOne, Trit::POne, Trit::NOne] => RegisterNumber::RN7,
            [Trit::NOne, Trit::POne, Trit::Zero] => RegisterNumber::RN6,
            [Trit::NOne, Trit::POne, Trit::POne] => RegisterNumber::RN5,
            [Trit::Zero, Trit::NOne, Trit::NOne] => RegisterNumber::RN4,
            [Trit::Zero, Trit::NOne, Trit::Zero] => RegisterNumber::RN3,
            [Trit::Zero, Trit::NOne, Trit::POne] => RegisterNumber::RN2,
            [Trit::Zero, Trit::Zero, Trit::NOne] => RegisterNumber::RN1,
            [Trit::Zero, Trit::Zero, Trit::Zero] => RegisterNumber::R0,
            [Trit::Zero, Trit::Zero, Trit::POne] => RegisterNumber::R1,
            [Trit::Zero, Trit::POne, Trit::NOne] => RegisterNumber::R2,
            [Trit::Zero, Trit::POne, Trit::Zero] => RegisterNumber::R3,
            [Trit::Zero, Trit::POne, Trit::POne] => RegisterNumber::R4,
            [Trit::POne, Trit::NOne, Trit::NOne] => RegisterNumber::R5,
            [Trit::POne, Trit::NOne, Trit::Zero] => RegisterNumber::R6,
            [Trit::POne, Trit::NOne, Trit::POne] => RegisterNumber::R7,
            [Trit::POne, Trit::Zero, Trit::NOne] => RegisterNumber::R8,
            [Trit::POne, Trit::Zero, Trit::Zero] => RegisterNumber::R9,
            [Trit::POne, Trit::Zero, Trit::POne] => RegisterNumber::R10,
            [Trit::POne, Trit::POne, Trit::NOne] => RegisterNumber::R11,
            [Trit::POne, Trit::POne, Trit::Zero] => RegisterNumber::R12,
            [Trit::POne, Trit::POne, Trit::POne] => RegisterNumber::R13,
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

#[cfg(test)]
pub mod test {
    use crate::{trits::Trit, tryte::Tryte, word::Word};
    use itertools::Either::{Left, Right};

    use super::{Register, RegisterFile};

    #[test]
    fn reg_file_test() {
        let mut reg_file = RegisterFile::default();

        reg_file.set_value(
            Register::to_register(Trit::Zero, [Trit::POne, Trit::Zero, Trit::NOne]),
            Word([Trit::POne; 27]),
        );

        assert_eq!(
            reg_file.get_value(Register::to_register(
                Trit::Zero,
                [Trit::POne, Trit::Zero, Trit::NOne]
            )),
            Left(Word([Trit::POne; 27]))
        );

        reg_file.set_value(
            Register::to_register(Trit::NOne, [Trit::POne, Trit::Zero, Trit::NOne]),
            Tryte([Trit::POne; 9]).into(),
        );

        assert_eq!(
            reg_file.get_value(Register::to_register(
                Trit::NOne,
                [Trit::POne, Trit::Zero, Trit::NOne]
            ),),
            Right(Tryte([Trit::POne; 9]).into())
        );
    }
}
