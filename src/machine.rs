use std::{isize, usize};

use crate::{errors::StackError, stack::Stack, types::{Trit, Tryte}};

/// Struct representing a virtual machine that loads program 
/// into the callstack
#[derive(Debug, Default, Clone, Copy)]
pub struct Machine<const S: usize> {
    pub stack: Stack<S>,
    pub program_counter: usize,
    c: Trit,
}

/// Trait for 3-Trit sized Tryte based VM
pub trait VirtualMachine {
    /// Left shift 3 trytes
    fn lsh(&mut self) -> Result<(), StackError>;
    /// Right shift 3 trytes
    fn rsh(&mut self) -> Result<(), StackError>;
    /// Negation
    fn neg(&mut self) -> Result<(), StackError>;

    /// Bitwise and
    fn and(&mut self) -> Result<(), StackError>;
    /// Bitwise or 
    fn or (&mut self) -> Result<(), StackError>;
    /// Bitwise xor
    fn xor(&mut self) -> Result<(), StackError>;

    /// Arithmetic add tryte
    fn add (&mut self) -> Result<(), StackError>;
    /// Arithmetic sub tryte
    fn sub (&mut self) -> Result<(), StackError>;
    /// Arithmetic mul tryte
    fn mul (&mut self) -> Result<(), StackError>;
    /// Arithmetic add 3 trytes
    fn add3(&mut self) -> Result<(), StackError>;
    /// Arithmetic sub 3 trytes
    fn sub3(&mut self) -> Result<(), StackError>;
    /// Arithmetic mul 3 trytes
    fn mul3(&mut self) -> Result<(), StackError>;
    /// Arithmetic add 9 trytes
    fn add9(&mut self) -> Result<(), StackError>;
    /// Arithmetic sub 9 trytes
    fn sub9(&mut self) -> Result<(), StackError>;
    /// Arithmetic mul 9 trytes
    fn mul9(&mut self) -> Result<(), StackError>;

    /// Set C flag to -1, 0, or 1 based on the top 
    /// items in the expression stack
    fn cmp(&mut self) -> Result<(), StackError>;
    /// Unconditional jump
    fn br (&mut self) -> Result<(), StackError>;
    /// Jump C=-1 || 1
    fn bne(&mut self) -> Result<(), StackError>;
    /// Jump C=1
    fn bgt(&mut self) -> Result<(), StackError>;
    /// Jump C=-1
    fn blt(&mut self) -> Result<(), StackError>;
    /// Jump C=0
    fn beq(&mut self) -> Result<(), StackError>;

    /// Push tryte
    fn pt  (&mut self, tryte: Tryte) -> Result<(), StackError>;
    /// Push third
    fn pth (&mut self, third: [Tryte; 3]) -> Result<(), StackError>;
    /// Push triword
    fn pw  (&mut self, tword: [Tryte; 9]) -> Result<(), StackError>;
    /// Push tryte command stack
    fn pct (&mut self, tryte: Tryte) -> Result<(), StackError>;
    /// Push third command stack
    fn pcth(&mut self, third: [Tryte; 3]) -> Result<(), StackError>;
    /// Push triword command stack
    fn pcw (&mut self, tword: [Tryte; 9]) -> Result<(), StackError>;
}

impl<const S: usize> VirtualMachine for Machine<S> {
    /// Left shift 3 trytes
    fn rsh(&mut self) -> Result<(), StackError> {
        let [ternary_0, ternary_1, ternary_2]: [[Trit; 3]; 3] = self.stack.pop_3_3_trit_exprstack()?;

        let combined: [Trit; 9] = [
            ternary_0[0], ternary_0[1], ternary_0[2], 
            ternary_1[0], ternary_1[1], ternary_1[2], 
            ternary_2[0], ternary_2[1], ternary_2[2]];

        let shifted = [
            Trit::Zero , combined[0], combined[1], 
            combined[2], combined[3], combined[4], 
            combined[5], combined[6], combined[7]];

        self.stack.push_tryte_exprstack(Tryte::from_arr([shifted[6], shifted[7], shifted[8]])?)?;
        self.stack.push_tryte_exprstack(Tryte::from_arr([shifted[3], shifted[4], shifted[5]])?)?;
        self.stack.push_tryte_exprstack(Tryte::from_arr([shifted[0], shifted[1], shifted[2]])?)?;

        self.program_counter += 1;

        Ok(())
    }

    /// Right shift 3 trytes
    fn lsh(&mut self) -> Result<(), StackError> {
        let [ternary_0, ternary_1, ternary_2]: [[Trit; 3]; 3] = self.stack.pop_3_3_trit_exprstack()?;

        let combined: [Trit; 9] = [
            ternary_0[0], ternary_0[1], ternary_0[2], 
            ternary_1[0], ternary_1[1], ternary_1[2], 
            ternary_2[0], ternary_2[1], ternary_2[2]];

        let shifted = [
            combined[1], combined[2], combined[3], 
            combined[4], combined[5], combined[6], 
            combined[7], combined[8], Trit::Zero];

        self.stack.push_tryte_exprstack(Tryte::from_arr([shifted[6], shifted[7], shifted[8]])?)?;
        self.stack.push_tryte_exprstack(Tryte::from_arr([shifted[3], shifted[4], shifted[5]])?)?;
        self.stack.push_tryte_exprstack(Tryte::from_arr([shifted[0], shifted[1], shifted[2]])?)?;

        self.program_counter += 1;

        Ok(())
    }

    /// Negation
    fn neg(&mut self) -> Result<(), StackError> {
        let value = self.stack.pop_tryte_exprstack()?;
        self.stack.push_tryte_exprstack(value.neg())?;
        self.program_counter += 1;
        Ok(())
    }

    /// Bitwise and
    fn and(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_tryte_exprstack()?;
        let r = self.stack.pop_tryte_exprstack()?;

        self.stack.push_tryte_exprstack(Tryte::and(l, r))?;
        self.program_counter += 1;
        Ok(())
    }
    /// Bitwise or 
    fn or (&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_tryte_exprstack()?;
        let r = self.stack.pop_tryte_exprstack()?;

        self.stack.push_tryte_exprstack(Tryte::or(l, r))?;
        self.program_counter += 1;
        Ok(())
    }
    /// Bitwise xor
    fn xor(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_tryte_exprstack()?;
        let r = self.stack.pop_tryte_exprstack()?;

        
        self.stack.push_tryte_exprstack(
            Tryte::and(Tryte::or(l, r), Tryte::neg(Tryte::and(l, r)))
            )?;
        self.program_counter += 1;
        Ok(())
    }

    /// Arithmetic add tryte
    fn add (&mut self) -> Result<(), StackError> {
        let r0 = self.stack.pop_tryte_exprstack()?;
        let r1 = self.stack.pop_tryte_exprstack()?;

        let (_, res) = Tryte::add(r0, r1);
        self.stack.push_tryte_exprstack(res)?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic sub tryte
    fn sub (&mut self) -> Result<(), StackError> {
        let r0 = self.stack.pop_tryte_exprstack()?;
        let r1 = self.stack.pop_tryte_exprstack()?;

        let (_, res) = Tryte::add(r0, Tryte::neg(r1));
        self.stack.push_tryte_exprstack(res)?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic mul tryte
    fn mul (&mut self) -> Result<(), StackError> {
        let r0 = self.stack.pop_tryte_exprstack()?;
        let r1 = self.stack.pop_tryte_exprstack()?;

        let res = Tryte::mul(r0, r1);
        self.stack.push_tryte_exprstack(res)?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic add 3 trytes
    fn add3(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_3_tryte_exprstack()?;
        let r = self.stack.pop_3_tryte_exprstack()?;
        let (_, res) = Tryte::add_3(l, r);

        self.stack.push_tryte_exprstack(res[2])?;
        self.stack.push_tryte_exprstack(res[1])?;
        self.stack.push_tryte_exprstack(res[0])?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic sub 3 trytes
    fn sub3(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_3_tryte_exprstack()?;
        let r = self.stack.pop_3_tryte_exprstack()?.map(|elem| Tryte::neg(elem));
        let (_, res) = Tryte::add_3(l, r);

        self.stack.push_tryte_exprstack(res[2])?;
        self.stack.push_tryte_exprstack(res[1])?;
        self.stack.push_tryte_exprstack(res[0])?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic mul 3 trytes
    fn mul3(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_3_tryte_exprstack()?;
        let r = self.stack.pop_3_tryte_exprstack()?;
        let res = Tryte::mul_3(l, r);

        self.stack.push_tryte_exprstack(res[2])?;
        self.stack.push_tryte_exprstack(res[1])?;
        self.stack.push_tryte_exprstack(res[0])?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic add 9 trytes
    fn add9(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_9_tryte_exprstack()?;
        let r = self.stack.pop_9_tryte_exprstack()?;
        let (_, res) = Tryte::add_9(l, r);

        self.stack.push_tryte_exprstack(res[8])?;
        self.stack.push_tryte_exprstack(res[7])?;
        self.stack.push_tryte_exprstack(res[6])?;
        self.stack.push_tryte_exprstack(res[5])?;
        self.stack.push_tryte_exprstack(res[4])?;
        self.stack.push_tryte_exprstack(res[3])?;
        self.stack.push_tryte_exprstack(res[2])?;
        self.stack.push_tryte_exprstack(res[1])?;
        self.stack.push_tryte_exprstack(res[0])?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic sub 9 trytes
    fn sub9(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_9_tryte_exprstack()?;
        let r = self.stack.pop_9_tryte_exprstack()?.map(|elem| Tryte::neg(elem));
        let (_, res) = Tryte::add_9(l, r);

        self.stack.push_tryte_exprstack(res[8])?;
        self.stack.push_tryte_exprstack(res[7])?;
        self.stack.push_tryte_exprstack(res[6])?;
        self.stack.push_tryte_exprstack(res[5])?;
        self.stack.push_tryte_exprstack(res[4])?;
        self.stack.push_tryte_exprstack(res[3])?;
        self.stack.push_tryte_exprstack(res[2])?;
        self.stack.push_tryte_exprstack(res[1])?;
        self.stack.push_tryte_exprstack(res[0])?;
        self.program_counter += 1;
        Ok(())
    }
    /// Arithmetic mul 9 trytes
    fn mul9(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_9_tryte_exprstack()?;
        let r = self.stack.pop_9_tryte_exprstack()?;
        let res = Tryte::mul_9(l, r);

        self.stack.push_tryte_exprstack(res[8])?;
        self.stack.push_tryte_exprstack(res[7])?;
        self.stack.push_tryte_exprstack(res[6])?;
        self.stack.push_tryte_exprstack(res[5])?;
        self.stack.push_tryte_exprstack(res[4])?;
        self.stack.push_tryte_exprstack(res[3])?;
        self.stack.push_tryte_exprstack(res[2])?;
        self.stack.push_tryte_exprstack(res[1])?;
        self.stack.push_tryte_exprstack(res[0])?;
        self.program_counter += 1;
        Ok(())
    }

    /// Set C flag to -1, 0, or 1 based on the top 
    /// items in the expression stack
    fn cmp(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_tryte_exprstack()?;
        let r = self.stack.pop_tryte_exprstack()?;

        match l.cmp(&r) {
            std::cmp::Ordering::Greater => self.c = Trit::POne,
            std::cmp::Ordering::Equal   => self.c = Trit::Zero,
            std::cmp::Ordering::Less    => self.c = Trit::NOne,
        }

        self.stack.push_tryte_exprstack(r)?;
        self.stack.push_tryte_exprstack(l)?;

        self.program_counter += 1;
        Ok(())
    }
    /// Unconditional jump
    fn br (&mut self) -> Result<(), StackError> {
        let addr = self.stack.pop_9_tryte_callstack()?;

        // If addr is positive the addr_usize will not underflow
        let mut addr_usize = 0;

        for i in 0u32..27 {
            let mul = match addr[i as usize / 3][i as usize % 3] {
                Trit::POne =>  1,
                Trit::Zero =>  0,
                Trit::NOne => -1,
            };
            addr_usize += mul * isize::pow(3, i);
        }

        self.program_counter = addr_usize as usize;

        Ok(())
    }
    /// Jump C=-1 || 1
    fn bne(&mut self) -> Result<(), StackError> {
        let addr = self.stack.pop_9_tryte_callstack()?;

        match self.c {
            Trit::Zero => {
                self.program_counter += 1;
                return Ok(());
            },
            _ => (),
        }

        // If addr is positive the addr_usize will not underflow
        let mut addr_usize = 0;

        for i in 0u32..27 {
            let mul = match addr[i as usize / 3][i as usize % 3] {
                Trit::POne =>  1,
                Trit::Zero =>  0,
                Trit::NOne => -1,
            };
            addr_usize += mul * isize::pow(3, i);
        }

        self.program_counter = addr_usize as usize;

        Ok(())
    }
    /// Jump C=1
    fn bgt(&mut self) -> Result<(), StackError> {
        let addr = self.stack.pop_9_tryte_callstack()?;

        match self.c {
            Trit::Zero => {
                self.program_counter += 1;
                return Ok(());
            },
            Trit::NOne => {
                self.program_counter += 1;
                return Ok(());
            },
            _ => (),
        }

        // If addr is positive the addr_usize will not underflow
        let mut addr_usize = 0;

        for i in 0u32..27 {
            let mul = match addr[i as usize / 3][i as usize % 3] {
                Trit::POne =>  1,
                Trit::Zero =>  0,
                Trit::NOne => -1,
            };
            addr_usize += mul * isize::pow(3, i);
        }

        self.program_counter = addr_usize as usize;

        Ok(())
    }
    /// Jump C=-1
    fn blt(&mut self) -> Result<(), StackError> {
        let addr = self.stack.pop_9_tryte_callstack()?;

        match self.c {
            Trit::Zero => {
                self.program_counter += 1;
                return Ok(());
            },
            Trit::POne => {
                self.program_counter += 1;
                return Ok(());
            },
            _ => (),
        }

        // If addr is positive the addr_usize will not underflow
        let mut addr_usize = 0;

        for i in 0u32..27 {
            let mul = match addr[i as usize / 3][i as usize % 3] {
                Trit::POne =>  1,
                Trit::Zero =>  0,
                Trit::NOne => -1,
            };
            addr_usize += mul * isize::pow(3, i);
        }

        self.program_counter = addr_usize as usize;

        Ok(())
    }

    /// Jump C=0
    fn beq(&mut self) -> Result<(), StackError> {
        let addr = self.stack.pop_9_tryte_callstack()?;

        match self.c {
            Trit::NOne => {
                self.program_counter += 1;
                return Ok(());
            },
            Trit::POne => {
                self.program_counter += 1;
                return Ok(());
            },
            _ => (),
        }

        // If addr is positive the addr_usize will not underflow
        let mut addr_usize = 0;

        for i in 0u32..27 {
            let mul = match addr[i as usize / 3][i as usize % 3] {
                Trit::POne =>  1,
                Trit::Zero =>  0,
                Trit::NOne => -1,
            };
            addr_usize += mul * isize::pow(3, i);
        }

        self.program_counter = addr_usize as usize;

        Ok(())
    }

    /// Push tryte
    fn pt  (&mut self, tryte: Tryte) -> Result<(), StackError> {
        self.stack.push_tryte_exprstack(tryte)?;
        self.program_counter += 1 + 1;
        Ok(())
    }
    /// Push third
    fn pth (&mut self, third: [Tryte; 3]) -> Result<(), StackError> {
        self.stack.push_tryte_exprstack(third[2])?;
        self.stack.push_tryte_exprstack(third[1])?;
        self.stack.push_tryte_exprstack(third[0])?;
        self.program_counter += 1 + 3;
        Ok(())
    }
    /// Push triword
    fn pw  (&mut self, tword: [Tryte; 9]) -> Result<(), StackError> {
        self.stack.push_tryte_exprstack(tword[8])?;
        self.stack.push_tryte_exprstack(tword[7])?;
        self.stack.push_tryte_exprstack(tword[6])?;
        self.stack.push_tryte_exprstack(tword[5])?;
        self.stack.push_tryte_exprstack(tword[4])?;
        self.stack.push_tryte_exprstack(tword[3])?;
        self.stack.push_tryte_exprstack(tword[2])?;
        self.stack.push_tryte_exprstack(tword[1])?;
        self.stack.push_tryte_exprstack(tword[0])?;
        self.program_counter += 1 + 9;
        Ok(())
    }
    /// Push tryte command stack
    fn pct (&mut self, tryte: Tryte) -> Result<(), StackError> {
        self.stack.push_tryte_callstack(tryte)?;
        self.program_counter += 1 + 1;
        Ok(())
    }
    /// Push third command stack
    fn pcth(&mut self, third: [Tryte; 3]) -> Result<(), StackError> {
        self.stack.push_tryte_callstack(third[2])?;
        self.stack.push_tryte_callstack(third[1])?;
        self.stack.push_tryte_callstack(third[0])?;
        self.program_counter += 1 + 3;
        Ok(())
    }
    /// Push triword command stack
    fn pcw (&mut self, tword: [Tryte; 9]) -> Result<(), StackError> {
        self.stack.push_tryte_callstack(tword[8])?;
        self.stack.push_tryte_callstack(tword[7])?;
        self.stack.push_tryte_callstack(tword[6])?;
        self.stack.push_tryte_callstack(tword[5])?;
        self.stack.push_tryte_callstack(tword[4])?;
        self.stack.push_tryte_callstack(tword[3])?;
        self.stack.push_tryte_callstack(tword[2])?;
        self.stack.push_tryte_callstack(tword[1])?;
        self.stack.push_tryte_callstack(tword[0])?;
        self.program_counter += 1 + 9;
        Ok(())
    }
}
