use ternary::{tryte::Tryte, word::Word};

use crate::{gpu::Gpu, memory::Memory, ports::Ports};

#[allow(non_camel_case_types)]
pub struct JX_01<'a> {
    memory: Memory<'a>,
    ports: Ports,
    gpu: Option<Gpu>,
    interrupt: bool,
    interrupt_num: Tryte,
    registers: [Word; 27],
}
