use super::registers::Register;
use super::Cpu;
use crate::cpu::jt1701isa::{jt1701, Instruction};
use crate::septivigntimal::*;
use crate::tryte::Tryte;
use crate::{word::Word, Trit};

impl Cpu {
    fn fetch(&mut self) {
        use Instruction::*;
        'main: loop {
            let instruction = self.stack.get_word(self.program_counter);
            let instruction = Cpu::decode(instruction);
            match instruction {
                LHT(register) => {
                    self.lht(register);
                    self.inc_pc();
                }
                HLT => {
                    self.hlt();
                    break 'main;
                }
                INT(interrupt) => {
                    self.int(interrupt);
                    todo!()
                }
                NOP => {
                    self.nop();
                }
                WFI => {
                    self.wfi();
                    todo!()
                }
                STI => {
                    self.sti();
                    self.inc_pc();
                }
                BTI => {
                    self.bti();
                    self.inc_pc();
                }
                RTI => {
                    self.rti();
                }
                LDRI(dest, src, imm) => {
                    self.ldri(dest, src, imm);
                    self.inc_pc();
                }
                LDRR(dest, src0, src1) => {
                    self.ldrr(dest, src0, src1);
                    self.inc_pc();
                }
                LDRRI(dest, src0, src1, imm) => {
                    self.ldrri(dest, src0, src1, imm);
                    self.inc_pc();
                }
                LDRPCI(dest, imm) => {
                    self.ldrpci(dest, imm);
                    self.inc_pc();
                }
                STRI(dest, src, imm) => {
                    self.stri(dest, src, imm);
                    self.inc_pc();
                }
                STRR(dest, src0, src1) => {
                    self.strr(dest, src0, src1);
                    self.inc_pc();
                }
                STRRI(dest, src0, src1, imm) => {
                    self.strri(dest, src0, src1, imm);
                    self.inc_pc();
                }
                STRPCI(dest, imm) => {
                    self.strpci(dest, imm);
                    self.inc_pc();
                }
                MOVRR(dest, src) => {
                    self.movrr(dest, src);
                    self.inc_pc();
                }
                MOVRI(dest, imm) => {
                    self.movri(dest, imm);
                    self.inc_pc();
                }
                OWO(imm, dest, src) => {
                    self.owo(imm, dest, src);
                    self.inc_pc();
                }
                UWU(imm, dest, src) => {
                    self.uwu(imm, dest, src);
                    self.inc_pc();
                }
                ADD(dest, imm, src0, src1) => {
                    self.add(dest, imm, src0, src1);
                    self.inc_pc();
                }
                MUL(dest, imm, src0, src1) => {
                    self.mul(dest, imm, src0, src1);
                    self.inc_pc();
                }
                SUB(dest, imm, src0, src1) => {
                    self.sub(dest, imm, src0, src1);
                    self.inc_pc();
                }
                EQOT(dest, imm, src0, src1) => {
                    self.eqot(dest, imm, src0, src1);
                    self.inc_pc();
                }
                EREM(dest, imm, src0, src1) => {
                    self.erem(dest, imm, src0, src1);
                    self.inc_pc();
                }
                NOT(dest, src) => {
                    self.not(dest, src);
                    self.inc_pc();
                }
                LSH(dest, src, count) => {
                    self.lsh(dest, src, count);
                    self.inc_pc();
                }
                RSH(dest, src, count) => {
                    self.rsh(dest, src, count);
                    self.inc_pc();
                }
                ANDR(dest, src0, src1) => {
                    self.and_r(dest, src0, src1);
                    self.inc_pc();
                }
                ORR(dest, src0, src1) => {
                    self.or_r(dest, src0, src1);
                    self.inc_pc();
                }
                ROTR(dest, src0, src1) => {
                    self.rot_r(dest, src0, src1);
                    self.inc_pc();
                }
                ROTI(dest, src0, num) => {
                    self.rot_i(dest, src0, num);
                    self.inc_pc();
                }
                PUSHR3(r0, r1, r2) => {
                    self.push_r3(r0, r1, r2);
                    self.inc_pc();
                }
                PUSHIMWORD(imm) => {
                    self.push_im_word(imm);
                    self.inc_pc();
                }
                PUSHIMTRYTE(imm) => {
                    self.push_im_tryte(imm);
                    self.inc_pc();
                }
                PUSHMEM(r0, r1, r2, imm) => {
                    self.push_mem(r0, r1, r2, imm);
                    self.inc_pc();
                }
                POP(dest) => {
                    self.pop(dest);
                    self.inc_pc();
                }
                CMP(r0, r1) => {
                    self.cmp(r0, r1);
                    self.inc_pc();
                }
                SPT(r) => {
                    self.spt(r);
                    self.inc_pc();
                }
                SST(r) => {
                    self.sst(r);
                    self.inc_pc();
                }
                BRR(r0, r1, r2) => {
                    self.br_r(r0, r1, r2);
                }
                BRI(imm) => {
                    self.br_i(imm);
                }
                BRM(r0, r1, r2, imm) => {
                    self.br_m(r0, r1, r2, imm);
                }
                BNER(r0, r1, r2) => {
                    self.bne_r(r0, r1, r2);
                }
                BNEI(imm) => {
                    self.bne_i(imm);
                }
                BNEM(r0, r1, r2, imm) => {
                    self.bne_m(r0, r1, r2, imm);
                }
                BGTR(r0, r1, r2) => {
                    self.bgt_r(r0, r1, r2);
                }
                BGTI(imm) => {
                    self.bgt_i(imm);
                }
                BGTM(r0, r1, r2, imm) => {
                    self.bgt_m(r0, r1, r2, imm);
                }
                BLTR(r0, r1, r2) => {
                    self.blt_r(r0, r1, r2);
                }
                BLTI(imm) => {
                    self.blt_i(imm);
                }
                BLTM(r0, r1, r2, imm) => {
                    self.blt_m(r0, r1, r2, imm);
                }
                BEQR(r0, r1, r2) => {
                    self.beq_r(r0, r1, r2);
                }
                BEQI(imm) => {
                    self.beq_i(imm);
                }
                BEQM(r0, r1, r2, imm) => {
                    self.beq_m(r0, r1, r2, imm);
                }
                BGEQR(r0, r1, r2) => {
                    self.bgeq_r(r0, r1, r2);
                }
                BGEQI(imm) => {
                    self.bgeq_i(imm);
                }
                BGEQM(r0, r1, r2, imm) => {
                    self.bgeq_m(r0, r1, r2, imm);
                }
                BLEQR(r0, r1, r2) => {
                    self.bleq_r(r0, r1, r2);
                }
                BLEQI(imm) => {
                    self.bleq_i(imm);
                }
                BLEQM(r0, r1, r2, imm) => {
                    self.bleq_m(r0, r1, r2, imm);
                }
                BOFNR(r0, r1, r2) => {
                    self.bofn_r(r0, r1, r2);
                }
                BOFNI(imm) => {
                    self.bofn_i(imm);
                }
                BOFNM(r0, r1, r2, imm) => {
                    self.bofn_m(r0, r1, r2, imm);
                }
                BOFZR(r0, r1, r2) => {
                    self.bofz_r(r0, r1, r2);
                }
                BOFZI(imm) => {
                    self.bofz_i(imm);
                }
                BOFZM(r0, r1, r2, imm) => {
                    self.bofz_m(r0, r1, r2, imm);
                }
                BOFPR(r0, r1, r2) => {
                    self.bofp_r(r0, r1, r2);
                }
                BOFPI(imm) => {
                    self.bofp_i(imm);
                }
                BOFPM(r0, r1, r2, imm) => {
                    self.bofp_m(r0, r1, r2, imm);
                }
                BPNR(r0, r1, r2) => {
                    self.bpn_r(r0, r1, r2);
                }
                BPNI(imm) => {
                    self.bpn_i(imm);
                }
                BPNM(r0, r1, r2, imm) => {
                    self.bpn_m(r0, r1, r2, imm);
                }
                BPZR(r0, r1, r2) => {
                    self.bpz_r(r0, r1, r2);
                }
                BPZI(imm) => {
                    self.bpz_i(imm);
                }
                BPZM(r0, r1, r2, imm) => {
                    self.bpz_m(r0, r1, r2, imm);
                }
                BPPR(r0, r1, r2) => {
                    self.bpp_r(r0, r1, r2);
                }
                BPPI(imm) => {
                    self.bpp_i(imm);
                }
                BPPM(r0, r1, r2, imm) => {
                    self.bpp_m(r0, r1, r2, imm);
                }
                INR(dest, loc) => {
                    self.in_r(dest, loc);
                    self.inc_pc();
                }
                OUTR(dest, loc) => {
                    self.out_r(dest, loc);
                    self.inc_pc();
                }
                OUTI(dest, loc) => {
                    self.out_i(dest, loc);
                    self.inc_pc();
                }
            }
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
            [L, R, [t, _, _], d, r, _, i0, i1, i2] => {
                Instruction::LDRI((t, d).into(), (t, r).into(), [i0, i1, i2].into())
            }
            [L, I, [t, _, _], d, r0, r1, ZERO, ZERO, ZERO] => {
                Instruction::LDRR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [L, B, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::LDRRI(
                (t, d).into(),
                (t, r0).into(),
                (t, r1).into(),
                [i0, i1, i2].into(),
            ),
            [L, P, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::LDRPCI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),
            // STORE
            [S, R, [t, _, _], d, r, _, i0, i1, i2] => {
                Instruction::STRI((t, d).into(), (t, r).into(), [i0, i1, i2].into())
            }
            [S, I, [t, _, _], d, r0, r1, ZERO, ZERO, ZERO] => {
                Instruction::STRR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [S, B, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::STRRI(
                (t, d).into(),
                (t, r0).into(),
                (t, r1).into(),
                [i0, i1, i2].into(),
            ),
            [S, P, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::STRPCI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),
            // MOV
            [M, I, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::MOVRI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),
            [M, R, [t, _, _], r0, r1, _, _, _, _] => {
                Instruction::MOVRR((t, r0).into(), (t, r1).into())
            }
            // BIT
            [O, W, O, i0, i1, i2, d, r, ZERO] => Instruction::OWO(todo!(), todo!(), todo!()),
            [U, W, U, i0, i1, i2, d, r, ZERO] => Instruction::UWU(todo!(), todo!(), todo!()),
            // ALU
            [A, D, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::ADD(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [M, U, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::MUL(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [S, U, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::SUB(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [E, Q, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::EQOT(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            [E, R, [t, _, _], i0, i1, i2, d, r0, r1] => Instruction::EREM(
                (t, d).into(),
                [i0, i1, i2].into(),
                (t, r0).into(),
                (t, r1).into(),
            ),
            // BIT
            [N, O, [t, _, _], i0, i1, i2, d, r, _] => {
                Instruction::NOT((t, d).into(), (t, r).into())
            }
            [L, S, [t, _, _], i0, i1, i2, d, r0, r1] => {
                Instruction::LSH((t, d).into(), (t, r0).into(), [i0, i1, i2].into())
            }
            [R, S, [t, _, _], i0, i1, i2, d, r0, r1] => {
                Instruction::RSH((t, d).into(), (t, r0).into(), [i0, i1, i2].into())
            }
            [A, N, [t, _, _], i0, i1, i2, d, r0, r1] => {
                Instruction::ANDR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [O, R, [t, _, _], i0, i1, i2, d, r0, r1] => {
                Instruction::ORR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [R, O, [t, Trit::POne, _], i0, i1, i2, d, r0, r1] => {
                Instruction::ROTR((t, d).into(), (t, r0).into(), (t, r1).into())
            }
            [R, O, [t, Trit::NOne, _], i0, i1, i2, d, r0, r1] => {
                Instruction::ROTI((t, d).into(), (t, r0).into(), todo!())
            }
            // Stack
            [P, R, [t, _, _], r0, r1, r2, _, _, _] => {
                Instruction::PUSHR3((t, r0).into(), (t, r1).into(), (t, r2).into())
            }
            [P, I, [t, _, _], i0, i1, i2, i3, i4, i5] => {
                Instruction::PUSHIMWORD([i0, i1, i2, i3, i4, i5, ZERO, ZERO, ZERO].into())
            }
            [P, T, [t, _, _], i0, i1, i2, ZERO, ZERO, ZERO] => {
                Instruction::PUSHIMTRYTE([i0, i1, i2].into())
            }
            [P, M, [t, _, _], d, r0, r1, i0, i1, i2] => Instruction::PUSHMEM(
                (t, d).into(),
                (t, r0).into(),
                (t, r1).into(),
                [i0, i1, i2].into(),
            ),
            [P, P, [t, _, _], d, _, _, _, _, _] => Instruction::POP((t, d).into()),
            [C, P, [t, _, _], r0, r1, _, _, _, _] => {
                Instruction::CMP((t, r0).into(), (t, r1).into())
            }
            [C, M, [t, _, _], r, _, _, _, _, _] => Instruction::SPT((t, r).into()),
            [C, S, [t, _, _], r, _, _, _, _, _] => Instruction::SST((t, r).into()),
            [B, ZERO, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, A, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, B, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, C, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, D, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, E, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, F, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, G, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, H, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, I, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, J, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, K, [t, h, _], a, b, c, d, e, f] => match h {
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
            [B, L, [t, h, _], a, b, c, d, e, f] => match h {
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
            [I, R, [t, _, _], r0, r1, _, _, _, _] => {
                Instruction::INR((t, r0).into(), (t, r1).into())
            }
            [O, R, [t, _, _], r0, r1, _, _, _, _] => {
                Instruction::OUTR((t, r0).into(), (t, r1).into())
            }
            [O, I, [t, _, _], r, i0, i1, i2, i3, i4] => Instruction::OUTI(
                (t, r).into(),
                [i0, i1, i2, i3, i4, ZERO, ZERO, ZERO, ZERO].into(),
            ),

            _ => panic!("illegal instruction (interrupts not implemented yet)"),
        }
    }
}
