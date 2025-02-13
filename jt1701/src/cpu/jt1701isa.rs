use crate::{cpu::registers, septivigntimal::*, trits::Trit, tryte::Tryte, word::Word};

use super::{errors::CpuError, registers::Register};

#[allow(non_camel_case_types)]
pub trait jt1701 {
    //==== CPU ====//
    /// Load Interrupt Handler Table
    fn lht(&mut self, register: Register);
    /// Halt
    fn hlt(&mut self);
    /// Interrupt
    fn int(&mut self, interrupt: Tryte);
    /// No Op
    fn nop(&self);
    /// Wait For Interrupt
    fn wfi(&mut self);
    /// Stop Interrupts
    fn sti(&mut self);
    /// Begin Interrupts
    fn bti(&mut self);
    /// Returns from interrupt and restores status register
    fn rti(&mut self);

    //== Loading Register to Memory ==//
    /// dest = *(src + imm)
    fn ldri(&mut self, dest: Register, src: Register, imm: Tryte);
    /// dest = *(src0 + src1)
    fn ldrr(&mut self, dest: Register, src0: Register, src1: Register);
    /// dest = *(src0 + src1 + imm)
    fn ldrri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte);
    /// Word should have most sig. tryte be 0.
    /// dest = *(pc + imm)
    fn ldrpci(&mut self, dest: Register, imm: Word);

    //== Storing Register to Memory ==//
    /// *(src + imm) = dest
    fn stri(&mut self, dest: Register, src: Register, imm: Tryte);
    /// *(src0 + src1) = dest
    fn strr(&mut self, dest: Register, src0: Register, src1: Register);
    /// *(src0 + src1 + imm) = dest
    fn strri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte);
    /// Word should have most sig. tryte be 0.
    /// *(pc + imm) = dest
    fn strpci(&mut self, dest: Register, imm: Word);

    //== Moving ==//
    fn movrr(&mut self, dest: Register, src: Register);
    /// imm will always be 2 trytes long, or 18 trits.
    /// To move a larger size, the assembler will create shifts and adds in order to do so
    fn movri(&mut self, dest: Register, imm: Word);

    //==== ALU ====//
    // owo/uwu	
    // 0..=2: op	
    // 3..=5: ___: #imm	
    // 6..=8: %d, %s, %z
    fn owo(&mut self, imm: Tryte, dest: Register, src: Register);
    fn uwu(&mut self, imm: Tryte, dest: Register, src: Register);

    /// 0..=2: __:op _: control tryte (first rep reg type, second condition, third ???)			
    /// 3..=5: ___: #imm			
    /// 6..=8: %d, %s1, %s2
    fn add(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register);

    /// d = s0 * (s1 + imm)
    fn mul(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register);

    /// d = s0 - (s1 + imm)
    fn sub(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register);

    /// d = s0 / (s1 + imm)
    fn eqot(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) -> Result<(), CpuError>;

    /// d = s0 % (s1 + imm)
    fn erem(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) -> Result<(), CpuError>;

    //=== Trit ===//
    /// d = ~s
    fn not(&mut self, dest: Register, src: Register);

    // NOTE: Should left and right be different? -- YES (For eventual extensions for unbaltern)
    fn lsh(&mut self, dest: Register, src: Register, count: Tryte);
    fn rsh(&mut self, dest: Register, src: Register, count: Tryte);

    // NOTE: Should these be separate? - No (compiler will determine how to handle imm ands)
    fn and_r(&mut self, dest: Register, src0: Register, src1: Register);
    // fn and_i(&mut self, dest: Register, src0: Register, mask: Word);

    // NOTE: Should these be separate? - No
    fn or_r(&mut self, dest: Register, src0: Register, src1: Register);
    // fn or_i(&mut self, dest: Register, src0: Register, mask: Word);

    // FIXME: Should these be separate? - Yes
    fn rot_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn rot_i(&mut self, dest: Register, src0: Register, num: Word);

    //== Stack ==//
    /// (r0 + r1) * r2
    fn push_r3(&mut self, r0: Register, r1: Register, r2: Register);
    fn push_im_word(&mut self, imm: Word);
    fn push_im_tryte(&mut self, imm: Tryte);
    /// *((r0 + r1) * (r2 + imm))
    fn push_mem(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn pop(&mut self, dest: Register);

    //== Branch ==//
    /// Compare 2 registers
    fn cmp(&mut self, r0: Register, r1: Register);
    /// Set Parity Trit
    fn spt(&mut self, r: Register);
    /// Set Sign Trit
    fn sst(&mut self, r: Register);

    /// (r0 + r1) * r2
    /// Branch
    fn br_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn br_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn br_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bne_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bne_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bne_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bgt_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bgt_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bgt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn blt_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn blt_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn blt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn beq_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn beq_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn beq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bgeq_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bgeq_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bgeq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bleq_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bleq_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bleq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bofn_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bofn_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bofn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bofz_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bofz_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bofz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bofp_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bofp_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bofp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bpn_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bpn_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bpn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bpz_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bpz_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bpz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    /// (r0 + r1) * r2
    fn bpp_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bpp_i(&mut self, imm: Word);
    /// *((r0 + r1) * (r2 + imm))
    fn bpp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    //== Ports ==//
    fn in_r(&mut self, dest: Register, loc: Register);

    fn out_r(&mut self, dest: Register, val: Register);
    fn out_i(&mut self, dest: Register, val: Word);
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    LHT(Register),
    HLT,
    INT(Tryte),
    NOP,
    WFI,
    STI,
    BTI,
    RTI,
    LDRI(Register, Register, Tryte),
    LDRR(Register, Register, Register),
    LDRRI(Register, Register, Register, Tryte),
    LDRPCI(Register, Word),
    STRI(Register, Register, Tryte),
    STRR(Register, Register, Register),
    STRRI(Register, Register, Register, Tryte),
    STRPCI(Register, Word),
    MOVRR(Register, Register),
    MOVRI(Register, Word),
    OWO(Tryte, Register, Register),
    UWU(Tryte, Register, Register),
    ADD(Register, Tryte, Register, Register),
    MUL(Register, Tryte, Register, Register),
    SUB(Register, Tryte, Register, Register),
    EQOT(Register, Tryte, Register, Register),
    EREM(Register, Tryte, Register, Register),
    NOT(Register, Register),
    LSH(Register, Register, Tryte),
    RSH(Register, Register, Tryte),
    ANDR(Register, Register, Register),
    ORR(Register, Register, Register),
    ROTR(Register, Register, Register),
    ROTI(Register, Register, Word),
    PUSHR3(Register, Register, Register),
    PUSHIMWORD(Word),
    PUSHIMTRYTE(Tryte),
    PUSHMEM(Register, Register, Register, Tryte),
    POP(Register),
    CMP(Register, Register),
    SPT(Register),
    SST(Register),
    BRR(Register, Register, Register),
    BRI(Word),
    BRM(Register, Register, Register, Tryte),
    BNER(Register, Register, Register),
    BNEI(Word),
    BNEM(Register, Register, Register, Tryte),
    BGTR(Register, Register, Register),
    BGTI(Word),
    BGTM(Register, Register, Register, Tryte),
    BLTR(Register, Register, Register),
    BLTI(Word),
    BLTM(Register, Register, Register, Tryte),
    BEQR(Register, Register, Register),
    BEQI(Word),
    BEQM(Register, Register, Register, Tryte),
    BGEQR(Register, Register, Register),
    BGEQI(Word),
    BGEQM(Register, Register, Register, Tryte),
    BLEQR(Register, Register, Register),
    BLEQI(Word),
    BLEQM(Register, Register, Register, Tryte),
    BOFNR(Register, Register, Register),
    BOFNI(Word),
    BOFNM(Register, Register, Register, Tryte),
    BOFZR(Register, Register, Register),
    BOFZI(Word),
    BOFZM(Register, Register, Register, Tryte),
    BOFPR(Register, Register, Register),
    BOFPI(Word),
    BOFPM(Register, Register, Register, Tryte),
    BPNR(Register, Register, Register),
    BPNI(Word),
    BPNM(Register, Register, Register, Tryte),
    BPZR(Register, Register, Register),
    BPZI(Word),
    BPZM(Register, Register, Register, Tryte),
    BPPR(Register, Register, Register),
    BPPI(Word),
    BPPM(Register, Register, Register, Tryte),
    INR(Register, Register),
    OUTR(Register, Register),
    OUTI(Register, Word),
}

impl From<Word> for Instruction {
    fn from(value: Word) -> Instruction {
        const ZT: Trit = Trit::Zero;
        let value: [[Trit; 3]; 9] = value.into();
        match value {
            // CPU
            [L, I, T, reg, [t, ZT, ZT], ZERO, ZERO, ZERO, ZERO] => Instruction::LHT((t, reg).into()),
            [H, L, T, ..] => Instruction::HLT,
            [N, O, P, ..] => Instruction::NOP,
            [I, N, T, t0, t1, t2, ..] => Instruction::INT([t0, t1, t2].into()),
            [W, F, I, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO] => Instruction::WFI,
            [S, T, I, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO] => Instruction::STI,
            [B, T, I, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO] => Instruction::BTI,
            [R, T, I, ..] => Instruction::RTI,
            // LOAD
            [L, R, [t, ZT, ZT], d, r, ZERO, i0, i1, i2] => {
                Instruction::LDRI((t, d).into(), (t, r).into(), [i0, i1, i2].into())
            }
            [L, I, [t, ZT, ZT], d, r0, r1, ZERO, ZERO, ZERO] => {
                Instruction::LDRR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [L, B, [t, ZT, ZT], d, r0, r1, i0, i1, i2] => Instruction::LDRRI(
                (t, d).into(),
                (t, r0).into(),
                (t, r1).into(),
                [i0, i1, i2].into(),
            ),
            [L, P, [t, ZT, ZT], r, i0, i1, i2, i3, i4] => Instruction::LDRPCI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),
            // STORE
            [S, R, [t, ZT, ZT], d, r, ZERO, i0, i1, i2] => {
                Instruction::STRI((t, d).into(), (t, r).into(), [i0, i1, i2].into())
            }
            [S, I, [t, ZT, ZT], d, r0, r1, ZERO, ZERO, ZERO] => {
                Instruction::STRR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [S, B, [t, ZT, ZT], d, r0, r1, i0, i1, i2] => Instruction::STRRI(
                (t, d).into(),
                (t, r0).into(),
                (t, r1).into(),
                [i0, i1, i2].into(),
            ),
            [S, P, [t, ZT, ZT], r, i0, i1, i2, i3, i4] => Instruction::STRPCI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),
            // MOV
            [M, I, [t, ZT, ZT], r, i0, i1, i2, i3, i4] => Instruction::MOVRI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),
            [M, R, [t, ZT, ZT], r0, r1, ZERO, ZERO, ZERO, ZERO] => {
                Instruction::MOVRR((t, r0).into(), (t, r1).into())
            }
            // BIT
            [O, W, O, i0, i1, i2, d, r, ZERO] => Instruction::OWO(todo!(), todo!(), todo!()),
            [U, W, U, i0, i1, i2, d, r, ZERO] => Instruction::UWU(todo!(), todo!(), todo!()),
            // ALU
            [A, D, [t, ZT, ZT], i0, i1, i2, d, r0, r1] => Instruction::ADD(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [M, U, [t, ZT, ZT], i0, i1, i2, d, r0, r1] => Instruction::MUL(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [S, U, [t, ZT, ZT], i0, i1, i2, d, r0, r1] => Instruction::SUB(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [E, Q, [t, ZT, ZT], i0, i1, i2, d, r0, r1] => Instruction::EQOT(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [E, R, [t, ZT, ZT], i0, i1, i2, d, r0, r1] => Instruction::EREM(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            // BIT
            [N, O, [t, ZT, ZT], ZERO, ZERO, ZERO, d, r, ZERO] => {
                Instruction::NOT((t, d).into(), (t, r).into())
            }
            [L, S, [t, ZT, ZT], i0, i1, i2, d, r, ZERO] => {
                Instruction::LSH((t, d).into(), (t, r).into(), [i0, i1, i2].into())
            }
            [R, S, [t, ZT, ZT], i0, i1, i2, d, r0, r1] => {
                Instruction::RSH((t, d).into(), (t, r0).into(), [i0, i1, i2].into())
            }
            [A, N, [t, ZT, ZT], ZERO, ZERO, ZERO, d, r0, r1] => {
                Instruction::ANDR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [O, R, [t, ZT, ZT], ZERO, ZERO, ZERO, d, r0, r1] => {
                Instruction::ORR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [R, O, [t, Trit::POne, ZT], ZERO, ZERO, ZERO, d, r0, r1] => {
                Instruction::ROTR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [R, I, [t, Trit::NOne, ZT], i0, i1, i2, d, r, ZERO] => {
                Instruction::ROTI((t, d).into(), (t, r).into(), [i0, i1, i2, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into())
            }
            // Stack
            [P, R, [t, ZT, ZT], r0, r1, r2, ZERO, ZERO, ZERO] => {
                Instruction::PUSHR3((t, r0).into(), (t, r1).into(), (t, r2).into())
            }
            [P, I, ZERO, i0, i1, i2, i3, i4, i5] => {
                Instruction::PUSHIMWORD([i0, i1, i2, i3, i4, i5, ZERO, ZERO, ZERO].into())
            }
            [P, T, ZERO, i0, i1, i2, ZERO, ZERO, ZERO] => {
                Instruction::PUSHIMTRYTE([i0, i1, i2].into())
            }
            [P, M, [t, ZT, ZT], d, r0, r1, i0, i1, i2] => Instruction::PUSHMEM(
                (t, d).into(),
                (t, r0).into(),
                (t, r1).into(),
                [i0, i1, i2].into(),
            ),
            [P, P, [t, ZT, ZT], d, ZERO, ZERO, ZERO, ZERO, ZERO] => Instruction::POP((t, d).into()),
            [C, P, [t, ZT, ZT], r0, r1, ZERO, ZERO, ZERO, ZERO] => {
                Instruction::CMP((t, r0).into(), (t, r1).into())
            }
            [C, M, [t, ZT, ZT], r, ZERO, ZERO, ZERO, ZERO, ZERO] => Instruction::SPT((t, r).into()),
            [C, S, [t, ZT, ZT], r, ZERO, ZERO, ZERO, ZERO, ZERO] => Instruction::SST((t, r).into()),
            [B, ZERO, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BRR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BRI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BRM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, A, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BNER((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BNEI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BNEM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, B, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BGTR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BGTI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BGTM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, C, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BLTR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BLTI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BLTM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, D, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BEQR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BEQI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BEQM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, E, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BGEQR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BGEQI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BGEQM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, F, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BLEQR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BLEQI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BLEQM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, G, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BOFNR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BOFNI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BOFNM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, H, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BOFZR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BOFZI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BOFZM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, I, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BOFPR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BOFPI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BOFPM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, J, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BPNR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BPNI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BPNM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, K, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BPZR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BPZI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BPZM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [B, L, [t, h, ZT], a, b, c, d, e, f] => match h {
                // Register
                Trit::NOne => Instruction::BPPR((t, a).into(), (t, b).into(), (t, c).into()),
                // Imm
                Trit::Zero => Instruction::BPPI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()),
                // Mem
                Trit::POne => Instruction::BPPM(
                    (t, a).into(),
                    (t, b).into(),
                    (t, c).into(),
                    [d, e, f].into(),
                ),
            },
            [I, R, [t, ZT, ZT], r0, r1, ZERO, ZERO, ZERO, ZERO] => {
                Instruction::INR((t, r0).into(), (t, r1).into())
            }
            [O, R, [t, ZT, ZT], r0, r1, ZERO, ZERO, ZERO, ZERO] => {
                Instruction::OUTR((t, r0).into(), (t, r1).into())
            }
            [O, I, [t, ZT, ZT], r, i0, i1, i2, i3, i4] => Instruction::OUTI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),

            x => {
                let instr: Word = x.into();
                let instruction: Instruction = instr.into();
                println!("{instruction:?}");
                panic!("illegal instruction (interrupts not implemented yet)");
            },
        }
    }
}

impl From<Instruction> for Word {
    fn from(value: Instruction) -> Self {
        use self::Instruction::*;
        use crate::trits::Trit;

        match value {
                // [L, I, T, reg, [t, _, _], _, _, _, _] => Instruction::LHT((t, reg).into()),
                LHT(register) => [L, I, T, register.into(), [register.into(), Trit::Zero, Trit::Zero], ZERO, ZERO, ZERO, ZERO].into(),
                HLT => [H, L, T, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
                // [I, N, T, t0, t1, t2, ..] => Instruction::INT([t0, t1, t2].intotodo!()),
                INT(interrupt) => {
                    let parts: [[Trit; 3]; 3] = interrupt.into();
                    [I, N, T, parts[0], parts[1], parts[2], ZERO, ZERO, ZERO].into()
                },
                NOP => [N, O, P, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
                WFI => [W, F, I, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
                STI => [S, F, I, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
                BTI => [B, F, I, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
                RTI => [R, T, I, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into(),
                // [L, R, [t, _, _], d, r, _, i0, i1, i2] => {
                LDRI(dest, src, imm) => {
                    let (t, d) = dest.into();
                    let (_, r) = src.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [L, R, [t, Trit::Zero, Trit::Zero], d, r, ZERO, parts[0], parts[1], parts[2]].into()
                },
                STRI(dest, src, imm) => {
                    let (t, d) = dest.into();
                    let (_, r) = src.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [S, R, [t, Trit::Zero, Trit::Zero], d, r, ZERO, parts[0], parts[1], parts[2]].into()
                },

                // [L, I, [t, _, _], d, r0, r1, ZERO, ZERO, ZERO] => {
                LDRR(dest, src0, src1) => {
                    let (t, d) = dest.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [L, I, [t, Trit::Zero, Trit::Zero], d, r0, r1, ZERO, ZERO, ZERO].into()
                },
                STRR(dest, src0, src1) => {
                    let (t, d) = dest.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [S, I, [t, Trit::Zero, Trit::Zero], d, r0, r1, ZERO, ZERO, ZERO].into()
                },

                // [L, B, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::LDRRI(
                LDRRI(dest, src0, src1, imm) => {
                    let (t, d) = dest.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [L, B, [t, Trit::Zero, Trit::Zero], d, r0, r1, parts[0], parts[1], parts[2]].into()
                },
                STRRI(dest, src0, src1, imm) => {
                    let (t, d) = dest.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [S, B, [t, Trit::Zero, Trit::Zero], d, r0, r1, parts[0], parts[1], parts[2]].into()
                },

                // [L, P, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::LDRPCI(
                LDRPCI(dest, imm) => {
                    let (t, r) = dest.into();
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [L, P, [t, Trit::Zero, Trit::Zero], r, parts[0], parts[1], parts[2], parts[3], parts[4]].into()
                },
                STRPCI(dest, imm) => {
                    let (t, r) = dest.into();
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [S, P, [t, Trit::Zero, Trit::Zero], r, parts[0], parts[1], parts[2], parts[3], parts[4]].into()
                },

                // [M, R, [t, _, _], r0, r1, _, _, _, _] => {
                MOVRR(dest, src) => {
                    let (t, r0) = dest.into();
                    let (_, r1) = src.into();
                    [M, R, [t, Trit::Zero, Trit::Zero], r0, r1, ZERO, ZERO, ZERO, ZERO].into()
                },
                // [M, I, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::MOVRI(
                MOVRI(dest, imm) => {
                    let (t, r) = dest.into();
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [M, I, [t, Trit::Zero, Trit::Zero], r, parts[0], parts[1], parts[2], parts[3], parts[4]].into()
                },

                // [O, W, O, i0, i1, i2, d, r, ZERO] => Instruction::OWO(todo!(), todo!(), todo!(), todo!()),
                // [U, W, U, i0, i1, i2, d, r, ZERO] => Instruction::UWU(todo!(), todo!(), todo!(), todo!()),
                OWO(imm, dest, src) => todo!(),
                UWU(imm, dest, src) => todo!(),

                // [A, D, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::ADD(
                ADD(dest, imm, src0, src1) => {
                    let (t, d) = dest.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [A, D, [t, Trit::Zero, Trit::Zero], parts[0], parts[1], parts[2], d, r0, r1].into()
                },
                MUL(dest, imm, src0, src1) =>  {
                    let (t, d) = dest.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [M, U, [t, Trit::Zero, Trit::Zero], parts[0], parts[1], parts[2], d, r0, r1].into()
                },
                SUB(dest, imm, src0, src1) =>  {
                    let (t, d) = dest.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [S, U, [t, Trit::Zero, Trit::Zero], parts[0], parts[1], parts[2], d, r0, r1].into()
                },
                EQOT(dest, imm, src0, src1) => {
                    let (t, d) = dest.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [E, Q, [t, Trit::Zero, Trit::Zero], parts[0], parts[1], parts[2], d, r0, r1].into()
                },
                EREM(dest, imm, src0, src1) => {
                    let (t, d) = dest.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [E, R, [t, Trit::Zero, Trit::Zero], parts[0], parts[1], parts[2], d, r0, r1].into()
                },

                // [N, O, [t, _, _], i0, i1, i2, d, r, _] => {
                NOT(dest, src) => {
                    let (t, d) = dest.into();
                    // let parts: [[Trit; 3]; 3] = imm.into();
                    let (_, r) = src.into();
                    [N, O, [t, Trit::Zero, Trit::Zero], ZERO, ZERO, ZERO, d, r, ZERO].into()
                },

                // [L, S, [t, _, _], i0, i1, i2, d, r0, r1] => {
                LSH(dest, src, count) => {
                    let (t, d) = dest.into();
                    let parts: [[Trit; 3]; 3] = count.into();
                    let (_, r) = src.into();
                    [L, S, [t, Trit::Zero, Trit::Zero], parts[0], parts[1], parts[2], d, r, ZERO].into()
                },
                RSH(dest, src, count) => {
                    let (t, d) = dest.into();
                    let parts: [[Trit; 3]; 3] = count.into();
                    let (_, r) = src.into();
                    [R, S, [t, Trit::Zero, Trit::Zero], parts[0], parts[1], parts[2], d, r, ZERO].into()
                },

                // [A, N, [t, _, _], _, _, _, d, r0, r1] => {
                ANDR(dest, src0, src1) => {
                    let (t, d) = dest.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [A, N, [t, Trit::Zero, Trit::Zero], ZERO, ZERO, ZERO, d, r0, r1].into()
                },
                ORR(dest, src0, src1) => {
                    let (t, d) = dest.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [O, R, [t, Trit::Zero, Trit::Zero], ZERO, ZERO, ZERO, d, r0, r1].into()
                },
                ROTR(dest, src0, src1) => {
                    let (t, d) = dest.into();
                    let (_, r0) = src0.into();
                    let (_, r1) = src1.into();
                    [R, O, [t, Trit::POne, Trit::Zero], ZERO, ZERO, ZERO, d, r0, r1].into()
                },

                // [R, I, [t, Trit::NOne, _], i0, i1, i2, d, r, _] => {
                //     Instruction::ROTI((t, d).into(), (t, r).into(), [i0, i1, i2, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into())
                ROTI(dest, src, num) => {
                    let (t, d) = dest.into();
                    let (_, r) = src.into();
                    let parts: [[Trit; 3]; 9] = num.into();
                    [R, I, [t, Trit::NOne, Trit::Zero], parts[0], parts[1], parts[2], d, r, ZERO].into()
                },

                // [P, R, [t, _, _], r0, r1, r2, _, _, _] => {
                PUSHR3(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [P, R, [t, Trit::Zero, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },

                // [P, I, [t, _, _], i0, i1, i2, i3, i4, i5] => {
                PUSHIMWORD(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [P, I, ZERO, parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                // [P, T, [t, _, _], i0, i1, i2, ZERO, ZERO, ZERO] => {
                PUSHIMTRYTE(imm) => {
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [P, I, ZERO, parts[0], parts[1], parts[2], ZERO, ZERO, ZERO].into()
                },
                // [P, M, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::PUSHMEM(
                PUSHMEM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [P, R, [t, Trit::Zero, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },

                // [P, P, [t, _, _], d, _, _, _, _, _] => Instruction::POP((t, d).intotodo!()),
                POP(dest) => {
                    let (t, d) = dest.into();
                    [P, P, [t, Trit::Zero, Trit::Zero], d,  ZERO, ZERO, ZERO, ZERO, ZERO].into()
                },
                // [C, P, [t, _, _], r0, r1, _, _, _, _] => {
                CMP(r0, r1) => {
                    let (t, r0) = r0.into();
                    let (t, r1) = r1.into();
                    [C, P, [t, Trit::Zero, Trit::Zero], r0, r1, ZERO, ZERO, ZERO, ZERO].into()
                },
                // [C, M, [t, _, _], r, _, _, _, _, _] => Instruction::SPT((t, r).intotodo!()),
                SPT(r) => {
                    let (t, r) = r.into();
                    [C, M, [t, Trit::Zero, Trit::Zero], r, ZERO, ZERO, ZERO, ZERO, ZERO].into()
                },
                // [C, S, [t, _, _], r, _, _, _, _, _] => Instruction::SST((t, r).intotodo!()),
                SST(r) => {
                    let (t, r) = r.into();
                    [C, S, [t, Trit::Zero, Trit::Zero], r, ZERO, ZERO, ZERO, ZERO, ZERO].into()
                },

                // Register: NONE
                // [B, A, [t, h, _], a, b, c, d, e, f]
                // Trit::NOne => Instruction::BNER((t, a).into(), (t, b).into(), (t, c).into()),
                BRR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, ZERO, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BNER(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, A, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BGTR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, B, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BLTR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, C, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BEQR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, D, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BGEQR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, E, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BLEQR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, F, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BOFNR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, G, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BOFZR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, H, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BOFPR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, I, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BPNR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, J, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BPZR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, K, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },
                BPPR(r0, r1, r2) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    [B, L, [t, Trit::NOne, Trit::Zero], r0, r1, r2, ZERO, ZERO, ZERO].into()
                },

                // Imm: ZERO
                // [B, A, [t, h, _], a, b, c, d, e, f] => Instruction::BPZI([a, b, c, d, e, f, ZERO, ZERO, ZERO].into()
                BRI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, ZERO, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BNEI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, A, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BGTI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, B, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BLTI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, C, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BEQI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, D, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BGEQI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, E, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BLEQI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, F, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BOFNI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, G, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BOFZI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, H, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BOFPI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, I, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BPNI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, J, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BPZI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, K, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },
                BPPI(imm) => {
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [B, L, [Trit::Zero; 3], parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]].into()
                },

                // MemoryL PONE
                // [B, A, [t, h, _], a, b, c, d, e, f]
                BRM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, ZERO, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BNEM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, A, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BGTM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, B, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BLTM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, C, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BEQM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, D, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BGEQM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, E, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BLEQM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, F, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BOFNM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, G, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BOFZM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, H, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BOFPM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, I, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BPNM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, J, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BPZM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, K, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },
                BPPM(r0, r1, r2, imm) => {
                    let (t, r0) = r0.into();
                    let (_, r1) = r1.into();
                    let (_, r2) = r2.into();
                    let parts: [[Trit; 3]; 3] = imm.into();
                    [B, L, [t, Trit::POne, Trit::Zero], r0, r1, r2, parts[0], parts[1], parts[2]].into()
                },

                // [I, R, [t, _, _], r0, r1, _, _, _, _] => {
                //     Instruction::INR((t, r0).into(), (t, r1).into())
                // }
                INR(dest, loc) => {
                    let (t, r0) = dest.into();
                    let (_, r1) = loc.into();
                    [I, R, [t, Trit::Zero, Trit::Zero], r0, r1, ZERO, ZERO, ZERO, ZERO].into()
                },
                // [O, R, [t, _, _], r0, r1, _, _, _, _] => {
                //     Instruction::OUTR((t, r0).into(), (t, r1).into())
                // }
                OUTR(dest, loc) => {
                    let (t, r0) = dest.into();
                    let (_, r1) = loc.into();
                    [I, R, [t, Trit::Zero, Trit::Zero], r0, r1, ZERO, ZERO, ZERO, ZERO].into()
                },
                // [O, I, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::OUTI(
                //     (t, r).into(),
                //     [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
                OUTI(dest, imm) => {
                    let (t, r0) = dest.into();
                    let parts: [[Trit; 3]; 9] = imm.into();
                    [I, R, [t, Trit::Zero, Trit::Zero], r0, parts[0], parts[1], parts[2], parts[3], parts[4]].into()
                },
            }
    }
}

