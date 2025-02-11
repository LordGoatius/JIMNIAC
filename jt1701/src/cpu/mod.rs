#![expect(unused)]
use std::ops::{BitAnd, Not, Shl};

use errors::CpuError;
use jt1701isa::jt1701;
use registers::{BimapEitherOps, EitherAddResult, MapResult, Register, RegisterFile, WordOrTryte, SP_TRYTE, SP_WORD};
use statusword::StatusWord;

use crate::{
    GetStatus,
    stack::Stack,
    trits::Trit,
    tryte::Tryte,
    word::{consts::{ONE_WORD, THREE_WORD}, Word},
};

use itertools::{Either, Either::{Left, Right}};

pub mod errors;
pub mod jt1701isa;
pub mod macros;
pub mod registers;
pub mod statusword;

#[derive(Default, Debug)]
pub struct Cpu {
    // Done
    register_file: RegisterFile,
    program_counter: Word,
    // Partial, can update as needed
    cpu_state_reg: StatusWord,
    saved_prog_status_reg: StatusWord,
    interrupt_vector: Option<Tryte>,
    // Done
    stack: Stack,
}

impl Cpu {
    pub fn run_program(&mut self, program: Vec<Tryte>) -> Result<(), CpuError> {
        self.copy_program_to_stack(Word([Trit::NOne; 27]), program);
        todo!()
    }

    fn copy_program_to_stack(&mut self, begin: Word, program: Vec<Tryte>) {
        let mut addr = begin;

        for tryte in program {
            self.stack.insert(addr, tryte);
            addr = (addr + Trit::POne).result;
        }
    }
}

impl jt1701 for Cpu {
    //==== CPU ====//
    /// Load Interrupt Handler Table
    fn lht(&mut self, register: Register) {
        if let Right(tryte) = self.register_file.get_value(register) {
            self.cpu_state_reg.set_interrupt_vector(tryte);
            return;
        } 
        panic!("deal with later")
    }
    /// Halt
    fn hlt(&mut self) {
        return;
    }
    // TODO
    /// Interrupt
    fn int(&mut self, interrupt: Tryte) {
        todo!()
    }
    /// No Op
    fn nop(&self) {
        return;
    }
    // TODO
    /// Wait For Interrupt
    fn wfi(&mut self) {
        todo!()
    }
    /// Stop Interrupts
    fn sti(&mut self) {
        self.cpu_state_reg.set_interrupt_enable(Trit::NOne);
    }
    /// Begin Interrupts
    fn bti(&mut self) {
        self.cpu_state_reg.set_interrupt_enable(Trit::POne);
    }
    // TODO
    /// Returns from interrupt and restores status register
    fn rti(&mut self) {
        todo!()
    }

    //== Loading Register from Memory ==//
    /// dest = *(src + imm)
    fn ldri(&mut self, dest: Register, src: Register, imm: Tryte) {
        let val: Word = (self.register_file.get_word(src) + imm).result;

        self.register_file.set_value(dest, self.stack.get_word(val));
    }
    /// dest = *(src0 + src1)
    fn ldrr(&mut self, dest: Register, src0: Register, src1: Register) {
        let src0: Word = self.register_file.get_word(src0);
        let src1: Word = self.register_file.get_word(src1);
        self.register_file.set_value(dest, self.stack.get_word((src0 + src1).result));
    }
    /// dest = *(src0 + src1 + imm)
    fn ldrri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte) {
        let src0: Word = self.register_file.get_word(src0);
        let src1: Word = self.register_file.get_word(src1);
        self.register_file.set_value(dest, self.stack.get_word(((src0 + src1).result + <Tryte as Into<Word>>::into(imm)).result));
    }
    /// Word should have most sig. tryte be 0.
    /// dest = *(pc + imm)
    fn ldrpci(&mut self, dest: Register, imm: Word) {
        let val: Word = (self.program_counter + imm).result;
        self.register_file.set_value(dest, self.stack.get_word(val));
    }

    //== Storing Register to Memory ==//
    /// *(src + imm) = dest
    fn stri(&mut self, dest: Register, src: Register, imm: Tryte) {
        let addr: Word = (self.register_file.get_word(src) + imm).result;

        match dest.size {
            WordOrTryte::Word => self.stack.insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self.stack.insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }
    }
    /// *(src0 + src1) = dest
    fn strr(&mut self, dest: Register, src0: Register, src1: Register) {
        let addr: Word = (self.register_file.get_word(src0) + self.register_file.get_word(src1)).result;

        match dest.size {
            WordOrTryte::Word => self.stack.insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self.stack.insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }

    }
    /// *(src0 + src1 + imm) = dest
    fn strri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte) {
        let addr: Word = (self.register_file.get_word(src0) + self.register_file.get_word(src1)).result;
        let addr: Word = (addr + imm).result;

        match dest.size {
            WordOrTryte::Word => self.stack.insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self.stack.insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }

    }
    /// Word should have most sig. tryte be 0.
    /// *(pc + imm) = dest
    fn strpci(&mut self, dest: Register, imm: Word) {
        let addr: Word = (self.program_counter + imm).result;

        match dest.size {
            WordOrTryte::Word => self.stack.insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self.stack.insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }
    }

    //== Moving ==//
    fn movrr(&mut self, dest: Register, src: Register) {
        self.register_file.set_value(dest, self.register_file.get_word(src));
    }
    /// imm will always be 2 trytes long, or 18 trits.
    /// To move a larger size, the assembler will create shifts and adds in order to do so
    fn movri(&mut self, dest: Register, imm: Word) {
        self.register_file.set_value(dest, imm);
    }

    //==== ALU ====//
    // owo/uwu
    // 0..=2: op
    // 3..=5: ___: #imm
    // 6..=8: %d, %s, %z
    fn owo(&mut self, imm: Tryte, dest: Register, src: Register) {
        todo!()
    }
    fn uwu(&mut self, imm: Tryte, dest: Register, src: Register) {
        todo!()
    }

    /// 0..=2: __:op _: control tryte (first rep reg type, second condition, third ???)
    /// 3..=5: ___: #imm
    /// 6..=8: %d, %s1, %s2
    /// d = s0 + (s1 + imm)
    fn add(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        let tmp = self.register_file.get_value(src1).bimap_add_tryte(imm).mapres();
        let val = (self.register_file.get_value(src0).bimap_add(tmp)).bubbleres();
        let EitherAddResult { result, carry } = val;

        self.cpu_state_reg.set_carry_flag(carry);
        self.cpu_state_reg.set_sign_flag(result.get_sign());
        self.cpu_state_reg.set_parity_flag(result.get_parity());
        self.register_file.set_value_either(dest, result);
    }

    /// d = s0 * (s1 + imm)
    fn mul(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        let tmp = self.register_file.get_value(src1).bimap_add_tryte(imm).mapres();
        let val = (self.register_file.get_value(src0).bimap_mul(tmp));

        self.cpu_state_reg.set_carry_flag(Trit::Zero);
        self.cpu_state_reg.set_sign_flag(val.get_sign());
        self.cpu_state_reg.set_parity_flag(val.get_parity());

        self.register_file.set_value_either(dest, val);
    }

    /// d = s0 - (s1 + imm)
    fn sub(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        let tmp = self.register_file.get_value(src1).bimap_add_tryte(imm).mapres();
        let val = (self.register_file.get_value(src0).bimap_sub(tmp)).bubbleres();
        let EitherAddResult { result, carry } = val;

        self.cpu_state_reg.set_carry_flag(carry);
        self.cpu_state_reg.set_sign_flag(result.get_sign());
        self.cpu_state_reg.set_parity_flag(result.get_parity());

        self.register_file.set_value_either(dest, result);
    }

    /// d = s0 / (s1 + imm)
    fn eqot(
        &mut self,
        dest: Register,
        imm: Tryte,
        src0: Register,
        src1: Register,
    ) -> Result<(), CpuError> {
        let tmp = self.register_file.get_value(src1).bimap_add_tryte(imm).mapres();
        let val = (self.register_file.get_value(src0).bimap_div(tmp))?;

        self.cpu_state_reg.set_carry_flag(Trit::Zero);
        self.cpu_state_reg.set_sign_flag(val.get_sign());
        self.cpu_state_reg.set_parity_flag(val.get_parity());
        self.register_file.set_value_either(dest, val);
        Ok(())
    }

    /// d = s0 % (s1 + imm)
    fn erem(
        &mut self,
        dest: Register,
        imm: Tryte,
        src0: Register,
        src1: Register,
    ) -> Result<(), CpuError> {
        let tmp = self.register_file.get_value(src1).bimap_add_tryte(imm).mapres();
        let val = (self.register_file.get_value(src0).bimap_mod(tmp))?;

        self.cpu_state_reg.set_carry_flag(Trit::Zero);
        self.cpu_state_reg.set_sign_flag(val.get_sign());
        self.cpu_state_reg.set_parity_flag(val.get_parity());
        self.register_file.set_value_either(dest, val);
        Ok(())
    }

    //=== Trit ===//
    /// d = ~s
    fn not(&mut self, dest: Register, src: Register) {
        self.register_file.set_value_either(dest, self.register_file.get_value(src).map_either(Not::not, Not::not));
    }

    // NOTE: Should left and right be different?
    // FIXME: This can be bugged
    fn lsh(&mut self, dest: Register, src: Register, count: Tryte) {
        let count = <Tryte as Into<isize>>::into(count) as usize;
        self.register_file.set_value_either(dest, self.register_file.get_value(src).map_either(|x| x << count, |x| x << count));
    }
    fn rsh(&mut self, dest: Register, src: Register, count: Tryte) {
        let count = <Tryte as Into<isize>>::into(count) as usize;
        self.register_file.set_value_either(dest, self.register_file.get_value(src).map_either(|x| x >> count, |x| x >> count));
    }

    fn and_r(&mut self, dest: Register, src0: Register, src1: Register) {
        self.register_file.set_value_either(dest, self.register_file.get_value(src0).bimap_and(self.register_file.get_value(src1)));
    }

    fn or_r(&mut self, dest: Register, src0: Register, src1: Register) {
        self.register_file.set_value_either(dest, self.register_file.get_value(src0).bimap_or(self.register_file.get_value(src1)));
    }

    // FIXME
    fn rot_r(&mut self, dest: Register, src0: Register, src1: Register) {
        todo!()
    }

    // FIXME
    fn rot_i(&mut self, dest: Register, src0: Register, mask: Word) {
        todo!()
    }

    //== Stack ==//
    /// (r0 + r1) * r2
    fn push_r3(&mut self, r0: Register, r1: Register, r2: Register) {
        let val = self.register_file.get_value(r0).bimap_add(self.register_file.get_value(r1)).bubbleres().result;
        let val = val.bimap_mul(self.register_file.get_value(r2));
        
        match val {
            Left(word) => {
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert_word(curr, word);
                self.register_file.set_value(SP_WORD, (curr + THREE_WORD).result);
            },
            Right(tryte) => {
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert(curr, tryte);
                self.register_file.set_value(SP_WORD, (curr + ONE_WORD).result);
            },
        }
    }

    fn push_im_word(&mut self, imm: Word) {
        let curr = self.register_file.get_word(SP_WORD);
        self.stack.insert_word(curr, imm);
        self.register_file.set_value(SP_WORD, (curr + THREE_WORD).result);
    }

    fn push_im_tryte(&mut self, imm: Tryte) {
        let curr = self.register_file.get_word(SP_WORD);
        self.stack.insert(curr, imm);
        self.register_file.set_value(SP_WORD, (curr + ONE_WORD).result);
    }

    /// *((r0 + r1) * (r2 + imm))
    /// IDK ABOUT THIS
    /// BUT
    /// IF THE REGISTERS ARE WORD REGS THEN IT PUSHES THE WORD IN MEMORY AT THE RESULT
    /// OTHERWISE IF ITS A TRYTE IT PUSHES A TRYTE
    fn push_mem(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let addr = self.register_file.get_value(r0).bimap_add(self.register_file.get_value(r1)).bubbleres().result;
        let mul = self.register_file.get_value(r2).bimap_add_tryte(imm).bubbleres().result;
        let addr = addr.bimap_mul(mul);

        match addr {
            Left(addr) => {
                let val = self.stack.get_word(addr);
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert_word(curr, val);
                self.register_file.set_value(SP_WORD, (curr + THREE_WORD).result);
            },
            Right(addr) => {
                let val = *self.stack.get(addr.into());
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert(curr, val);
                self.register_file.set_value(SP_WORD, (curr + ONE_WORD).result);
            },
        }
    }

    //== Branch ==//
    /// Compare 2 registers
    fn cmp(&mut self, r0: Register, r1: Register) {
        todo!()
    }
    /// Set Parity Trit
    fn spt(&mut self, r: Register) {
        self.cpu_state_reg.set_parity_flag(self.register_file.get_value(r).get_parity());
    }
    /// Set SignTrit
    fn sst(&mut self, r: Register) {
        self.cpu_state_reg.set_parity_flag(self.register_file.get_value(r).get_sign());
    }

    /// (r0 + r1) * r2
    fn br_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn br_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn br_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bne_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bne_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bne_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bgt_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bgt_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bgt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn blt_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn blt_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn blt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn beq_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn beq_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn beq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bgeq_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bgeq_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bgeq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bleq_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bleq_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bleq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bofn_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bofn_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bofn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bofz_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bofz_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bofz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bofp_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bofp_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bofp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bpn_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bpn_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bpn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bpz_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bpz_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bpz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    /// (r0 + r1) * r2
    fn bpp_r(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn bpp_i(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bpp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    //== Ports ==//
    fn in_r(&mut self, dest: Register, loc: Register) {
        todo!()
    }
    fn in_i(&mut self, dest: Register, loc: Word) {
        todo!()
    }

    fn out_r(&mut self, dest: Register, loc: Register) {
        todo!()
    }
    fn out_i(&mut self, dest: Register, loc: Word) {
        todo!()
    }
}
