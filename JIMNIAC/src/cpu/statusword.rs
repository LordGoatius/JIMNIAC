use std::ops::{Deref, DerefMut};

use ternary::{trits::Trit, tryte::Tryte, word::Word};

/// Status Word:
/// [C, S, P, I, R, _, _, _, _,
///  [_; 9],
///  interrupt_number,
/// ]
/// C: Carry Flag
/// S: Sign Flag
/// P: Parity Flag
/// I: Interrupts Enabled
/// R: Privledge Level
/// N: Interrupt number

#[derive(Debug, Clone, Copy, Default)]
pub struct StatusWord(Word);

impl Deref for StatusWord {
    type Target = Word;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatusWord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StatusWord {
    #[inline]
    pub(crate) fn set_carry_flag(&mut self, flag: Trit) {
        self[0] = flag;
    }

    #[inline]
    pub(crate) fn set_sign_flag(&mut self, flag: Trit) {
        self[1] = flag;
    }

    #[inline]
    pub(crate) fn set_parity_flag(&mut self, flag: Trit) {
        self[2] = flag;
    }

    #[inline]
    pub(crate) fn set_interrupt_enable(&mut self, flag: Trit) {
        self[18] = flag;
    }

    #[inline]
    pub(crate) fn set_priv_level(&mut self, flag: Trit) {
        self[19] = flag;
    }

    #[inline]
    pub(crate) fn set_interrupt_number(&mut self, num: Tryte) {
        let [first, second, _] = self.0.into();
        *self = StatusWord([first, second, num].into());
    }

    // Getting
    #[inline]
    pub(crate) fn get_carry_flag(&self) -> Trit {
        self[0]
    }

    #[inline]
    pub(crate) fn get_sign_flag(&self) -> Trit {
        self[1]
    }

    #[inline]
    pub(crate) fn get_parity_flag(&self) -> Trit {
        self[2]
    }

    #[inline]
    pub(crate) fn get_interrupt_enable(&self) -> Trit {
        self[18]
    }

    #[inline]
    pub(crate) fn get_priv_level(&self) -> Trit {
        self[19]
    }

    #[inline]
    pub(crate) fn get_interrupt_number(&mut self) -> Tryte {
        let [_, _, num] = self.0.into();
        num
    }
}
