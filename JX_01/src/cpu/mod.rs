pub mod event_loop;

use septivigntimal::{to_num, ZERO};

use ternary::{trits::Trit, tryte::Tryte, word::Word};

use crate::{gpu::Gpu, isa::registers::Register, memory::Memory, ports::Ports};

#[allow(non_camel_case_types)]
pub struct JX_01<'a> {
    memory: Memory<'a>,
    ports: Ports,
    gpu: Option<Gpu>,
    interrupt: bool,
    interrupt_num: Tryte,
    // Change to allow zero register to be zero
    registers: Registers,
    status: Status,
}

#[derive(Default)]
pub struct Registers([Word; 27]);

impl Registers {
    fn get_word(&self, index: Register) -> Word {
        let ind = to_num(index.0) + 13;

        if ind == to_num(ZERO) + 13 {
            return Word::ZERO;
        }

        self.0[ind as usize]
    }

    fn set_word(&mut self, index: Register, val: Word) {
        let ind = to_num(index.0) + 13;

        if ind == to_num(ZERO) + 13 {
            return;
        }

        self.0[ind as usize] = val;
    }

    fn get_tryte(&self, index: Register) -> Tryte {
        let ind = to_num(index.0) + 13;

        if ind == to_num(ZERO) + 13 {
            return Tryte::ZERO;
        }

        self.0[ind as usize].into()
    }

    fn set_tryte(&mut self, index: Register, val: Tryte) {
        let ind = to_num(index.0) + 13;

        if ind == to_num(ZERO) + 13 {
            return;
        }

        self.0[ind as usize] = val.into();
    }
}
pub struct Status {
    // CSR
    csr: CSR,
    // Program Table Register
    ptr: Word,
    sp: Word,
    bp: Word,
    ip: Word,
}

// The CPU status register holds informations such as:
// Status Word:
/// [C, S, P, I, R, G, T, _, _,
///  interrupt_vector,
///  interrupt_number,
/// ]
/// C: Carry Flag
/// S: Sign Flag
/// P: Parity Flag
/// I: Interrupt Enable
/// R: Privledge Level
/// G: GPU Enable
/// T: Page Enable
#[derive(Clone, Copy)]
pub struct CSR(Word);

impl CSR {
    pub fn get_carry(&self) -> Trit {
        todo!()
    }
    pub fn get_sign(&self) -> Trit {
        todo!()
    }
    pub fn get_parity(&self) -> Trit {
        todo!()
    }
    pub fn get_interrupt(&self) -> Trit {
        todo!()
    }
    pub fn get_privilege(&self) -> Trit {
        todo!()
    }
    pub fn get_gpu(&self) -> Trit {
        todo!()
    }
    pub fn get_paging(&self) -> Trit {
        todo!()
    }
    pub fn get_interrupt_vector(&self) -> Tryte {
        todo!()
    }

    pub fn set_carry(&mut self, val: Trit) {
        todo!()
    }
    pub fn set_sign(&mut self, val: Trit) {
        todo!()
    }
    pub fn set_parity(&mut self, val: Trit) {
        todo!()
    }
    pub fn set_interrupt(&mut self, val: Trit) {
        todo!()
    }
    pub fn set_privilege(&mut self, val: Trit) {
        todo!()
    }
    pub fn set_gpu(&mut self, val: Trit) {
        todo!()
    }
    pub fn set_paging(&mut self, val: Trit) {
        todo!()
    }
    pub fn set_interrupt_vector(&mut self, val: Tryte) {
        todo!()
    }
}
