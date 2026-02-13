use std::hint::unreachable_unchecked;

use ternary::{prelude::{Tryte, Word}, trits::Trit};

use crate::{
    cpu::{CSR, JX_01},
    isa::{
        self,
        registers::{Register, RegisterSized},
        *,
    },
};

impl<'a> JX_01<'a> {
    pub fn run_program(&mut self) {
        // Initalize multi-threaded portions here
        // Ports
        // Status register
        // Option<Gpu>

        loop {
            // Put check for interrupt here
            use Instr::*;

            let instruction_ptr = self.status.ip;
            let instruction = *self.memory.get_physical_word(instruction_ptr);
            match isa::decode(instruction) {
                HALT => break,
                DTI => (),
                STI => (),
                WFI => (),
                RTI => (),
                LIT(reg) => (),
                INTERRUPT(int) => (),
                EGPU(reg) => (),
                LVB(reg, imm) => (),
                EGEL(reg) => (),
                PCSR => (),
                PPSR => (),
                PPTR => (),
                POCSR => (),
                POPSR => (),
                POPTR => (),
                LPT(reg) => (),
                INTM(imm) => (),
                INTE(imm) => (),
                INTS(imm) => (),
                IN(reg, ctrl, imm) => (),
                OUT(reg, ctrl, imm) => (),
                OPRR(ctrl, op, reg1, reg2, imm) => self.execute_rr_op(op, ctrl, reg1, reg2, imm),
                OPRI(ctrl, op, reg, imm) => self.execute_ri_op(op, ctrl, reg, imm),
                CALL(reg, ctrl, imm) => (),
                RET => (),
                ENTER => (),
                LEAVE => (),
                INVALID => (),
            }
        }
        println!("CPU Program Halted.")
    }

    /// Documentation for ALU
    ///                                   Load               Branch
    ///                                   [R] = *imm         [PC] = [R] + imm
    ///  ALU:-[C][I]-[R][imm @ 3-8]       [R] = *([R] + imm) [PC] = [R] +
    ///             \[R][R][imm @ 4-8]                              [R] * imm
    ///                                   Store              Cmp
    ///  Stack Ops:      ALU Ops          *imm = [R]         [R] ~ imm
    ///  [R] + imm       [R] = [R] op imm *([R] + imm) = [R] [R] ~ [R] + imm
    ///  [R] + [R] * imm [R] = [R] op imm
    fn execute_rr_op(&mut self, op: Op, ctrl: Control, reg1: Register, reg2: Register, imm: Word) {
        let (reg1_val, reg2_val) = match ctrl[1] {
            Trit::NOne => (self.registers.get_tryte(reg1).into(), self.registers.get_tryte(reg2).into()),
            Trit::POne => (self.registers.get_word(reg1), self.registers.get_word(reg2)),
            Trit::Zero => unsafe { unreachable_unchecked() },
        };

        let op = op_to_opt(op);

        // Values to override after execution
        let ip: Word;
        let mut sp: Word = self.status.sp;
        let mut bp: Word = self.status.bp;
        let mut csr: CSR = self.status.csr;

        match op {
            BPN => {
                if csr.get_parity() == Trit::NOne {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BPP => {
                if csr.get_parity() == Trit::POne {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BPZ => {
                if csr.get_parity() == Trit::Zero {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BGQ => {
                if csr.get_sign() == Trit::Zero || csr.get_sign() == Trit::POne {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BLQ => {
                if csr.get_sign() == Trit::Zero || csr.get_sign() == Trit::NOne {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BLT => {
                if csr.get_sign() == Trit::NOne {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BGT => {
                if csr.get_sign() == Trit::POne {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BNE => {
                if csr.get_sign() == Trit::NOne || csr.get_sign() == Trit::POne {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BEQ => {
                if csr.get_sign() == Trit::Zero {
                    ip = reg1_val + (reg2_val + imm);
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            CMP => {
                let res = reg1_val - reg2_val;
                // set sign, parity, and carry
                csr.set_sign(res.get_sign());
                csr.set_parity(res.get_parity());
                csr.set_carry(res.get_carry());

                ip = self.status.ip + (Word::PONE << 1);
            }
            STRE => {
                // *([R] + imm) = [R]
                let addr = reg1_val + imm;
                let mem_loc = self.memory.get_physical_word_mut(addr);
                // I might have messed things up but it's okay :)
                *mem_loc = reg2_val;
                ip = self.status.ip + (Word::PONE << 1);
            }
            LOAD => {
                // [R] = *([R] + imm)
                let addr = reg1_val + imm;
                let val = self.memory.get_physical_word(addr);
                self.registers.set_word(reg2, *val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            ADD => {
                //  ALU:-[C][I]-[R][imm @ 3-8]
                //             \[R][R][imm @ 4-8]
                //  ALU Ops         
                //  [R] = [R] op ([R] + imm)
                //  [R] = [R] op imm
                let val = reg1_val + (reg2_val + imm);
                self.registers.set_word(reg1, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            SUB => {
                let val = reg1_val - (reg2_val + imm);
                self.registers.set_word(reg1, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            MUL => {
                let val = reg1_val * (reg2_val + imm);
                self.registers.set_word(reg1, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            QOT => {
                // TODO: Handle Div by Zero Fault
                let val = reg1_val / (reg2_val + imm);
                self.registers.set_word(reg1, val.unwrap());
                ip = self.status.ip + (Word::PONE << 1);
            }
            REM => {
                // TODO: Handle Div by Zero Fault
                let val = reg1_val % (reg2_val + imm);
                self.registers.set_word(reg1, val.unwrap());
                ip = self.status.ip + (Word::PONE << 1);
            }
            AND => {
                let val = reg1_val & (reg2_val + imm);
                self.registers.set_word(reg1, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            OR => {
                let val = reg1_val | (reg2_val + imm);
                self.registers.set_word(reg1, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            SFT => {
                let shift: isize = (reg2_val + imm).into();
                let val = match shift.signum() {
                    -1 => reg1_val >> (shift.abs() as usize),
                    1 => reg1_val << (shift.abs() as usize),
                    _ => unsafe {
                        std::hint::unreachable_unchecked()
                    }
                };
                self.registers.set_word(reg1, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            NOT => {
                //  ALU:-[C][I]-[R][imm @ 3-8]
                //             \[R][R][imm @ 4-8]
                //  ALU Ops
                //  [R] = [R] op ([R] + imm)
                //  [R] = [R] op imm
                self.registers.set_word(reg1, -reg1_val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            ROT => {
                self.registers.set_word(reg1, reg1_val.rot(reg2_val.into()));
                ip = self.status.ip + (Word::PONE << 1);
            }
            //  Stack Ops:
            //  [R] + imm
            //  [R] + ([R] * imm)
            // ONLY SUPPORTING WORD SIZE VALUES RIGHT NOW
            PUSH => {
                // TODO: Memory write
                ip = self.status.ip + (Word::PONE << 1);
                sp = self.status.sp + (Word::PONE << 1);
            }
            POP => {
                // TODO: Memory write
                ip = self.status.ip + (Word::PONE << 1);
                sp = self.status.sp - (Word::PONE << 1);
            }
            // Setup the new stack frame
            CALL => {
                // TODO:
                // - Push next IP to stack
                // - Push BP to stack
                // - Move BP to after prev BP location
                // - Move SP to base pointer
                ip = self.status.ip + (Word::PONE << 1);
            }
            RET => {
                // TODO:
                // - Move SP to BP location
                // - Pop prev BP to BP
                // - Pop prev IP to SP
                ip = self.status.ip + (Word::PONE << 1);
            }
            _ => unsafe { unreachable_unchecked() },
        }
        self.status.ip = ip;
        self.status.sp = sp;
    }

    fn execute_ri_op(&mut self, op: Op, ctrl: Control, reg: Register, imm: Word) {
        let reg = RegisterSized(ctrl[1], reg.0);
    }

    pub fn import_memory(&mut self, trytes: &[Tryte]) {
        todo!()
    }
}
