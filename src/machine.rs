use std::usize;

use crate::{errors::StackError, stack::Stack, types::{Trit, Tryte}};

/// Struct representing a virtual machine that loads program 
/// into the callstack
pub struct Machine<const S: usize> {
    pub stack: Stack<S>,
    program_counter: usize,
}

/// Struct representing a virtual machine that loads program 
/// into immutable ROM
pub struct MachineROM<const S: usize> {
    stack: Stack<S>,
    program_counter: usize,
    rom: Vec<Tryte>,
}

/// Trait for 3-Trit sized Tryte based VM
trait VirtualMachine {
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
    fn pt  (&mut self) -> Result<(), StackError>;
    /// Push third
    fn pth (&mut self) -> Result<(), StackError>;
    /// Push triword
    fn pw  (&mut self) -> Result<(), StackError>;
    /// Push tryte command stack
    fn pct (&mut self) -> Result<(), StackError>;
    /// Push third command stack
    fn pcth(&mut self) -> Result<(), StackError>;
    /// Push triword command stack
    fn pcw (&mut self) -> Result<(), StackError>;
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

        Ok(())
    }

    /// Negation
    fn neg(&mut self) -> Result<(), StackError> {
        let value = self.stack.pop_tryte_exprstack()?;
        self.stack.push_tryte_exprstack(value.neg());
        Ok(())
    }

    /// Bitwise and
    fn and(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_tryte_exprstack()?;
        let r = self.stack.pop_tryte_exprstack()?;

        self.stack.push_tryte_exprstack(Tryte::and(l, r));
        Ok(())
    }
    /// Bitwise or 
    fn or (&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_tryte_exprstack()?;
        let r = self.stack.pop_tryte_exprstack()?;

        self.stack.push_tryte_exprstack(Tryte::or(l, r));
        Ok(())
    }
    /// Bitwise xor
    fn xor(&mut self) -> Result<(), StackError> {
        let l = self.stack.pop_tryte_exprstack()?;
        let r = self.stack.pop_tryte_exprstack()?;

        
        self.stack.push_tryte_exprstack(
            Tryte::and(Tryte::or(l, r), Tryte::neg(Tryte::and(l, r)))
            );
        Ok(())
    }

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
    fn pt  (&mut self) -> Result<(), StackError>;
    /// Push third
    fn pth (&mut self) -> Result<(), StackError>;
    /// Push triword
    fn pw  (&mut self) -> Result<(), StackError>;
    /// Push tryte command stack
    fn pct (&mut self) -> Result<(), StackError>;
    /// Push third command stack
    fn pcth(&mut self) -> Result<(), StackError>;
    /// Push triword command stack
    fn pcw (&mut self) -> Result<(), StackError>;
}
