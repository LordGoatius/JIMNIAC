use crate::{trits::Trit, word::Word};

/// Status Word:
/// [C, S, P, _, _, _, _, _, _,
///  interrupt_vector
///  I, R, _, _, _, _, _, _, _,
/// ]
/// C: Carry Flag
/// S: Sign Flag
/// P: Parity Flag
/// I: Interrupts Enabled
/// R: Privledge Level

pub type StatusWord = Word;

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
}
