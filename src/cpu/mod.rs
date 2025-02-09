#![expect(unused)]
use errors::CpuError;
use jt1701isa::jt1701;
use registers::{Register, RegisterFile};
use statusword::StatusWord;

use crate::{
    stack::Stack,
    trits::Trit,
    tryte::Tryte,
    word::Word,
};

pub mod errors;
pub mod jt1701isa;
pub mod macros;
pub mod registers;
pub mod statusword;

#[derive(Default, Debug)]
pub struct Cpu {
    // Done
    register_file: RegisterFile,
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
        todo!()
    }
    /// Halt
    fn hlt(&mut self) {
        todo!()
    }
    /// Interrupt
    fn int(&mut self, interrupt: Tryte) {
        todo!()
    }
    /// No Op
    fn nop(&self) {
        todo!()
    }
    /// Wait For Interrupt
    fn wfi(&mut self) {
        todo!()
    }
    /// Stop Interrupts
    fn sti(&mut self) {
        todo!()
    }
    /// Begin Interrupts
    fn bti(&mut self) {
        todo!()
    }
    /// Returns from interrupt and restores status register
    fn rti(&mut self) {
        todo!()
    }

    //== Loading Register to Memory ==//
    /// dest = *(src + imm)
    fn ldri(&mut self, dest: Register, src: Register, imm: Tryte) {
        todo!()
    }
    /// dest = *(src0 + src1)
    fn ldrr(&mut self, dest: Register, src0: Register, src1: Register) {
        todo!()
    }
    /// dest = *(src0 + src1 + imm)
    fn ldrri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte) {
        todo!()
    }
    /// Word should have most sig. tryte be 0.
    /// dest = *(pc + imm)
    fn ldrpci(&mut self, dest: Register, imm: Word) {
        todo!()
    }

    //== Storing Register to Memory ==//
    /// *(src + imm) = dest
    fn stri(&mut self, dest: Register, src: Register, imm: Tryte) {
        todo!()
    }
    /// *(src0 + src1) = dest
    fn strr(&mut self, dest: Register, src0: Register, src1: Register) {
        todo!()
    }
    /// *(src0 + src1 + imm) = dest
    fn strri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte) {
        todo!()
    }
    /// Word should have most sig. tryte be 0.
    /// *(pc + imm) = dest
    fn strpci(&mut self, dest: Register, imm: Word) {
        todo!()
    }

    //== Moving ==//
    fn movrr(&mut self, dest: Register, src: Register) {
        todo!()
    }
    /// imm will always be 2 trytes long, or 18 trits.
    /// To move a larger size, the assembler will create shifts and adds in order to do so
    fn movri(&mut self, dest: Register, imm: Word) {
        todo!()
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
    fn add(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        todo!()
    }

    /// d = s0 * (s1 + imm)
    fn mul(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        todo!()
    }

    /// d = s0 - (s1 + imm)
    fn sub(&mut self, dest: Register, imm: Tryte, src0: Register, src1: Register) {
        todo!()
    }

    /// d = s0 / (s1 + imm)
    fn eqot(
        &mut self,
        dest: Register,
        imm: Tryte,
        src0: Register,
        src1: Register,
    ) -> Result<(), CpuError> {
        todo!()
    }

    /// d = s0 % (s1 + imm)
    fn erem(
        &mut self,
        dest: Register,
        imm: Tryte,
        src0: Register,
        src1: Register,
    ) -> Result<(), CpuError> {
        todo!()
    }

    //=== Trit ===//
    /// d = ~s
    fn not(&mut self, dest: Register, src: Register) {
        todo!()
    }

    // NOTE: Should left and right be different?
    fn lsh(&mut self, dest: Register, src: Register, count: Tryte) {
        todo!()
    }
    fn rsh(&mut self, dest: Register, src: Register, count: Tryte) {
        todo!()
    }

    // FIXME: Should these be separate?
    fn and_r(&mut self, dest: Register, src0: Register, src1: Register) {
        todo!()
    }
    fn and_i(&mut self, dest: Register, src0: Register, mask: Word) {
        todo!()
    }

    // FIXME: Should these be separate?
    fn or_r(&mut self, dest: Register, src0: Register, src1: Register) {
        todo!()
    }
    fn or_i(&mut self, dest: Register, src0: Register, mask: Word) {
        todo!()
    }

    // FIXME: Should these be separate?
    fn rot_r(&mut self, dest: Register, src0: Register, src1: Register) {
        todo!()
    }
    fn rot_i(&mut self, dest: Register, src0: Register, mask: Word) {
        todo!()
    }

    //== Stack ==//
    /// (r0 + r1) * r2
    fn push_r3(&mut self, r0: Register, r1: Register, r2: Register) {
        todo!()
    }
    fn push_im(&mut self, imm: Word) {
        todo!()
    }
    /// *((r0 + r1) * (r2 + imm))
    fn push_mem(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte) {
        todo!()
    }

    //== Branch ==//
    /// Compare 2 registers
    fn cmp(&mut self, r0: Register, r1: Register) {
        todo!()
    }
    /// Set Parity Trit
    fn spt(&mut self, r: Register) {
        todo!()
    }
    /// Set SignTrit
    fn sst(&mut self, r: Register) {
        todo!()
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
