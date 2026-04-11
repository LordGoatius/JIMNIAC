use std::{hint::unreachable_unchecked, sync::atomic::Ordering};

use ternary::{prelude::Word, trits::Trit};

use crate::{
    cpu::{CSR, JX_01, Status}, gpu::Gpu, isa::{
        self,
        registers::{N11, N12, N13, NN11, NN12, NN13, RN11, Register},
        *,
    }, ports::Ports
};

impl JX_01 {
    pub fn run_program(&mut self) {
        // Initalize multi-threaded portions here
        // Ports
        self.ports = Some(Ports::init(self.interrupt.clone(), self.interrupt_num.clone()));
        // A program must setup the stack and base pointer
        self.status = Status {
            csr: CSR(Word::ZERO),
            ptr: Word::ZERO,
            psr: Word::ZERO,
            sp: Word::ZERO,
            bp: Word::ZERO,
            ip: Word::ZERO,
        };
        // Option<Gpu>
        self.gpu = None;
        struct IntStatus {
            enabled: bool,
            waiting: bool,
        }

        let mut int_status = IntStatus {
            enabled: true,
            waiting: false,
        };

        loop {
            use Instr::*;

            let instruction_ptr = self.status.ip;
            let _instr_ptr: isize = dbg!(instruction_ptr.into());

            let instruction = *self.memory.get_physical_word(instruction_ptr);
            // Check for status here: Interrupts
            if let Some(int) = self.interrupt() && int_status.enabled {
                // interrupt logic
            } else if int_status.waiting {
                continue;
            }

            match dbg!(isa::decode(instruction)) {
                HALT => break,
                DTI => int_status.enabled = false,
                STI => int_status.enabled = true,
                WFI => int_status.waiting = true,
                RTI => {
                    // store return address when exceptions occur and return.
                    // Don't manage the stack or registers.
                    todo!()
                },
                LIT(reg) => match self.idt_loc.as_mut() {
                    Some(loc) => {
                        *loc = self.registers.get_word(reg);
                        println!("Idt loaded at {loc}")
                    },
                    None => {
                        println!("Interrupt without IDT: Unrecoverable Fault");
                        break;
                    },
                },
                INTERRUPT(int) => {
                    // TODO: Valid user int checks.
                    self.interrupt.store(true, Ordering::Release);
                    self.interrupt_num.store(int.num(), Ordering::Release);
                },
                EGPU(reg) => {
                    let addr = self.registers.get_word(reg);
                    self.gpu = Some(Gpu::from_addr(addr, &mut self.memory));
                },
                LVB(reg, imm) => {
                    self.gpu.iter_mut().for_each(|gpu| {
                        gpu.vector_buffer = self.registers.get_word(reg);
                        gpu.vector_buffer_size = imm;
                    });
                },
                EGEL(reg) => {
                    self.gpu.iter_mut().for_each(|gpu| {
                        gpu.event_loop_callback = Some(self.registers.get_word(reg))
                    });
                },
                PCSR => {
                    *self.memory.get_physical_word_mut(self.status.sp) = self.status.csr.0;
                    self.status.ip = self.status.ip + (Word::PONE << 1);
                    self.status.sp = self.status.sp + (Word::PONE << 1);
                },
                PPSR => {
                    *self.memory.get_physical_word_mut(self.status.sp) = self.status.psr;
                    self.status.ip = self.status.ip + (Word::PONE << 1);
                    self.status.sp = self.status.sp + (Word::PONE << 1);
                },
                PPTR => {
                    *self.memory.get_physical_word_mut(self.status.sp) = self.status.ptr;
                    self.status.ip = self.status.ip + (Word::PONE << 1);
                    self.status.sp = self.status.sp + (Word::PONE << 1);
                },
                POCSR => {
                    self.registers.set_word(N13, *self.memory.get_physical_word(self.status.sp));
                    self.status.ip = self.status.ip + (Word::PONE << 1);
                    self.status.sp = self.status.sp - (Word::PONE << 1);
                    
                },
                POPSR => {
                    self.registers.set_word(N12, *self.memory.get_physical_word(self.status.sp));
                    self.status.ip = self.status.ip + (Word::PONE << 1);
                    self.status.sp = self.status.sp - (Word::PONE << 1);
                    
                },
                POPTR => {
                    self.registers.set_word(N11, *self.memory.get_physical_word(self.status.sp));
                    self.status.ip = self.status.ip + (Word::PONE << 1);
                    self.status.sp = self.status.sp - (Word::PONE << 1);
                    
                },
                LPT(reg) => {
                    self.page_table = Some(self.registers.get_word(reg));
                },
                INTM(imm) => (),
                INTE(imm) => (),
                INTS(imm) => (),
                IN(reg, ctrl, imm) => (),
                OUT(reg, ctrl, imm) => (),
                // Unused: Don't use OP CALL/RET
                // For full compliance/optimization, make them identical with different meanings
                OPRR(ctrl, op, reg1, reg2, imm) => self.execute_rr_op(op, ctrl, reg1, reg2, imm),
                OPRI(ctrl, op, reg, imm) => self.execute_ri_op(op, ctrl, reg, imm),
                // Call calls (jumps to addr in rn13)
                CALL(reg, ctrl, imm) => (),
                // Retun jumps to value in rn13
                RET => (),
                // Enter:
                // - Push next IP to stack
                // - Push BP to stack
                // - Move BP to after prev BP location
                // - Move SP to base pointer
                ENTER => (),
                // Leave:
                // - Move SP to BP location
                // - Pop prev BP to BP
                // - Pop prev IP to SP
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
    fn execute_rr_op(&mut self, op: Op, _ctrl: Control, reg1: Register, reg2: Register, imm: Word) {
        let (reg1_val, reg2_val) =  (self.registers.get_word(reg1), self.registers.get_word(reg2));

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
                let res: Word = reg1_val - reg2_val;
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
                *self.memory.get_physical_word_mut(sp) = reg1_val + (reg2_val * imm);
                ip = self.status.ip + (Word::PONE << 1);
                sp = self.status.sp + (Word::PONE << 1);
            }
            POP => {
                // TODO: Memory write
                self.registers.set_word(reg1, *self.memory.get_physical_word(sp));
                ip = self.status.ip + (Word::PONE << 1);
                sp = self.status.sp - (Word::PONE << 1);
            }
            CALL => {
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
        self.status.csr = csr;
    }

    fn execute_ri_op(&mut self, op: Op, _ctrl: Control, reg: Register, imm: Word) {
        let reg_val = self.registers.get_word(reg);

        let op = op_to_opt(op);

        // Values to override after execution
        let ip: Word;
        let mut sp: Word = self.status.sp;
        let mut bp: Word = self.status.bp;
        let mut csr: CSR = self.status.csr;

        match op {
            BPN => {
                if csr.get_parity() == Trit::NOne {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BPP => {
                if csr.get_parity() == Trit::POne {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BPZ => {
                if csr.get_parity() == Trit::Zero {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BGQ => {
                if csr.get_sign() == Trit::Zero || csr.get_sign() == Trit::POne {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BLQ => {
                if csr.get_sign() == Trit::Zero || csr.get_sign() == Trit::NOne {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BLT => {
                if csr.get_sign() == Trit::NOne {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BGT => {
                if csr.get_sign() == Trit::POne {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BNE => {
                if csr.get_sign() == Trit::NOne || csr.get_sign() == Trit::POne {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            BEQ => {
                if csr.get_sign() == Trit::Zero {
                    ip = reg_val + imm;
                } else {
                    ip = self.status.ip + (Word::PONE << 1);
                }
            }
            CMP => {
                let res = reg_val - imm;
                // set sign, parity, and carry
                csr.set_sign(res.get_sign());
                csr.set_parity(res.get_parity());
                csr.set_carry(res.get_carry());

                ip = self.status.ip + (Word::PONE << 1);
            }
            // Documentation for ALU
            //                                   Load               Branch
            //                                   [R] = *imm         [PC] = [R] + imm
            //  ALU:-[C][I]-[R][imm @ 3-8]       [R] = *([R] + imm) [PC] = [R] +
            //             \[R][R][imm @ 4-8]                              [R] * imm
            //                                   Store              Cmp
            //  Stack Ops:      ALU Ops          *imm = [R]         [R] ~ imm
            //  [R] + imm       [R] = [R] op imm *([R] + imm) = [R] [R] ~ [R] + imm
            //  [R] + [R] * imm [R] = [R] op imm
            STRE => {
                let addr = imm;
                let mem_loc = self.memory.get_physical_word_mut(addr);
                // I might have messed things up but it's okay :)
                *mem_loc = reg_val;
                ip = self.status.ip + (Word::PONE << 1);
            }
            LOAD => {
                let addr = imm;
                let val = self.memory.get_physical_word(addr);
                self.registers.set_word(reg, *val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            ADD => {
                //  ALU:-[C][I]-[R][imm @ 3-8]
                //             \[R][R][imm @ 4-8]
                //  ALU Ops         
                //  [R] = [R] op ([R] + imm)
                //  [R] = [R] op imm
                let val = reg_val + imm;
                self.registers.set_word(reg, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            SUB => {
                let val = reg_val - imm;
                self.registers.set_word(reg, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            MUL => {
                let val = reg_val * imm;
                self.registers.set_word(reg, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            QOT => {
                // TODO: Handle Div by Zero Fault
                let val = reg_val / imm;
                self.registers.set_word(reg, val.unwrap());
                ip = self.status.ip + (Word::PONE << 1);
            }
            REM => {
                // TODO: Handle Div by Zero Fault
                let val = reg_val % imm;
                self.registers.set_word(reg, val.unwrap());
                ip = self.status.ip + (Word::PONE << 1);
            }
            AND => {
                let val = reg_val & imm;
                self.registers.set_word(reg, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            OR => {
                let val = reg_val | imm;
                self.registers.set_word(reg, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            SFT => {
                let shift: isize = imm.into();
                let val = match shift.signum() {
                    -1 => reg_val >> (shift.abs() as usize),
                    1 => reg_val << (shift.abs() as usize),
                    _ => unsafe {
                        std::hint::unreachable_unchecked()
                    }
                };
                self.registers.set_word(reg, val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            NOT => {
                //  ALU:-[C][I]-[R][imm @ 3-8]
                //             \[R][R][imm @ 4-8]
                //  ALU Ops
                //  [R] = [R] op ([R] + imm)
                //  [R] = [R] op imm
                self.registers.set_word(reg, -reg_val);
                ip = self.status.ip + (Word::PONE << 1);
            }
            ROT => {
                self.registers.set_word(reg, reg_val.rot(imm.into()));
                ip = self.status.ip + (Word::PONE << 1);
            }
            //  Stack Ops:
            //  [R] + imm
            //  [R] + ([R] * imm)
            // ONLY SUPPORTING WORD SIZE VALUES RIGHT NOW
            PUSH => {
                // TODO: Memory write
                *self.memory.get_physical_word_mut(sp) = reg_val + imm;
                ip = self.status.ip + (Word::PONE << 1);
                sp = self.status.sp + (Word::PONE << 1);
            }
            POP => {
                // TODO: Memory write
                self.registers.set_word(reg, *self.memory.get_physical_word(sp));
                ip = self.status.ip + (Word::PONE << 1);
                sp = self.status.sp - (Word::PONE << 1);
            }
            // Setup the new stack frame
            CALL => {
                ip = self.status.ip + (Word::PONE << 1);
            }
            RET => {
                ip = self.status.ip + (Word::PONE << 1);
            }
            _ => unsafe { unreachable_unchecked() },
        }
        self.status.ip = ip;
        self.status.sp = sp;
        self.status.csr = csr;
    }

    pub fn import_memory(&mut self, memory: &[Word]) {
        let mut index = Word::ZERO;
        let add = Word::PONE << 1;

        for &word in memory {
            *self.memory.get_physical_word_mut(index) = word;
            index = index + add;
        }
    }

    pub fn import_instrs(&mut self, instrs: &[Instr]) {
        let mut index = Word::ZERO;
        let add = Word::PONE << 1;

        for &word in instrs {
            *self.memory.get_physical_word_mut(index) = encode(word);
            index = index + add;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use ternary::word::Word;
    use crate::{cpu::JX_01, isa::{ADD_T, ALU_CTRL_R_RI, ALU_CTRL_R_RR, BEQ_T, BGT_T, BLQ_T, BLT_T, CMP_T, Instr, MUL_T, SUB_T, code::DecEncExt, decode, encode, registers::*}};

    #[test]
    fn test_exec() {
        let n: Word = 6.into();
        // NOTE: Mov is not a thing in my ISA, since it is risc. Uhh work tbd because this
        // current ISA is a hot mess. I'll redo it for real later.
        // Anyways, mov is really [add %R, imm], or some equivalent set of instructions for larger immediates.

        // 0  mov %RN11, 1
        // 3  mov %RN13, n
        // 6  mov %RN12, 1
        // 9  cmp %RN13, %RN11
        // 12 bleqi 24
        // 15 mul %RN12, %RN12, %RN13
        // 18 add %RN13, %RN13, %R0, -1
        // 21 bri 9
        // 27 hlt
        use Instr::*;
        let instrs = [
            OPRI(ALU_CTRL_R_RI, ADD_T, NN11, Word::PONE),       // 0 
            OPRI(ALU_CTRL_R_RI, ADD_T, NN13, n),                // 3 
            OPRI(ALU_CTRL_R_RI, ADD_T, NN12, Word::PONE),       // 6 
            OPRR(ALU_CTRL_R_RR, CMP_T, NN13, NN11, Word::ZERO), // 9 
            OPRI(ALU_CTRL_R_RI, BLQ_T, N0, 27.into()),          // 12
            OPRR(ALU_CTRL_R_RR, MUL_T, NN12, NN13, Word::ZERO), // 15
            OPRR(ALU_CTRL_R_RR, ADD_T, NN13, N0, Word::NONE),   // 18
            OPRR(ALU_CTRL_R_RR, CMP_T, N0, N0, Word::ZERO),     // 21
            OPRI(ALU_CTRL_R_RI, BEQ_T, N0, 9.into()),           // 24
            HALT                                                // 27
        ];

        // NOTES:
        // For next time
        // - Make branch and OP different
        // - Follow RISCV. Just do what they do. they figured something good out.
        // - If you really want you can do variable length instruction encoding ig
        // - The ctrl tribble is stupid. Easy to write "assembly" errors.
        //   - Especially with load-store.
        instrs.check();

        let mut cpu = JX_01::new();
        cpu.import_instrs(&instrs);
        cpu.run_program();

        let fact = |mut n| {
            let none = Word::NONE;
            let mut prod = Word::PONE;
            while n > Word::ZERO {
                prod = prod * n;
                n = n + none;
            }
            prod
        };
        assert_eq!(cpu.registers.get_word(NN12), fact(n));
    }
}
