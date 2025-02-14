#![expect(unused)]
use std::ops::{BitAnd, Not, Shl};

use jt1701isa::jt1701;
use ports::Ports;
use registers::{
    BimapEitherOps, EitherAddResult, MapResult, Register, RegisterFile, WordOrTryte, SP_TRYTE,
    SP_WORD,
};
use statusword::StatusWord;

use ternary::{
    errors::DivByZeroError, trits::Trit, tryte::Tryte, word::Word
};

use crate::{
    word::{ONE_WORD, THREE_WORD},
    tryte::{ONE_TRYTE, THREE_TRYTE},
    septivigntimal::*, stack::Stack, word::{
    }, GetStatus
};

use itertools::{
    Either,
    Either::{Left, Right},
};

pub mod errors;
pub mod fedeex;
pub mod macros;
pub mod consts;
pub mod ports;
pub mod jt1701isa;
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
    // TODO
    ports: Ports,
}

// impl Default for Cpu {
//     fn default() -> Self {
//         Cpu {
//             ..Default::default()
//         }
//     }
// }

impl Cpu {
    fn copy_program_to_stack(&mut self, begin: Word, program: Vec<Tryte>) {
        let mut addr = begin;

        for tryte in program {
            self.stack.insert(addr, tryte);
            addr = (addr + Trit::POne).result;
        }
    }

    fn inc_pc(&mut self) {
        self.program_counter = (self.program_counter + THREE_WORD).result;
    }
}

impl jt1701 for Cpu {
    //==== CPU ====//
    /// Load Interrupt Handler Table
    fn lht(&mut self, register: Register) {
        match  self.register_file.get_value(register) {
            Left(word) => {
                let word: [Tryte; 3] = word.into();
                self.cpu_state_reg.set_interrupt_vector(word[0]);
            },
            Right(tryte) => {
                self.cpu_state_reg.set_interrupt_vector(tryte);
            }
        }
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
        let addr = self
            .register_file
            .get_value(src)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result
            .as_word();

        match dest.size {
            WordOrTryte::Word => {
                let val = self.stack.get_word(addr);
                self.register_file.set_value_either(dest, Left(val));
            }
            WordOrTryte::Tryte => {
                let val = *self.stack.get(addr);
                self.register_file.set_value_either(dest, Right(val));
            }
        }
    }

    /// dest = *(src0 + src1)
    fn ldrr(&mut self, dest: Register, src0: Register, src1: Register) {
        let addr = self
            .register_file
            .get_value(src0)
            .bimap_add(self.register_file.get_value(src1))
            .bubbleres()
            .result
            .as_word();

        match dest.size {
            WordOrTryte::Word => {
                let val = self.stack.get_word(addr);
                self.register_file.set_value_either(dest, Left(val));
            }
            WordOrTryte::Tryte => {
                let val = *self.stack.get(addr);
                self.register_file.set_value_either(dest, Right(val));
            }
        }
    }
    /// dest = *(src0 + src1 + imm)
    fn ldrri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte) {
        let addr = self
            .register_file
            .get_value(src0)
            .bimap_add(self.register_file.get_value(src1))
            .bubbleres()
            .result
            .bimap_add_tryte(imm)
            .bubbleres()
            .result
            .as_word();

        match dest.size {
            WordOrTryte::Word => {
                let val = self.stack.get_word(addr);
                self.register_file.set_value_either(dest, Left(val));
            }
            WordOrTryte::Tryte => {
                let val = *self.stack.get(addr);
                self.register_file.set_value_either(dest, Right(val));
            }
        }
    }
    /// Word should have most sig. tryte be 0.
    /// dest = *(pc + imm)
    fn ldrpci(&mut self, dest: Register, imm: Word) {
        let addr: Word = (self.program_counter + imm).result;

        match dest.size {
            WordOrTryte::Word => {
                let val = self.stack.get_word(addr);
                self.register_file.set_value_either(dest, Left(val));
            }
            WordOrTryte::Tryte => {
                let val = *self.stack.get(addr);
                self.register_file.set_value_either(dest, Right(val));
            }
        }
    }

    //== Storing Register to Memory ==//
    /// *(src + imm) = dest
    fn stri(&mut self, dest: Register, src: Register, imm: Tryte) {
        let addr = self
            .register_file
            .get_value(src)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result
            .as_word();

        match dest.size {
            WordOrTryte::Word => self
                .stack
                .insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self
                .stack
                .insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }
    }
    /// *(src0 + src1) = dest
    fn strr(&mut self, dest: Register, src0: Register, src1: Register) {
        let addr = self
            .register_file
            .get_value(src0)
            .bimap_add(self.register_file.get_value(src1))
            .bubbleres()
            .result
            .as_word();

        match dest.size {
            WordOrTryte::Word => self
                .stack
                .insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self
                .stack
                .insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }
    }
    /// *(src0 + src1 + imm) = dest
    fn strri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte) {
        let addr = self
            .register_file
            .get_value(src0)
            .bimap_add(self.register_file.get_value(src1))
            .bubbleres()
            .result
            .bimap_add_tryte(imm)
            .bubbleres()
            .result
            .as_word();

        match dest.size {
            WordOrTryte::Word => self
                .stack
                .insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self
                .stack
                .insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }
    }
    /// Word should have most sig. tryte be 0.
    /// *(pc + imm) = dest
    fn strpci(&mut self, dest: Register, imm: Word) {
        let addr: Word = (self.program_counter + imm).result;

        match dest.size {
            WordOrTryte::Word => self
                .stack
                .insert_word(addr, self.register_file.get_value(dest).unwrap_left()),
            WordOrTryte::Tryte => self
                .stack
                .insert(addr, self.register_file.get_value(dest).unwrap_right()),
        }
    }

    //== Moving ==//
    fn movrr(&mut self, dest: Register, src: Register) {
        self.register_file
            .set_value_either(dest, self.register_file.get_value(src));
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
        let mut owo: Either<Word, Tryte> = match src.size {
            WordOrTryte::Word => Left([O, W, O, W, O, W, O, W, O].into()),
            WordOrTryte::Tryte => Right([O, W, O].into())
        };
        owo = owo.bimap_add_tryte(imm).bubbleres().result;

        let val =self.register_file.get_value(src).bimap_and(owo);
        self.register_file.set_value_either(dest, val);
    }

    fn uwu(&mut self, imm: Tryte, dest: Register, src: Register) {
        let mut uwu: Either<Word, Tryte> = match src.size {
            WordOrTryte::Word => Left([U, W, U, W, U, W, U, W, U].into()),
            WordOrTryte::Tryte => Right([U, W, U].into())
        };
        uwu = uwu.bimap_add_tryte(imm).bubbleres().result;

        let val =self.register_file.get_value(src).bimap_and(uwu);
        self.register_file.set_value_either(dest, val);
    }

    /// 0..=2: __:op _: control tryte (first rep reg type, second condition, third ???)
    /// 3..=5: ___: #imm
    /// 6..=8: %d, %s1, %s2
    /// d = s0 + (s1 + imm)
    fn add(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        let tmp = self
            .register_file
            .get_value(src1)
            .bimap_add_tryte(imm)
            .mapres();
        let val = (self.register_file.get_value(src0).bimap_add(tmp)).bubbleres();
        let EitherAddResult { result, carry } = val;

        self.cpu_state_reg.set_carry_flag(carry);
        self.cpu_state_reg.set_sign_flag(result.get_sign());
        self.cpu_state_reg.set_parity_flag(result.get_parity());
        self.register_file.set_value_either(dest, result);
    }

    /// d = s0 * (s1 + imm)
    fn mul(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        let tmp = self
            .register_file
            .get_value(src1)
            .bimap_add_tryte(imm)
            .mapres();
        let val = (self.register_file.get_value(src0).bimap_mul(tmp));

        self.cpu_state_reg.set_carry_flag(Trit::Zero);
        self.cpu_state_reg.set_sign_flag(val.get_sign());
        self.cpu_state_reg.set_parity_flag(val.get_parity());

        self.register_file.set_value_either(dest, val);
    }

    /// d = s0 - (s1 + imm)
    fn sub(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        let tmp = self
            .register_file
            .get_value(src1)
            .bimap_add_tryte(imm)
            .mapres();
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
    ) -> Result<(), DivByZeroError> {
        let tmp = self
            .register_file
            .get_value(src1)
            .bimap_add_tryte(imm)
            .mapres();
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
    ) -> Result<(), DivByZeroError> {
        let tmp = self
            .register_file
            .get_value(src1)
            .bimap_add_tryte(imm)
            .mapres();
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
        self.register_file.set_value_either(
            dest,
            self.register_file
                .get_value(src)
                .map_either(Not::not, Not::not),
        );
    }

    // NOTE: Should left and right be different?
    // FIXME: This can be bugged
    /// NOTE: THIS IS VERY BUGGED
    fn lsh(&mut self, dest: Register, src: Register, count: Tryte) {
        let count = <Tryte as Into<isize>>::into(count).abs() as usize;
        self.register_file.set_value_either(
            dest,
            self.register_file
                .get_value(src)
                .map_either(|x| x << count, |x| x << count),
        );
    }

    // NOTE: Should left and right be different?
    // FIXME: This can be bugged
    /// NOTE: THIS IS VERY BUGGED
    fn rsh(&mut self, dest: Register, src: Register, count: Tryte) {
        let count = <Tryte as Into<isize>>::into(count) as usize;
        self.register_file.set_value_either(
            dest,
            self.register_file
                .get_value(src)
                .map_either(|x| x >> count, |x| x >> count),
        );
    }

    fn and_r(&mut self, dest: Register, src0: Register, src1: Register) {
        self.register_file.set_value_either(
            dest,
            self.register_file
                .get_value(src0)
                .bimap_and(self.register_file.get_value(src1)),
        );
    }

    fn or_r(&mut self, dest: Register, src0: Register, src1: Register) {
        self.register_file.set_value_either(
            dest,
            self.register_file
                .get_value(src0)
                .bimap_or(self.register_file.get_value(src1)),
        );
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
        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        match val {
            Left(word) => {
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert_word(curr, word);
                self.register_file
                    .set_value(SP_WORD, (curr + THREE_WORD).result);
            }
            Right(tryte) => {
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert(curr, tryte);
                self.register_file
                    .set_value(SP_WORD, (curr + ONE_WORD).result);
            }
        }
    }

    fn push_im_word(&mut self, imm: Word) {
        let curr = self.register_file.get_word(SP_WORD);
        self.stack.insert_word(curr, imm);
        self.register_file
            .set_value(SP_WORD, (curr + THREE_WORD).result);
    }

    fn push_im_tryte(&mut self, imm: Tryte) {
        let curr = self.register_file.get_word(SP_WORD);
        self.stack.insert(curr, imm);
        self.register_file
            .set_value(SP_WORD, (curr + ONE_WORD).result);
    }

    /// *((r0 + r1) * (r2 + imm))
    /// IDK ABOUT THIS
    /// BUT
    /// IF THE REGISTERS ARE WORD REGS THEN IT PUSHES THE WORD IN MEMORY AT THE RESULT
    /// OTHERWISE IF ITS A TRYTE IT PUSHES A TRYTE
    fn push_mem(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let addr = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let mul = self
            .register_file
            .get_value(r2)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let addr = addr.bimap_mul(mul);

        match addr {
            Left(addr) => {
                let val = self.stack.get_word(addr);
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert_word(curr, val);
                self.register_file
                    .set_value(SP_WORD, (curr + THREE_WORD).result);
            }
            Right(addr) => {
                let val = *self.stack.get(addr.into());
                let curr = self.register_file.get_word(SP_WORD);
                self.stack.insert(curr, val);
                self.register_file
                    .set_value(SP_WORD, (curr + ONE_WORD).result);
            }
        }
    }

    fn pop(&mut self, dest: Register) {
        match dest.size {
            WordOrTryte::Word => {
                let mut curr = self.register_file.get_word(SP_WORD);
                self.register_file.set_value(SP_WORD, (curr - THREE_WORD).result);
                curr = self.register_file.get_word(SP_WORD);
                let val = self.stack.get_word(curr);
                self.register_file.set_value(dest, val);
            }
            WordOrTryte::Tryte => {
                let mut curr = self.register_file.get_word(SP_WORD);
                self.register_file.set_value(SP_WORD, (curr - ONE_WORD).result);
                curr = self.register_file.get_word(SP_WORD);
                let val = *self.stack.get(curr);
                self.register_file.set_value_either(dest, Right(val));
            }
        }
    }

    //== Branch ==//
    /// Compare 2 registers
    fn cmp(&mut self, r0: Register, r1: Register) {
        let EitherAddResult { result, carry } = self
            .register_file
            .get_value(r0)
            .bimap_sub(self.register_file.get_value(r1))
            .bubbleres();
        self.cpu_state_reg.set_carry_flag(carry);
        self.cpu_state_reg.set_parity_flag(result.get_parity());
        self.cpu_state_reg.set_sign_flag(result.get_sign());
    }

    /// Set Parity Trit
    fn spt(&mut self, r: Register) {
        self.cpu_state_reg
            .set_parity_flag(self.register_file.get_value(r).get_parity());
    }

    /// Set SignTrit
    fn sst(&mut self, r: Register) {
        self.cpu_state_reg
            .set_sign_flag(self.register_file.get_value(r).get_sign());
    }

    /// (r0 + r1) * r2
    fn br_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn br_i(&mut self, imm: Word) {
        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn br_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bne_r(&mut self, r0: Register, r1: Register, r2: Register) {
        if (self.cpu_state_reg.get_sign_flag() == Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bne_i(&mut self, imm: Word) {
        if (self.cpu_state_reg.get_sign_flag() == Trit::Zero) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bne_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        if (self.cpu_state_reg.get_sign_flag() == Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bgt_r(&mut self, r0: Register, r1: Register, r2: Register) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::POne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bgt_i(&mut self, imm: Word) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::POne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bgt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::POne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn blt_r(&mut self, r0: Register, r1: Register, r2: Register) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn blt_i(&mut self, imm: Word) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::NOne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn blt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn beq_r(&mut self, r0: Register, r1: Register, r2: Register) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn beq_i(&mut self, imm: Word) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::Zero) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn beq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        if (self.cpu_state_reg.get_sign_flag() != Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bgeq_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_sign_flag();
        if (sign == Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bgeq_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_sign_flag();
        if (sign == Trit::NOne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bgeq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_sign_flag();
        if (sign == Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bleq_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_sign_flag();
        if (sign == Trit::POne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bleq_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_sign_flag();
        if (sign == Trit::POne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bleq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_sign_flag();
        if (sign == Trit::POne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bofn_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bofn_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::NOne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }
    /// *((r0 + r1) * (r2 + imm))
    fn bofn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bofz_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bofz_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::Zero) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bofz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bofp_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::POne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bofp_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::POne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bofp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_carry_flag();
        if (sign != Trit::POne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bpn_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bpn_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::NOne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bpn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::NOne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bpz_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bpz_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::Zero) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bpz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::Zero) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    /// (r0 + r1) * r2
    fn bpp_r(&mut self, r0: Register, r1: Register, r2: Register) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::POne) {
            self.inc_pc();
            return;
        }

        let val = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val = val.bimap_mul(self.register_file.get_value(r2));

        self.program_counter = val.as_word()
    }

    fn bpp_i(&mut self, imm: Word) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::POne) {
            self.inc_pc();
            return;
        }

        self.program_counter = imm;
    }

    /// *((r0 + r1) * (r2 + imm))
    fn bpp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        let sign = self.cpu_state_reg.get_parity_flag();
        if (sign != Trit::POne) {
            self.inc_pc();
            return;
        }

        let val0 = self
            .register_file
            .get_value(r0)
            .bimap_add(self.register_file.get_value(r1))
            .bubbleres()
            .result;
        let val1 = self
            .register_file
            .get_value(r1)
            .bimap_add_tryte(imm)
            .bubbleres()
            .result;
        let val = val0.bimap_mul(val1);

        let val = self.stack.get_word(val.as_word());

        self.program_counter = val;
    }

    // TODO

    //== Ports ==//
    fn in_r(&mut self, dest: Register, loc: Register) {
        todo!()
    }

    fn out_r(&mut self, dest: Register, loc: Register) {
        todo!()
    }

    fn out_i(&mut self, dest: Register, val: Word) {
        todo!()
    }
}

#[cfg(test)]
pub mod test {
    use crate::{cpu::{jt1701isa::jt1701, registers::SP_WORD}, tryte::{ONE_TRYTE, THREE_TRYTE}, word::{ONE_WORD, THREE_WORD}};

    #[test]
    fn test_cpu_instr() {
        use crate::cpu::jt1701isa::Instruction;
        use crate::cpu::{consts::*, Cpu};
        use crate::septivigntimal::*;
        use ternary::trits::Trit;
        use ternary::word::Word;
        use ternary::tryte::Tryte;
        use super::jt1701isa::Instruction::*;

        let word: Word = [[Trit::Zero, Trit::POne, Trit::NOne], [Trit::POne, Trit::NOne, Trit::Zero], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into();
        let tryte: Tryte = <Word as Into<[Tryte; 3]>>::into(word)[0];
        let mut cpu = Cpu::default();
        // MOVRI(R12_WORD, word)
        cpu.movri(R12_WORD, word);
        // MOVRR(R11_WORD, R12_WORD)
        cpu.movrr(R11_WORD, R12_WORD);
        // LHT(R11_WORD)
        cpu.lht(R11_WORD);
        assert_eq!(cpu.cpu_state_reg.get_interrupt_vector(), tryte);

        // let instr = INT([[Trit::Zero, Trit::POne, Trit::NOne]; 3].into());
        cpu.sti();
        assert_eq!(cpu.cpu_state_reg.get_interrupt_enable(), Trit::NOne);
        cpu.bti();
        assert_eq!(cpu.cpu_state_reg.get_interrupt_enable(), Trit::POne);
        // let instr = RTI;

        // STRI(R12_WORD, R11_WORD, [[Trit::Zero, Trit::POne, Trit::NOne]; 3].into());
        cpu.stri(R12_WORD, R11_WORD, tryte);
        // LDRI(R12_WORD, R11_WORD, [[Trit::Zero, Trit::POne, Trit::NOne]; 3].into());
        cpu.ldri(R10_WORD, R11_WORD, tryte);

        assert_eq!(cpu.register_file.get_word(R12_WORD), cpu.register_file.get_word(R10_WORD));

        // STRR(R12_WORD, R11_WORD, R11_WORD);
        cpu.strr(R12_WORD, R11_WORD, R11_WORD);
        // LDRR(R9_WORD, R11_WORD, R11_WORD);
        cpu.ldrr(R9_WORD, R11_WORD, R11_WORD);

        assert_eq!(cpu.register_file.get_word(R12_WORD), cpu.register_file.get_word(R9_WORD));

        // STRRI(R9_WORD, R11_WORD, R11_WORDm tryte)
        cpu.strri(R12_WORD, R11_WORD, R11_WORD, tryte);
        // LDRRI(R9_WORD, R11_WORD, R11_WORDm tryte)
        cpu.ldrri(R8_WORD, R11_WORD, R11_WORD, tryte);

        assert_eq!(cpu.register_file.get_word(R12_WORD), cpu.register_file.get_word(R8_WORD));

        // STRPCI(R12_WORD, word);
        cpu.strpci(R12_WORD, [[Trit::POne; 9].into(), Tryte::default(), Tryte::default()].into());
        // LDRPCI(R7_WORD, word);
        cpu.ldrpci(R7_WORD, [[Trit::POne; 9].into(), Tryte::default(), Tryte::default()].into());
        assert_eq!(cpu.register_file.get_word(R12_WORD), cpu.register_file.get_word(R7_WORD));

        cpu.movrr(R13_WORD, R12_WORD);
        cpu.owo(Tryte::default(), R6_WORD, R13_WORD);
        assert_eq!(cpu.register_file.get_word(R6_WORD), cpu.register_file.get_word(R13_WORD) & [O, W, O, W, O, W, O, W, O].into());

        cpu.movrr(R13_WORD, R12_WORD);
        cpu.uwu(Tryte::default(), R6_WORD, R13_WORD);
        assert_eq!(cpu.register_file.get_word(R6_WORD), cpu.register_file.get_word(R13_WORD) & [U, W, U, W, U, W, U, W, U].into());

        cpu.eqot(R5_WORD, tryte, R12_WORD, R11_WORD);
        assert_eq!(cpu.register_file.get_word(R5_WORD), (cpu.register_file.get_word(R12_WORD) / (<Tryte as Into<Word>>::into(tryte) + cpu.register_file.get_word(R11_WORD)).result).unwrap());

        cpu.erem(R5_WORD, tryte, R12_WORD, R11_WORD);
        assert_eq!(cpu.register_file.get_word(R5_WORD), (cpu.register_file.get_word(R12_WORD) % (<Tryte as Into<Word>>::into(tryte) + cpu.register_file.get_word(R11_WORD)).result).unwrap());

        // MOVRI(R12_WORD, word)
        cpu.movri(R12_WORD, word);
        // MOVRR(R11_WORD, R12_WORD)
        cpu.movrr(R11_WORD, R12_WORD);

        cpu.not(RN12_WORD, RN11_WORD);
        assert_eq!(cpu.register_file.get_word(RN12_WORD), !cpu.register_file.get_word(RN11_WORD));

        cpu.lsh(RN12_WORD, RN11_WORD, THREE_TRYTE);
        assert_eq!(cpu.register_file.get_word(RN12_WORD), cpu.register_file.get_word(RN11_WORD) << 3);

        cpu.rsh(RN12_WORD, RN11_WORD, THREE_TRYTE);
        assert_eq!(cpu.register_file.get_word(RN12_WORD), cpu.register_file.get_word(RN11_WORD) >> 3);

        cpu.movri(RN13_WORD, word);
        cpu.movri(RN12_WORD, word);
        cpu.movri(RN11_WORD, word);

        cpu.and_r(RN13_WORD, RN12_WORD, RN11_WORD);
        assert_eq!(cpu.register_file.get_word(RN13_WORD), cpu.register_file.get_word(RN11_WORD) & cpu.register_file.get_word(RN11_WORD));

        cpu.or_r(RN13_WORD, RN12_WORD, RN11_WORD);
        assert_eq!(cpu.register_file.get_word(RN13_WORD), cpu.register_file.get_word(RN11_WORD) | cpu.register_file.get_word(RN11_WORD));

        // TODO
        // ROTR(R12_WORD, RN11_WORD, RN11_WORD);
        // ROTI(R12_WORD, RN11_WORD, [[Trit::Zero, Trit::POne, Trit::NOne], [Trit::POne, Trit::NOne, Trit::Zero], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into());

        cpu.movri(SP_WORD, word);
        cpu.push_r3(RN12_WORD, RN11_WORD, RN13_WORD);
        // PUSHR3(R12_WORD, RN11_WORD, RN13_WORD);
        cpu.pop(RN1_WORD); 
        assert_eq!(cpu.register_file.get_word(RN1_WORD), (cpu.register_file.get_word(RN12_WORD) + cpu.register_file.get_word(RN11_WORD)).result * cpu.register_file.get_word(RN13_WORD));

        cpu.push_im_tryte((tryte + Trit::POne).result);
        cpu.pop(R1_TRYTE);
        assert_eq!(cpu.register_file.get_tryte(R1_TRYTE), (tryte + Trit::POne).result);

        // *((r0 + r1) * (r2 + imm))
        cpu.movri(R2_WORD, (word + Trit::POne).result);
        cpu.strr(R2_WORD, R2_WORD, R2_WORD);
        cpu.ldrr(R3_WORD, R2_WORD, R2_WORD);
        cpu.push_mem(R2_WORD, R2_WORD, R0_WORD, ONE_TRYTE);
        cpu.pop(R1_WORD);
        assert_eq!(cpu.register_file.get_word(R1_WORD), cpu.register_file.get_word(R3_WORD));

        cpu.add(RN11_WORD, ONE_TRYTE, R0_WORD, R0_WORD);
        cpu.br_r(R12_WORD, RN11_WORD, RN11_WORD);
        let sum = (cpu.register_file.get_word(R12_WORD) + cpu.register_file.get_word(RN11_WORD)).result * cpu.register_file.get_word(RN11_WORD);

        assert_eq!(cpu.program_counter, sum);

        cpu.add(RN11_WORD, ONE_TRYTE, R0_WORD, R0_WORD);
        let tr: Tryte = [[Trit::Zero, Trit::POne, Trit::NOne]; 3].into();
        let addr = (cpu.register_file.get_word(R12_WORD) + cpu.register_file.get_word(RN11_WORD)).result * (cpu.register_file.get_word(RN11_WORD) + <Tryte as Into<Word>>::into(tr)).result;
        cpu.movri(R5_WORD, addr);
        cpu.stri(RN13_WORD, R5_WORD, Tryte::default());

        cpu.br_m(R12_WORD, RN11_WORD, RN11_WORD, tr);

        assert_eq!(cpu.program_counter, cpu.register_file.get_word(RN13_WORD));

        // let instr = INR(R12_WORD, RN11_WORD);
        // let instr = OUTR(R12_WORD, RN11_WORD);
        // let instr = OUTI(R12_WORD, [[Trit::Zero, Trit::POne, Trit::NOne], [Trit::POne, Trit::NOne, Trit::Zero], ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO].into());
    }
}
