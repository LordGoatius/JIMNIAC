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

pub const fn op_to_opt(op: Op) -> Opt {
    let mut opt: u32 = 0;
    opt |= (op[0] as u8) as u32 ;
    opt |= ((op[1] as u8) << 2) as u32 ;
    opt |= ((op[2] as u8) << 4) as u32 ;

    opt
}

pub const BPN: Opt = op_to_opt(Z);
pub const BPP: Opt = op_to_opt(Y);
pub const BPZ: Opt = op_to_opt(X);
pub const BGQ: Opt = op_to_opt(W);
pub const BLQ: Opt = op_to_opt(V);
pub const BLT: Opt = op_to_opt(U);
pub const BGT: Opt = op_to_opt(T);
pub const BNE: Opt = op_to_opt(S);
pub const BEQ: Opt = op_to_opt(R);
pub const CMP: Opt = op_to_opt(Q);
pub const STRE: Opt = op_to_opt(P);
pub const LOAD: Opt = op_to_opt(O);
pub const ADD: Opt = op_to_opt(A);
pub const SUB: Opt = op_to_opt(B);
pub const MUL: Opt = op_to_opt(C);
pub const QOT: Opt = op_to_opt(D);
pub const REM: Opt = op_to_opt(E);
pub const AND: Opt = op_to_opt(F);
pub const OR: Opt = op_to_opt(G);
pub const SFT: Opt = op_to_opt(H);
pub const NOT: Opt = op_to_opt(I);
pub const ROT: Opt = op_to_opt(J);
pub const PUSH: Opt = op_to_opt(K);
pub const POP: Opt = op_to_opt(L);
pub const CALL: Opt = op_to_opt(M);
pub const RET: Opt = op_to_opt(N);

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
