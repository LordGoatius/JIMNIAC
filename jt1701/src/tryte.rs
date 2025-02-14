use ternary::prelude::*;
use crate::GetStatus;

impl GetStatus for Tryte {
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

pub const ONE_TRYTE: Tryte = Tryte([P_ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO]);
pub const TWO_TRYTE: Tryte = Tryte([N_ONE, P_ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO]);
pub const THREE_TRYTE: Tryte = Tryte([ZERO, P_ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO]);
