use errors::CpuError;
use jt1701isa::jt1701;
use registers::{Register, RegisterFile};
use statusword::StatusWord;

use crate::{stack::Stack, trits::Trit, tryte::Tryte, word::Word};

pub mod errors;
pub mod registers;
pub mod statusword;
pub mod jt1701isa;

#[derive(Default, Debug)]
pub struct Cpu {
    // Done
    register_file: RegisterFile,
    // Partial, can update as needed
    cpu_state_reg: StatusWord,
    saved_prog_status_reg: StatusWord,
    interrupt_vector: Option<Tryte>,
    // Done
    stack: Stack
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
    fn lht(&mut self, register: Register) {
        self.interrupt_vector = Some(self.register_file.get_value(register).lowest_tryte())
    }

    fn hlt(&mut self) {
        unreachable!()
    }
    fn int(&mut self, interrupt: Tryte);
    fn nop(&self) {
        return;
    }
    /// Wait For Interrupt
    fn wfi(&mut self);
    /// Stop Interrupts
    fn sti(&mut self);
    /// Begin Interrupts
    fn bti(&mut self);

    //== Loading ==//
    fn ldri(&mut self, dest: Register, src: Register, imm: Tryte);
    fn ldrr(&mut self, dest: Register, src0: Register, src1: Register);
    fn ldrri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte);
    // Word should have most sig. tryte be 0.
    fn ldrpci(&mut self, dest: Register, imm: Word);

    //== Storing ==//
    fn stri(&mut self, dest: Register, src: Register, imm: Tryte);
    fn strr(&mut self, dest: Register, src0: Register, src1: Register);
    fn strri(&mut self, dest: Register, src0: Register, src1: Register, imm: Tryte);
    // Word should have most sig. tryte be 0.
    fn strpci(&mut self, dest: Register, imm: Word);

    //== Moving ==//
    fn movrr(&mut self, dest: Register, src: Register);
    fn movri(&mut self, dest: Register, imm: Word);

    //==== ALU ====//
    fn owo_r(&mut self, dest: Register, src: Register);
    fn owo_i(&mut self, dest: Register, imm: Tryte);

    fn uwu_r(&mut self, dest: Register, src: Register);
    fn uwu_i(&mut self, dest: Register, imm: Tryte);

    fn add_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn add_i(&mut self, dest: Register, src: Register, imm: Tryte);

    fn mul_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn mul_i(&mut self, dest: Register, src: Register, imm: Tryte);

    fn sub_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn sub_i(&mut self, dest: Register, src: Register, imm: Tryte);

    fn eqot_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn eqot_i(&mut self, dest: Register, src: Register, imm: Tryte);

    fn erem_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn erem_i(&mut self, dest: Register, src: Register, imm: Tryte);

    //=== Trit ===//
    fn not(&mut self, dest: Register, src: Register);

    fn lsh(&mut self, dest: Register, src: Register, count: Tryte);
    fn rsh(&mut self, dest: Register, src: Register, count: Tryte);

    fn and_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn and_i(&mut self, dest: Register, src0: Register, mask: Word);

    fn or_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn or_i(&mut self, dest: Register, src0: Register, mask: Word);

    fn rot_r(&mut self, dest: Register, src0: Register, src1: Register);
    fn rot_i(&mut self, dest: Register, src0: Register, mask: Word);

    //== Stack ==//
    fn push_r3(&mut self, r0: Register, r1: Register, r2: Register);
    fn push_im(&mut self, imm: Word);
    fn push_mem(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    //== Branch ==//
    /// Compare 2 registers
    fn cmp(&mut self, r0: Register, r1: Register);
    /// Set Parity Trit
    fn spt(&mut self, r: Register);
    /// Set SignTrit
    fn sst(&mut self, r: Register);

    fn br_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn br_i(&mut self, imm: Word);
    fn br_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bne_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bne_i(&mut self, imm: Word);
    fn bne_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bgt_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bgt_i(&mut self, imm: Word);
    fn bgt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn blt_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn blt_i(&mut self, imm: Word);
    fn blt_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn beq_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn beq_i(&mut self, imm: Word);
    fn beq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bgeq_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bgeq_i(&mut self, imm: Word);
    fn bgeq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bleq_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bleq_i(&mut self, imm: Word);
    fn bleq_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bofn_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bofn_i(&mut self, imm: Word);
    fn bofn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bofz_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bofz_i(&mut self, imm: Word);
    fn bofz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bofp_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bofp_i(&mut self, imm: Word);
    fn bofp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bpn_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bpn_i(&mut self, imm: Word);
    fn bpn_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bpz_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bpz_i(&mut self, imm: Word);
    fn bpz_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    fn bpp_r(&mut self, r0: Register, r1: Register, r2: Register);
    fn bpp_i(&mut self, imm: Word);
    fn bpp_m(&mut self, r0: Register, r1: Register, r2: Register, imm: Tryte);

    //== Ports ==//
    fn in_r(&mut self, dest: Register, loc: Register);
    fn in_i(&mut self, dest: Register, loc: Word);

    fn out_r(&mut self, dest: Register, loc: Register);
    fn out_i(&mut self, dest: Register, loc: Word);
}
