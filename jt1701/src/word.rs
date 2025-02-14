use ternary::prelude::*;
use crate::GetStatus;

impl GetStatus for Word {
    fn get_sign(&self) -> Trit {
        for i in (0..self.len()).rev() {
            if self[i] != Trit::Zero {
                return self[i];
            }
        }
        Trit::Zero
    }

    fn get_parity(&self) -> Trit {
        self[0]
    }
}

const N_ONE: Trit = Trit::NOne;
const ZERO:  Trit = Trit::Zero;
const P_ONE: Trit = Trit::POne;

pub const ONE_WORD: Word = Word([P_ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO]);
pub const TWO_WORD: Word = Word([N_ONE, P_ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO]);
pub const THREE_WORD: Word = Word([ZERO, P_ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO]);
