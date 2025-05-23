use super::registers::Register;
use super::Cpu;
use crate::cpu::jt1701isa::{jt1701, Instruction};
use crate::cpu::registers::SP_WORD;
use crate::septivigntimal::*;
use ternary::{tryte::Tryte, word::Word, trits::Trit};

impl Cpu {
    // (fetch/execute)
    fn fexecute(&mut self) {
        'main: loop {
            use Instruction::*;
            let instruction = self.stack.get_word(self.program_counter);
            let instr = instruction.into();
            match instr {
                LHT(register) => {
                    self.lht(register);
                    self.inc_pc();
                }
                HLT => {
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
                    // returns to interrupt
                    self.rti();
                    // doesn't call int again because that would be dumb
                    self.inc_pc();
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
                INR(dest, port) => {
                    self.in_r(dest, port);
                    self.inc_pc();
                }
                INI(dest, port) => {
                    self.in_i(dest, port);
                    self.inc_pc();
                }
                OUTR(dest, port) => {
                    self.out_r(dest, port);
                    self.inc_pc();
                }
                OUTI(dest, port) => {
                    self.out_i(dest, port);
                    self.inc_pc();
                }
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{cpu::{consts::*, jt1701isa::{self, jt1701, Instruction}, registers::{BP_WORD, SP_WORD}, Cpu}, septivigntimal::*};
    use ternary::{trits::Trit, tryte::Tryte, word::{consts::THREE_WORD, Word}};

    #[test]
    fn fedeex() {
        use super::Instruction::*;
        let mut cpu = Cpu::default();
        cpu.program_counter = Word::default();
        // 0  mov %RN11, 1
        // 3  mov %RN13, n
        // 6  mov %RN12, 1
        // 9  cmp %RN13, %RN11
        // 12 bleqi 24
        // 15 mul %RN12, %RN12, %RN13
        // 18 add %RN13, %RN13, %R0, -1
        // 21 bri 9
        // 24 hlt
        let instrs = vec![
            // 1
            MOVRI(TN11, [[Trit::POne, Trit::Zero, Trit::Zero], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            // 6
            MOVRI(TN13, [[Trit::Zero, Trit::Zero, Trit::POne], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            // 1
            MOVRI(TN12, [[Trit::POne, Trit::Zero, Trit::Zero], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            CMP(TN13, TN11),
            // 24
            BLEQI([[Trit::Zero, Trit::NOne, Trit::Zero], [Trit::POne, Trit::Zero, Trit::Zero], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            MUL(RN12, Tryte::default(), RN12, RN13),
            ADD(RN13, [[Trit::NOne, Trit::Zero, Trit::Zero], ZERO, ZERO].into(), RN13, R0),
            BRI([[Trit::Zero, Trit::Zero, Trit::POne], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            HLT
        ];

        let mut loc = cpu.program_counter;
        for i in &instrs {
            cpu.stack.insert_word(loc, (*i).into());
            loc = (loc + THREE_WORD).result;
        }

        cpu.fexecute();
        let val: isize = cpu.register_file.get_word(RN12).into();
        println!("{val:?}");
    }

    #[test]
    fn stack_test() {
        use super::Instruction::*;
        let mut cpu = Cpu::default();
        cpu.program_counter = Word::default();
        // (8 + 9) * 5 => 8 9 + 5 * = 85
        // 0  pushim 8
        // 3  pushim 9
        // 6  pushim 5
        // 9  pop %RN13
        // 12 pop %RN12
        // 15 add %RN11, %RN13, %RN12, 0
        // 18 pop %RN10
        // 21 mul %RN9, %RN11, %RN10
        // 24 hlt
        let instrs = vec![
            PUSHIMWORD([[Trit::NOne, Trit::Zero, Trit::POne], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            PUSHIMWORD([[Trit::Zero, Trit::Zero, Trit::POne], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            POP(RN13),
            POP(RN12),
            ADD(RN11, Tryte::default(), RN12, RN13),
            PUSHIMWORD([[Trit::NOne, Trit::NOne, Trit::POne], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into()),
            POP(RN10),
            MUL(RN9, Tryte::default(), RN10, RN11),
            HLT
        ];

        cpu.register_file.set_value(SP_WORD, [Trit::NOne; 27].into());

        let mut loc = cpu.program_counter;
        for i in &instrs {
            cpu.stack.insert_word(loc, (*i).into());
            loc = (loc + THREE_WORD).result;
        }

        cpu.fexecute();
        for (i, reg) in [RN13, RN12, RN11, RN10, RN9].into_iter().enumerate() {
            let val: isize = cpu.register_file.get_word(reg).into();
            println!("result: {val:?}");
        }
    }

    #[test]
    fn function_test() {
        use super::Instruction::*;
        let mut cpu = Cpu::default();
        cpu.program_counter = Word::default();
        /// 0  mov %RN13, 6
        ///    ; (call fact)
        /// 3  push 60 ; push continue address to stack
        /// 6  b fact
        ///    ; fact(7)
        ///    ; // ARG: RN13
        ///    ; // RET: RN12
        ///    fact:
        /// 9      push %BP
        /// 12     mov  %BP, %SP
        /// 15     mov  %RN11, 2
        /// 18     cmp  %RN13, %RN11
        /// 21     bgti calcs
        /// 24     mov  %RN12, 2
        /// 27     b return
        ///    calcs:
        ///        ; move argument onto stack
        /// 30     push %RN13
        ///        ; subtract one from argument
        /// 33     sub  %RN13, 1
        /// 36     push 42 ; push continue address to stack
        ///        ; addr will be popped off by return
        ///        ; call again
        /// 39     b fact ; (call fact)
        ///        ; value will be in %RN12
        /// 42     pop %RN10
        ///        ; multiply return value with value on the stack
        /// 45     mul %RN12, %RN12, %RN10
        ///    return:
        /// 48     pop  %BP
        ///        ; pop return address into %RN10
        /// 51     pop  %RN10
        /// 54     mov  %R4, 1
        /// 57     b %RN10 ; * R4
        ///
        /// 60 hlt
        let instrs = vec![
            MOVRI(RN13, 6.into()),
            PUSHIMWORD(60.into()),
            BRI(9.into()),
            PUSHR3(BP_WORD, R0, R0),
            MOVRR(BP_WORD, SP_WORD),
            MOVRI(RN11, 2.into()),
            CMP(RN13, RN11),
            // We want it to branch if we still need to recursively call
            // We were BGEQ with 2, which was bad because that does an extra multiply by 2
            BGTI(30.into()),
            MOVRI(RN12, 2.into()),
            BRI(48.into()),
            PUSHR3(RN13, R0, R0),
            SUB(RN13, (1).into(), RN13, R0),
            PUSHIMWORD(42.into()),
            BRI(9.into()),
            POP(RN10),
            MUL(RN12, 0.into(), RN12, RN10),
            POP(BP_WORD),
            POP(RN10),
            // (r0 + r1) * r2
            MOVRI(R4, 1.into()),
            BRR(RN10, R0, R4),
            HLT
        ];


        cpu.register_file.set_value(SP_WORD, [Trit::NOne; 27].into());

        let mut loc = cpu.program_counter;
        for i in &instrs {
            cpu.stack.insert_word(loc, (*i).into());
            loc = (loc + THREE_WORD).result;
        }

        cpu.fexecute();
        assert_eq!(6isize*5*4*3*2*1, cpu.register_file.get_word(RN12).into());
        for (i, reg) in [RN13, RN12, RN11, RN10, RN9].into_iter().enumerate() {
            let val: isize = cpu.register_file.get_word(reg).into();
            println!("result: {val:?}");
        }

    }
}
