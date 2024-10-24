use registers::RegisterFile;
use statusword::StatusWord;

use crate::{stack::Stack, tryte::Tryte};

pub mod errors;
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
    stack: Stack
}
