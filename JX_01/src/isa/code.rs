use septivigntimal::*;
use ternary::{trits::Trit, word::Word};

use crate::isa::{
    CALL_T as CALL,
    RET_T as RET,
    Instr,
    registers::Register
};

pub fn encode(instr: Instr) -> Word {
    match instr {
        Instr::HALT => [V, M, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::DTI => [V, B, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::STI => [V, S, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::WFI => [V, W, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::RTI => [V, R, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::LIT(register) => [V, A, register.0, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::INTERRUPT(tryte) => {
            let [i, n, t]: [[Trit; 3]; 3] = tryte.into();
            [V, I, I, t, n, i, ZERO, ZERO, ZERO].into()
        }
        Instr::EGPU(register) => [V, G, register.0, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::LVB(register, word) => {
            let [a, b, c, d, e, f, _, _, _]: [[Trit; 3]; 9] = word.into();
            [V, L, register.0, f, e, d, c, b, a].into()
        }
        Instr::EGEL(register) => [V, X, register.0, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::PCSR => [V, C, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::PPSR => [V, D, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::PPTR => [V, E, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::POCSR => [V, C, A, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::POPSR => [V, D, A, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::POPTR => [V, E, A, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::INTM(tryte) => {
            let [a, b, c]: [[Trit; 3]; 3] = tryte.into();
            [V, I, M, c, b, a, ZERO, ZERO, ZERO].into()
        }
        Instr::INTE(tryte) => {
            let [a, b, c]: [[Trit; 3]; 3] = tryte.into();
            [V, I, E, c, b, a, ZERO, ZERO, ZERO].into()
        }
        Instr::INTS(tryte) => {
            let [a, b, c]: [[Trit; 3]; 3] = tryte.into();
            [V, I, S, c, b, a, ZERO, ZERO, ZERO].into()
        }
        Instr::LPT(register) => [V, P, register.0, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::IN(register, control, tryte) => {
            let [a, b, c]: [[Trit; 3]; 3] = tryte.into();
            [
                [control[0], control[1], Trit::POne],
                Q,
                register.0,
                c,
                b,
                a,
                ZERO,
                ZERO,
                ZERO,
            ]
            .into()
        }
        Instr::OUT(register, control, tryte) => {
            let [a, b, c]: [[Trit; 3]; 3] = tryte.into();
            [
                [control[0], control[1], Trit::POne],
                V,
                register.0,
                c,
                b,
                a,
                ZERO,
                ZERO,
                ZERO,
            ]
            .into()
        }
        Instr::OPRR(control, op, register, register1, word) => {
            let [a, b, c, d, e, _, _, _, _] = word.into();
            [control, op, register.0, register1.0, e, d, c, b, a].into()
        }
        Instr::OPRI(control, op, register, word) => {
            let [a, b, c, d, e, f, _, _, _] = word.into();
            [control, op, register.0, f, e, d, c, b, a].into()
        }
        Instr::CALL(register, control, word) => {
            let [a, b, c, d, e, f, _, _, _] = word.into();
            [
                [Trit::Zero, control[1], Trit::Zero],
                CALL,
                register.0,
                f,
                e,
                d,
                c,
                b,
                a,
            ]
            .into()
        }
        Instr::ENTER => [I, CALL, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::RET => [ZERO, RET, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::LEAVE => [I, RET, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        Instr::INVALID => [A, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
    }
}

pub fn decode(word: Word) -> Instr {
    let word: [[Trit; 3]; 9] = word.into();
    match word {
        [V, I, I, i, n, t, ..] => {
            let tryte = [t, n, i].into();
            Instr::INTERRUPT(tryte)
        }
        [V, B, ZERO, ..] => Instr::DTI,
        [V, S, ZERO, ..] => Instr::STI,
        [V, W, ZERO, ..] => Instr::WFI,
        [V, R, ZERO, ..] => Instr::RTI,
        [V, M, ZERO, ..] => Instr::HALT,
        // CPU instructions always use a 27 trit register
        [V, A, r, ..] => Instr::LIT(Register(r)),
        [V, G, r, ..] => Instr::EGPU(Register(r)),
        [V, L, r, a, b, c, d, e, f] => {
            let word = [f, e, d, c, b, a, ZERO, ZERO, ZERO].into();
            Instr::LVB(Register(r), word)
        }
        [V, X, r, ..] => Instr::EGEL(Register(r)),
        [V, C, ZERO, ..] => Instr::PCSR,
        [V, D, ZERO, ..] => Instr::PPSR,
        [V, E, ZERO, ..] => Instr::PPTR,
        [V, C, A, ..] => Instr::POCSR,
        [V, D, A, ..] => Instr::POPSR,
        [V, E, A, ..] => Instr::POPTR,
        [V, P, r, ..] => Instr::LPT(Register(r)),
        [V, I, M, a, b, c, ..] => Instr::INTM([c, b, a].into()),
        [V, I, E, a, b, c, ..] => Instr::INTE([c, b, a].into()),
        [V, I, S, a, b, c, ..] => Instr::INTS([c, b, a].into()),
        // POne, control (reg size), RR/RI (doesn't matter)
        [ctrl @ [_, _, Trit::POne], Q, r, a, b, c, ..] => {
            Instr::IN(Register(r), ctrl, [c, b, a].into())
        }
        [ctrl @ [_, _, Trit::POne], V, r, a, b, c, ..] => {
            Instr::OUT(Register(r), ctrl, [c, b, a].into())
        }
        // Call/Ret/Enter/Leave
        [ctrl @ [_, _, Trit::Zero], CALL, r, a, b, c, d, e, f] => Instr::CALL(
            Register(r),
            ctrl,
            [f, e, d, c, b, a, ZERO, ZERO, ZERO].into(),
        ),
        // Instr::ENTER => [I, M, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        [I, CALL, ..] => Instr::ENTER,
        // Instr::RET => [ZERO, N, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        [ZERO, RET, ..] => Instr::RET,
        // Instr::LEAVE => [I, N, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
        [I, RET, ..] => Instr::LEAVE,
        // TODO: Consolidate the Op RR/RI into one instruction?
        [ctrl @ [Trit::NOne, _, Trit::Zero], op, r1, r2, a, b, c, d, e] => Instr::OPRR(
            ctrl,
            op,
            Register(r1),
            Register(r2),
            [e, d, c, b, a, ZERO, ZERO, ZERO, ZERO].into(),
        ),
        [ctrl @ [Trit::Zero, _, Trit::Zero], op, r, a, b, c, d, e, f] => Instr::OPRI(
            ctrl,
            op,
            Register(r),
            [f, e, d, c, b, a, ZERO, ZERO, ZERO].into(),
        ),
        _ => Instr::INVALID,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::isa::{
        code::{decode, encode},
        ADD_T as ADD,
        QOT_T as QOT
    };

    #[test]
    fn enc_dec() {
        use super::Instr::*;
        use crate::isa::registers::*;
        use crate::isa::{
            IN_CTRL_R,
            IN_CTRL_T,
            ALU_CTRL_R_RI,
            ALU_CTRL_R_RR,
            CALL_CTRL_R,
            CALL_CTRL_T
        };
        use ternary::word::Word;

        let instrs = vec![
            HALT,
            DTI,
            STI,
            WFI,
            RTI,
            LIT(N3),
            INTERRUPT(931.into()),
            EGPU(N8),
            LVB(NN12, [A, B, C, D, E, F, ZERO, ZERO, ZERO].into()),
            EGEL(N6),
            PCSR,
            PPSR,
            PPTR,
            POCSR,
            POPSR,
            POPTR,
            LPT(N5),
            INTM(3.into()),
            INTE(2.into()),
            INTS(1.into()),
            IN(N13, IN_CTRL_T, 7.into()),
            OUT(NN13, IN_CTRL_T, 12.into()),
            IN(N13, IN_CTRL_R, 7.into()),
            OUT(NN13, IN_CTRL_R, 12.into()),
            OPRR(ALU_CTRL_R_RR, ADD, N12, N13, Word::ZERO),
            OPRI(ALU_CTRL_R_RI, QOT, N11, Word::NONE),
            CALL(N4, CALL_CTRL_R, 6.into()),
            CALL(N4, CALL_CTRL_T, 6.into()),
            RET,
            ENTER,
            LEAVE,
            INVALID,
        ];

        for i in instrs {
            assert_eq!(i, decode(encode(i)));
        }
    }
}
