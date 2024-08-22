use crate::{errors::StackError, types::Tryte};

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
}
