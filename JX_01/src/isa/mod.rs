use septivigntimal::{Tribble, *};
use ternary::{trits::Trit, tryte::Tryte, word::Word};

use crate::isa::registers::Register;

pub use code::{encode, decode};

pub mod code;
pub mod registers;

pub type Control = Tribble;
pub type Op = Tribble;
pub type Opt = u32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instr {
    HALT,
    /// Disable Int
    DTI,
    /// Enable Int
    STI,
    /// Wait for Int
    WFI,
    /// Return from Int
    RTI,
    /// Load IDT
    LIT(Register),
    /// Int
    INTERRUPT(Tryte),
    /// Enable GPU
    EGPU(Register),
    /// Load Vector Buffer
    LVB(Register, Word),
    /// Enable GPU event loop
    EGEL(Register),
    /// Push CSR
    PCSR,
    /// Push PSR
    PPSR,
    /// Push PTR
    PPTR,
    /// POP CSR
    POCSR,
    /// POP PSR
    POPSR,
    /// POP PTR
    POPTR,
    /// Load Page Table
    LPT(Register),
    /// Int Mask
    INTM(Tryte),
    /// Int Enable
    INTE(Tryte),
    /// Int Toggle
    INTS(Tryte),
    /// Read In
    IN(Register, Control, Tryte),
    /// Write out
    OUT(Register, Control, Tryte),
    // Certain instructions ignore some of these arguments
    /// Op Instr (2 reg)
    OPRR(Control, Op, Register, Register, Word),
    /// Op Instr (1 reg)
    OPRI(Control, Op, Register, Word),
    /// Call function
    CALL(Register, Control, Word),
    /// Return function
    RET,
    // ENTER/LEAVE are CALL/RET
    // *but* with a beginning 1 instead of a 0 (usual) or T (cpu reserved)
    /// Enter Function
    ENTER,
    /// Leave Function
    LEAVE,
    /// Unrecognized Instr
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

pub const fn op_to_opt(op: Op) -> Opt {
    let mut opt: u32 = 0;
    opt |= (op[0] as u8) as u32 ;
    opt |= ((op[1] as u8) << 2) as u32 ;
    opt |= ((op[2] as u8) << 4) as u32 ;

    opt
}

pub const BPN:  Opt = op_to_opt(BPN_T);
pub const BPP:  Opt = op_to_opt(BPP_T);
pub const BPZ:  Opt = op_to_opt(BPZ_T);
pub const BGQ:  Opt = op_to_opt(BGQ_T);
pub const BLQ:  Opt = op_to_opt(BLQ_T);
pub const BLT:  Opt = op_to_opt(BLT_T);
pub const BGT:  Opt = op_to_opt(BGT_T);
pub const BNE:  Opt = op_to_opt(BNE_T);
pub const BEQ:  Opt = op_to_opt(BEQ_T);
pub const CMP:  Opt = op_to_opt(CMP_T);
pub const STRE: Opt = op_to_opt(STRE_T);
pub const LOAD: Opt = op_to_opt(LOAD_T);
pub const ADD:  Opt = op_to_opt(ADD_T);
pub const SUB:  Opt = op_to_opt(SUB_T);
pub const MUL:  Opt = op_to_opt(MUL_T);
pub const QOT:  Opt = op_to_opt(QOT_T);
pub const REM:  Opt = op_to_opt(REM_T);
pub const AND:  Opt = op_to_opt(AND_T);
pub const OR:   Opt = op_to_opt(OR_T);
pub const SFT:  Opt = op_to_opt(SFT_T);
pub const NOT:  Opt = op_to_opt(NOT_T);
pub const ROT:  Opt = op_to_opt(ROT_T);
pub const PUSH: Opt = op_to_opt(PUSH_T);
pub const POP:  Opt = op_to_opt(POP_T);
pub const CALL: Opt = op_to_opt(CALL_T);
pub const RET:  Opt = op_to_opt(RET_T);

pub const BPN_T: Op = Z;
pub const BPP_T: Op = Y;
pub const BPZ_T: Op = X;
pub const BGQ_T: Op = W;
pub const BLQ_T: Op = V;
pub const BLT_T: Op = U;
pub const BGT_T: Op = T;
pub const BNE_T: Op = S;
pub const BEQ_T: Op = R;
pub const CMP_T: Op = Q;
pub const STRE_T: Op = P;
pub const LOAD_T: Op = O;
pub const ADD_T: Op = A;
pub const SUB_T: Op = B;
pub const MUL_T: Op = C;
pub const QOT_T: Op = D;
pub const REM_T: Op = E;
pub const AND_T: Op = F;
pub const OR_T: Op = G;
pub const SFT_T: Op = H;
pub const NOT_T: Op = I;
pub const ROT_T: Op = J;
pub const PUSH_T: Op = K;
pub const POP_T: Op = L;
pub const CALL_T: Op = M;
pub const RET_T: Op = N;
