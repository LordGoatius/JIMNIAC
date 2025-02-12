use crate::{tryte::Tryte, word::Word};

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

