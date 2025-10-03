use septivigntimal::{Tribble, *};
use ternary::{trits::Trit, tryte::Tryte, word::Word};

use crate::isa::registers::Register;

pub mod code;
pub mod registers;

type Control = Tribble;
type Op = Tribble;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instr {
    HALT,
    DTI,
    STI,
    WFI,
    RTI,
    LIT(Register),
    INTERRUPT(Tryte),
    EGPU(Register),
    LVB(Register, Word),
    EGEL(Register),
    PCSR,
    PPSR,
    PPTR,
    POCSR,
    POPSR,
    POPTR,
    LPT(Register),
    INTM(Tryte),
    INTE(Tryte),
    INTS(Tryte),
    IN(Register, Control, Tryte),
    OUT(Register, Control, Tryte),
    // Certain instructions ignore some of these arguments
    OPRR(Control, Op, Register, Register, Word),
    OPRI(Control, Op, Register, Word),
    CALL(Register, Control, Word),
    RET,
    // ENTER/LEAVE are CALL/RET
    // *but* with a beginning 1 instead of a 0 (usual) or T (cpu reserved)
    ENTER,
    LEAVE,
    INVALID,
}

const IN_CTRL_T: Control = [Trit::Zero, Trit::NOne, Trit::POne];
const IN_CTRL_R: Control = [Trit::Zero, Trit::POne, Trit::POne];
const OUT_CTRL_T: Control = [Trit::Zero, Trit::NOne, Trit::POne];
const OUT_CTRL_R: Control = [Trit::Zero, Trit::POne, Trit::POne];

const ALU_CTRL_T_RR: Control = [Trit::NOne, Trit::NOne, Trit::Zero];
const ALU_CTRL_T_RI: Control = [Trit::Zero, Trit::NOne, Trit::Zero];
const ALU_CTRL_R_RR: Control = [Trit::NOne, Trit::POne, Trit::Zero];
const ALU_CTRL_R_RI: Control = [Trit::Zero, Trit::POne, Trit::Zero];

const CALL_CTRL_R: Control = [Trit::Zero, Trit::POne, Trit::Zero];
const CALL_CTRL_T: Control = [Trit::Zero, Trit::NOne, Trit::Zero];

pub const BPN: Op = Z;
pub const BPP: Op = Y;
pub const BPZ: Op = X;
pub const BGQ: Op = W;
pub const BLQ: Op = V;
pub const BLT: Op = U;
pub const BGT: Op = T;
pub const BNE: Op = S;
pub const BEQ: Op = R;
pub const CMP: Op = Q;
pub const STRE: Op = P;
pub const LOAD: Op = O;
pub const ADD: Op = A;
pub const SUB: Op = B;
pub const MUL: Op = C;
pub const QOT: Op = D;
pub const REM: Op = E;
pub const AND: Op = F;
pub const OR: Op = G;
pub const SFT: Op = H;
pub const NOT: Op = I;
pub const ROT: Op = J;
pub const PUSH: Op = K;
pub const POP: Op = L;
pub const CALL: Op = M;
pub const RET: Op = N;
