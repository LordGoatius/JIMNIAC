use super::registers::Register;
use super::Cpu;
use crate::cpu::jt1701isa::jt1701;
use crate::septivigntimal::*;
use crate::tryte::Tryte;
use crate::{word::Word, Trit};

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
    INI(Register, Word),
    OUTR(Register, Register),
    OUTI(Register, Word),
}

impl Cpu {
    fn fetch(&mut self) {
        'main: loop {
            let instruction = self.stack.get_word(self.program_counter);
            let instruction = Cpu::decode(instruction);
            self.execute(instruction);
        }
    }

    fn decode(instr: Word) -> Instruction {
        let instr: [[Trit; 3]; 9] = instr.into();
        match instr {
            // [_, _, _, _, _, _, _, _, _] => Instruction::
            // CPU
            [L, I, T, reg, [t, _, _], _, _, _, _] => Instruction::LHT((t, reg).into()),
            [H, L, T, ..] => Instruction::HLT,
            [I, N, T, t0, t1, t2, ..] => Instruction::INT([t0, t1, t2].into()),
            [W, F, I, _, _, _, _, _, _] => Instruction::WFI,
            [S, T, I, _, _, _, _, _, _] => Instruction::STI,
            [B, T, I, _, _, _, _, _, _] => Instruction::BTI,
            // LOAD
            [L, R, [t, _, _], d, r, _, i0, i1, i2] => Instruction::LDRI((t, d).into(), (t, r).into(), [i0, i1, i2].into()),
            [L, I, [t, _, _], d, r0, r1, ZERO, ZERO, ZERO] => Instruction::LDRR((t, d).into(), (t, r0).into(), (t, r1).into()),
            [L, B, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::LDRRI((t, d).into(), (t, r0).into(), (t, r1).into(), [i0, i1, i2].into()),
            [L, P, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::LDRPCI((t, r).into(), [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into()),
            // STORE
            [S, R, [t, _, _], d, r, _, i0, i1, i2] => Instruction::STRI((t, d).into(), (t, r).into(), [i0, i1, i2].into()),
            [S, I, [t, _, _], d, r0, r1, ZERO, ZERO, ZERO] => Instruction::STRR((t, d).into(), (t, r0).into(), (t, r1).into()),
            [S, B, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::STRRI((t, d).into(), (t, r0).into(), (t, r1).into(), [i0, i1, i2].into()),
            [S, P, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::STRPCI((t, r).into(), [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into()),
            // MOV
            [M, I, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::MOVRI((t, r).into(), [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into()),
            [M, R, [t, _, _], r0, r1, _, _, _, _] => Instruction::MOVRR((t, r0).into(), (t, r1).into()),
            // BIT
            [O, W, O, i0, i1, i2, d, r, ZERO] => Instruction::OWO(todo!(), todo!(), todo!()),
            [U, W, U, i0, i1, i2, d, r, ZERO] => Instruction::UWU(todo!(), todo!(), todo!()),
            // ALU
            [A, D, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::ADD((t, d).into(), [i0, i1, i2].into(), (t, r0).into(), (t, r1).into()),
            [M, U, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::MUL((t, d).into(), [i0, i1, i2].into(), (t, r0).into(), (t, r1).into()),
            [S, U, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::SUB((t, d).into(), [i0, i1, i2].into(), (t, r0).into(), (t, r1).into()),
            [E, Q, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::EQOT((t, d).into(), [i0, i1, i2].into(), (t, r0).into(), (t, r1).into()),
            [E, R, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::EREM((t, d).into(), [i0, i1, i2].into(), (t, r0).into(), (t, r1).into()),
            // BIT
            [N, O, [t, _, _], i0, i1, i2, d, r, _] => Instruction::NOT((t, d).into(), (t, r).into()),
            [L, S, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::LSH((t, d).into(), (t, r0).into(), [i0, i1, i2].into()),
            [R, S, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::RSH((t, d).into(), (t, r0).into(), [i0, i1, i2].into()),
            [A, N, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::ANDR((t, d).into(), (t, r0).into(), (t, r1).into()),
            [O, R, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::ORR((t, d).into(), (t, r0).into(), (t, r1).into()),
            [R, O, [t, Trit::POne, _], i0, i1, i2, d, r0, r1] => Instruction::ROTR((t, d).into(), (t, r0).into(), (t, r1).into()),
            [R, O, [t, Trit::NOne, _], i0, i1, i2, d, r0, r1] => Instruction::ROTI((t, d).into(), (t, r0).into(), todo!()),
            // Stack
            [P, R, [t, _, _], r0, r1, r2, _, _, _] => Instruction::PUSHR3((t, r0).into(), (t, r1).into(), (t, r2).into()),
            [P, I, [t, _, _], i0, i1, i2, i3, i4, i5] => Instruction::PUSHIMWORD([i0, i1, i2, i3, i4, i5, ZERO, ZERO, ZERO].into()),
            [P, T, [t, _, _], i0, i1, i2, ZERO, ZERO, ZERO] => Instruction::PUSHIMTRYTE([i0, i1, i2].into()),
            [P, M, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::PUSHMEM((t, d).into(), (t, r0).into(), (t, r1).into(), [i0, i1, i2].into()),


            // [_, _, _, _, _, _, _, _, _] => Instruction::
            _ => panic!("illegal instruction (interrupts not implemented yet)"),
        }
    }

    fn execute(&mut self, machine_op: Instruction) {
        // I KNOW BUT IN THIS ONE CASE IT IS OKAY
        use Instruction::*;

        // FIXME: Some of these require doing stuff
        match machine_op {
            LHT(register) => self.lht(register),
            HLT => self.hlt(),
            INT(interrupt) => self.int(interrupt),
            NOP => self.nop(),
            WFI => self.wfi(),
            STI => self.sti(),
            BTI => self.bti(),
            RTI => self.rti(),
            LDRI(dest, src, imm) => self.ldri(dest, src, imm),
            LDRR(dest, src0, src1) => self.ldrr(dest, src0, src1),
            LDRRI(dest, src0, src1, imm) => self.ldrri(dest, src0, src1, imm),
            LDRPCI(dest, imm) => self.ldrpci(dest, imm),
            STRI(dest, src, imm) => self.stri(dest, src, imm),
            STRR(dest, src0, src1) => self.strr(dest, src0, src1),
            STRRI(dest, src0, src1, imm) => self.strri(dest, src0, src1, imm),
            STRPCI(dest, imm) => self.strpci(dest, imm),
            MOVRR(dest, src) => self.movrr(dest, src),
            MOVRI(dest, imm) => self.movri(dest, imm),
            OWO(imm, dest, src) => self.owo(imm, dest, src),
            UWU(imm, dest, src) => self.uwu(imm, dest, src),
            ADD(dest, imm, src0, src1) => self.add(dest, imm, src0, src1),
            MUL(dest, imm, src0, src1) => self.mul(dest, imm, src0, src1),
            SUB(dest, imm, src0, src1) => self.sub(dest, imm, src0, src1),
            EQOT(dest, imm, src0, src1) => {
                self.eqot(dest, imm, src0, src1);
            }
            EREM(dest, imm, src0, src1) => {
                self.erem(dest, imm, src0, src1);
            }
            NOT(dest, src) => self.not(dest, src),
            LSH(dest, src, count) => self.lsh(dest, src, count),
            RSH(dest, src, count) => self.rsh(dest, src, count),
            ANDR(dest, src0, src1) => self.and_r(dest, src0, src1),
            ORR(dest, src0, src1) => self.or_r(dest, src0, src1),
            ROTR(dest, src0, src1) => self.rot_r(dest, src0, src1),
            ROTI(dest, src0, num) => self.rot_i(dest, src0, num),
            PUSHR3(r0, r1, r2) => self.push_r3(r0, r1, r2),
            PUSHIMWORD(imm) => self.push_im_word(imm),
            PUSHIMTRYTE(imm) => self.push_im_tryte(imm),
            PUSHMEM(r0, r1, r2, imm) => self.push_mem(r0, r1, r2, imm),
            CMP(r0, r1) => self.cmp(r0, r1),
            SPT(r) => self.spt(r),
            SST(r) => self.sst(r),
            BRR(r0, r1, r2) => self.br_r(r0, r1, r2),
            BRI(imm) => self.br_i(imm),
            BRM(r0, r1, r2, imm) => self.br_m(r0, r1, r2, imm),
            BNER(r0, r1, r2) => self.bne_r(r0, r1, r2),
            BNEI(imm) => self.bne_i(imm),
            BNEM(r0, r1, r2, imm) => self.bne_m(r0, r1, r2, imm),
            BGTR(r0, r1, r2) => self.bgt_r(r0, r1, r2),
            BGTI(imm) => self.bgt_i(imm),
            BGTM(r0, r1, r2, imm) => self.bgt_m(r0, r1, r2, imm),
            BLTR(r0, r1, r2) => self.blt_r(r0, r1, r2),
            BLTI(imm) => self.blt_i(imm),
            BLTM(r0, r1, r2, imm) => self.blt_m(r0, r1, r2, imm),
            BEQR(r0, r1, r2) => self.beq_r(r0, r1, r2),
            BEQI(imm) => self.beq_i(imm),
            BEQM(r0, r1, r2, imm) => self.beq_m(r0, r1, r2, imm),
            BGEQR(r0, r1, r2) => self.bgeq_r(r0, r1, r2),
            BGEQI(imm) => self.bgeq_i(imm),
            BGEQM(r0, r1, r2, imm) => self.bgeq_m(r0, r1, r2, imm),
            BLEQR(r0, r1, r2) => self.bleq_r(r0, r1, r2),
            BLEQI(imm) => self.bleq_i(imm),
            BLEQM(r0, r1, r2, imm) => self.bleq_m(r0, r1, r2, imm),
            BOFNR(r0, r1, r2) => self.bofn_r(r0, r1, r2),
            BOFNI(imm) => self.bofn_i(imm),
            BOFNM(r0, r1, r2, imm) => self.bofn_m(r0, r1, r2, imm),
            BOFZR(r0, r1, r2) => self.bofz_r(r0, r1, r2),
            BOFZI(imm) => self.bofz_i(imm),
            BOFZM(r0, r1, r2, imm) => self.bofz_m(r0, r1, r2, imm),
            BOFPR(r0, r1, r2) => self.bofp_r(r0, r1, r2),
            BOFPI(imm) => self.bofp_i(imm),
            BOFPM(r0, r1, r2, imm) => self.bofp_m(r0, r1, r2, imm),
            BPNR(r0, r1, r2) => self.bpn_r(r0, r1, r2),
            BPNI(imm) => self.bpn_i(imm),
            BPNM(r0, r1, r2, imm) => self.bpn_m(r0, r1, r2, imm),
            BPZR(r0, r1, r2) => self.bpz_r(r0, r1, r2),
            BPZI(imm) => self.bpz_i(imm),
            BPZM(r0, r1, r2, imm) => self.bpz_m(r0, r1, r2, imm),
            BPPR(r0, r1, r2) => self.bpp_r(r0, r1, r2),
            BPPI(imm) => self.bpp_i(imm),
            BPPM(r0, r1, r2, imm) => self.bpp_m(r0, r1, r2, imm),
            INR(dest, loc) => self.in_r(dest, loc),
            INI(dest, loc) => self.in_i(dest, loc),
            OUTR(dest, loc) => self.out_r(dest, loc),
            OUTI(dest, loc) => self.out_i(dest, loc),
        }
    }
}
