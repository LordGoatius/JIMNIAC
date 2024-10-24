use crate::trits::Trit;
use crate::tryte::Tryte;
use crate::word::Word;

pub(super) enum WordOrTryte {
    Word(Word),
    Tryte(Tryte),
}

pub(super) enum Register {
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
}

impl Register {
    pub(super) fn to_register(operands: [Trit; 3]) -> Register {
        match operands {
            [Trit::NOne, Trit::NOne, Trit::NOne] => Register::RN13,
            [Trit::NOne, Trit::NOne, Trit::Zero] => Register::RN12,
            [Trit::NOne, Trit::NOne, Trit::POne] => Register::RN11,
            [Trit::NOne, Trit::Zero, Trit::NOne] => Register::RN10,
            [Trit::NOne, Trit::Zero, Trit::Zero] => Register::RN9,
            [Trit::NOne, Trit::Zero, Trit::POne] => Register::RN8,
            [Trit::NOne, Trit::POne, Trit::NOne] => Register::RN7,
            [Trit::NOne, Trit::POne, Trit::Zero] => Register::RN6,
            [Trit::NOne, Trit::POne, Trit::POne] => Register::RN5,
            [Trit::Zero, Trit::NOne, Trit::NOne] => Register::RN4,
            [Trit::Zero, Trit::NOne, Trit::Zero] => Register::RN3,
            [Trit::Zero, Trit::NOne, Trit::POne] => Register::RN2,
            [Trit::Zero, Trit::Zero, Trit::NOne] => Register::RN1,
            [Trit::Zero, Trit::Zero, Trit::Zero] => Register::R0,
            [Trit::Zero, Trit::Zero, Trit::POne] => Register::R1,
            [Trit::Zero, Trit::POne, Trit::NOne] => Register::R2,
            [Trit::Zero, Trit::POne, Trit::Zero] => Register::R3,
            [Trit::Zero, Trit::POne, Trit::POne] => Register::R4,
            [Trit::POne, Trit::NOne, Trit::NOne] => Register::R5,
            [Trit::POne, Trit::NOne, Trit::Zero] => Register::R6,
            [Trit::POne, Trit::NOne, Trit::POne] => Register::R7,
            [Trit::POne, Trit::Zero, Trit::NOne] => Register::R8,
            [Trit::POne, Trit::Zero, Trit::Zero] => Register::R9,
            [Trit::POne, Trit::Zero, Trit::POne] => Register::R10,
            [Trit::POne, Trit::POne, Trit::NOne] => Register::R11,
            [Trit::POne, Trit::POne, Trit::Zero] => Register::R12,
            [Trit::POne, Trit::POne, Trit::POne] => Register::R13,
        }
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

impl RegisterFile {
    /// Register selects which register
    /// toreplace is the value to replace selects lower Tryte or Full word
    pub(crate) fn set_value(&mut self, reg: Register, toreplace: WordOrTryte) {
        let word_replace = match toreplace {
            WordOrTryte::Word(word) => word,
            WordOrTryte::Tryte(tryte) => [tryte, Tryte::default(), Tryte::default()].into(),
        };
        match reg {
            Register::RN13 => self.rn13 = word_replace,
            Register::RN12 => self.rn12 = word_replace,
            Register::RN11 => self.rn11 = word_replace,
            Register::RN10 => self.rn10 = word_replace,
            Register::RN9 => self.rn9 = word_replace,
            Register::RN8 => self.rn8 = word_replace,
            Register::RN7 => self.rn7 = word_replace,
            Register::RN6 => self.rn6 = word_replace,
            Register::RN5 => self.rn5 = word_replace,
            Register::RN4 => self.rn4 = word_replace,
            Register::RN3 => self.rn3 = word_replace,
            Register::RN2 => self.rn2 = word_replace,
            Register::RN1 => self.rn1 = word_replace,
            Register::R0 => self.r0 = word_replace,
            Register::R1 => self.r1 = word_replace,
            Register::R2 => self.r2 = word_replace,
            Register::R3 => self.r3 = word_replace,
            Register::R4 => self.r4 = word_replace,
            Register::R5 => self.r5 = word_replace,
            Register::R6 => self.r6 = word_replace,
            Register::R7 => self.r7 = word_replace,
            Register::R8 => self.r8 = word_replace,
            Register::R9 => self.r9 = word_replace,
            Register::R10 => self.r10 = word_replace,
            Register::R11 => self.r11 = word_replace,
            Register::R12 => self.r12 = word_replace,
            Register::R13 => self.r13 = word_replace,
        }
    }

    pub(crate) fn get_value(&self, reg: Register) -> Word {
        match reg {
            Register::RN13 => self.rn13,
            Register::RN12 => self.rn12,
            Register::RN11 => self.rn11,
            Register::RN10 => self.rn10,
            Register::RN9 => self.rn9,
            Register::RN8 => self.rn8,
            Register::RN7 => self.rn7,
            Register::RN6 => self.rn6,
            Register::RN5 => self.rn5,
            Register::RN4 => self.rn4,
            Register::RN3 => self.rn3,
            Register::RN2 => self.rn2,
            Register::RN1 => self.rn1,
            Register::R0 => self.r0,
            Register::R1 => self.r1,
            Register::R2 => self.r2,
            Register::R3 => self.r3,
            Register::R4 => self.r4,
            Register::R5 => self.r5,
            Register::R6 => self.r6,
            Register::R7 => self.r7,
            Register::R8 => self.r8,
            Register::R9 => self.r9,
            Register::R10 => self.r10,
            Register::R11 => self.r11,
            Register::R12 => self.r12,
            Register::R13 => self.r13,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{trits::Trit, tryte::Tryte, word::Word};

    use super::{Register, RegisterFile, WordOrTryte};

    #[test]
    fn reg_file_test() {
        let mut reg_file = RegisterFile::default();

        reg_file.set_value(
            Register::to_register([Trit::POne, Trit::Zero, Trit::NOne]),
            WordOrTryte::Word(Word([Trit::POne; 27])),
        );

        assert_eq!(reg_file.get_value(
            Register::to_register([Trit::POne, Trit::Zero, Trit::NOne]),
        ), Word([Trit::POne; 27]));

        reg_file.set_value(
            Register::to_register([Trit::POne, Trit::Zero, Trit::NOne]),
            WordOrTryte::Tryte(Tryte([Trit::POne; 9])),
        );

        assert_eq!(reg_file.get_value(
            Register::to_register([Trit::POne, Trit::Zero, Trit::NOne]),
        ), [[Trit::POne; 9].into(), [Trit::Zero; 9].into(), [Trit::Zero; 9].into()].into());

    }
}
