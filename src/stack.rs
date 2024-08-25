use crate::{errors::StackError, types::{Trit, Tryte}};

/// Represents the expression stack and the call stack of 
/// the Virtual Machine, and contains pointers to the top
#[derive(Debug, Clone, Copy)]
pub struct Stack<const S: usize> {
    callstack: [Tryte; S],
    exprstack: [Tryte; S],
    callstack_ptr: usize,
    exprstack_ptr: usize,
}

impl<const S: usize> Default for Stack<S> {
    fn default() -> Self {
        Self {
            callstack: [Tryte::default(); S],
            exprstack: [Tryte::default(); S],
            callstack_ptr: 0,
            exprstack_ptr: 0,
        }
    }
}

impl<const S: usize> Stack<S> {
    /// Constructs a stack program from a provided expression stack
    pub fn construct_from_exprstack(exprstack: [Tryte; S]) -> Self {
        Self {
            exprstack,
            ..Default::default()
        }
    }

    /// Pushes a tryte to the callstack if there is space, 
    /// returns Stack Overflow otherwise
    pub fn push_tryte_callstack(&mut self, tryte: Tryte) -> Result<(), StackError> {
        if self.callstack_ptr == (S - 1) { 
            Err(StackError::StackOverflow) 
        } else {
            self.callstack[self.callstack_ptr] = tryte;
            self.callstack_ptr += 1;
            Ok(())
        }
    }

    /// Pops the top tryte from the callstack, and
    /// returns a Stack Underflow otherwise
    pub fn pop_tryte_callstack(&mut self) -> Result<Tryte, StackError> {
        if self.callstack_ptr == 0 {
            Err(StackError::StackUnderflow)
        } else {
            self.callstack_ptr -= 1;
            let tryte = self.callstack[self.callstack_ptr];
            self.callstack[self.callstack_ptr] = Tryte::zero();
            return Ok(tryte)
        }
    }

    /// Pops an array of 3 trytes from the callstack
    pub fn pop_3_tryte_callstack(&mut self) -> Result<[Tryte; 3], StackError> {
        let tryte_0 = self.pop_tryte_callstack()?;
        let tryte_1 = self.pop_tryte_callstack()?;
        let tryte_2 = self.pop_tryte_callstack()?;

        Ok([tryte_0, tryte_1, tryte_2])
    }

    /// Pops an array of 9 trytes from the callstack
    pub fn pop_9_tryte_callstack(&mut self) -> Result<[Tryte; 9], StackError> {
        let tryte_0 = self.pop_tryte_callstack()?;
        let tryte_1 = self.pop_tryte_callstack()?;
        let tryte_2 = self.pop_tryte_callstack()?;
        let tryte_3 = self.pop_tryte_callstack()?;
        let tryte_4 = self.pop_tryte_callstack()?;
        let tryte_5 = self.pop_tryte_callstack()?;
        let tryte_6 = self.pop_tryte_callstack()?;
        let tryte_7 = self.pop_tryte_callstack()?;
        let tryte_8 = self.pop_tryte_callstack()?;

        Ok([tryte_0, tryte_1, tryte_2, 
            tryte_3, tryte_4, tryte_5, 
            tryte_6, tryte_7, tryte_8])
    }

    /// Pops an array of 3 trits from the callstack
    pub fn pop_3_3_trit_callstack(&mut self) -> Result<[[Trit; 3]; 3], StackError> {
        let zero = self.pop_tryte_callstack()?.value;
        let one  = self.pop_tryte_callstack()?.value;
        let two  = self.pop_tryte_callstack()?.value;

        Ok([zero, one, two])
    }

    /// Pops an array of 3 trits from the exprstack
    pub fn pop_3_3_trit_exprstack(&mut self) -> Result<[[Trit; 3]; 3], StackError> {
        let zero = self.pop_tryte_exprstack()?.value;
        let one  = self.pop_tryte_exprstack()?.value;
        let two  = self.pop_tryte_exprstack()?.value;

        Ok([zero, one, two])
    }

    /// Pushes a tryte to the expression stack if there is space, 
    /// returns Stack Overflow otherwise
    pub fn push_tryte_exprstack(&mut self, tryte: Tryte) -> Result<(), StackError> {
        if self.exprstack_ptr == (S - 1) { 
            Err(StackError::StackOverflow) 
        } else {
            self.exprstack[self.exprstack_ptr] = tryte;
            self.exprstack_ptr += 1;
            Ok(())
        }
    }

    /// Pops the top tryte from the callstack, and
    /// returns a Stack Underflow otherwise
    pub fn pop_tryte_exprstack(&mut self) -> Result<Tryte, StackError> {
        if self.exprstack_ptr == 0 {
            Err(StackError::StackUnderflow)
        } else {
            self.exprstack_ptr -= 1;
            let tryte = self.exprstack[self.exprstack_ptr];
            self.exprstack[self.exprstack_ptr] = Tryte::zero();
            return Ok(tryte)
        }
    }

    /// Pops an array of 3 trytes from the exprstack
    pub fn pop_3_tryte_exprstack(&mut self) -> Result<[Tryte; 3], StackError> {
        let tryte_0 = self.pop_tryte_exprstack()?;
        let tryte_1 = self.pop_tryte_exprstack()?;
        let tryte_2 = self.pop_tryte_exprstack()?;

        Ok([tryte_0, tryte_1, tryte_2])
    }

    /// Pops an array of 9 trytes from the exprstack
    pub fn pop_9_tryte_exprstack(&mut self) -> Result<[Tryte; 9], StackError> {
        let tryte_0 = self.pop_tryte_exprstack()?;
        let tryte_1 = self.pop_tryte_exprstack()?;
        let tryte_2 = self.pop_tryte_exprstack()?;
        let tryte_3 = self.pop_tryte_exprstack()?;
        let tryte_4 = self.pop_tryte_exprstack()?;
        let tryte_5 = self.pop_tryte_exprstack()?;
        let tryte_6 = self.pop_tryte_exprstack()?;
        let tryte_7 = self.pop_tryte_exprstack()?;
        let tryte_8 = self.pop_tryte_exprstack()?;

        Ok([tryte_0, tryte_1, tryte_2, 
            tryte_3, tryte_4, tryte_5, 
            tryte_6, tryte_7, tryte_8])
    }

    /// Sets callstack_ptr to correct location for program execution
    pub fn set_callstack_ptr(&mut self, callstack_ptr: usize) {
        self.callstack_ptr = callstack_ptr;
    }
}
