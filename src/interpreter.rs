use std::error::Error;
use std::fs;
use std::io::Read;

use itertools::Itertools;

use crate::errors::StackError;
use crate::machine::{Machine, VirtualMachine};
use crate::types::{Trit, Tryte};

/// Interpreter struct that uses (3^9 - 1) / 2 address space
struct Interpreter {
    pub machine: Machine<9841>,
}

impl Interpreter {
    /// Reads a program from an "executable file" (ascii 0, 1, or 2 only) into the callstack;
    /// Sets the program counter as well.
    pub fn read_program_into_memory(&mut self, mut file: fs::File) -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();
        let _  = file.read_to_end(&mut buffer)?;

        let buf_len = buffer.len();

        if buffer.len() % 3 != 0 {
            panic!("Should be in Trytecode");
        }

        let buffer = buffer
            .iter()
            .map(|byte| Trit::from_num(*byte - 48).unwrap())
            .collect_vec();

        for mut chunk in &buffer.iter().chunks(3) {
            self.machine.stack
                .push_tryte_callstack(
                    Tryte::from_arr(
                        [*chunk.next().unwrap(), 
                         *chunk.next().unwrap(), 
                         *chunk.next().unwrap()])
                    .unwrap())
                .unwrap();
        }

        self.machine.stack.set_callstack_ptr(buf_len + 1);

        Ok(())
    }

    /// Runs program. Does not set program counter or load program data.
    pub fn run(&mut self) -> Result<(), StackError> {
        loop {
            let tryte: [Trit; 3] = self.machine.stack.get_at_pc(self.machine.program_counter).into();

            // decode trytecode into instructions 
            match tryte {
                // lsh
                [Trit::Zero, Trit::Zero, Trit::Zero] => self.machine.lsh().unwrap(),
                // rsh
                [Trit::Zero, Trit::Zero, Trit::POne] => self.machine.rsh().unwrap(),
                // neg
                [Trit::Zero, Trit::Zero, Trit::NOne] => self.machine.neg().unwrap(),
                // and
                [Trit::Zero, Trit::POne, Trit::Zero] => self.machine.and().unwrap(),
                // or
                [Trit::Zero, Trit::POne, Trit::POne] => self.machine.or().unwrap(),
                // xor
                [Trit::Zero, Trit::POne, Trit::NOne] => self.machine.xor().unwrap(),
                // add
                [Trit::Zero, Trit::NOne, Trit::Zero] => self.machine.add().unwrap(),
                // sub
                [Trit::Zero, Trit::NOne, Trit::POne] => self.machine.sub().unwrap(),
                // mul
                [Trit::Zero, Trit::NOne, Trit::NOne] => self.machine.mul().unwrap(),
                // add3
                [Trit::POne, Trit::NOne, Trit::Zero] => self.machine.add3().unwrap(),
                // sub3
                [Trit::POne, Trit::NOne, Trit::POne] => self.machine.sub3().unwrap(),
                // mul3
                [Trit::POne, Trit::NOne, Trit::NOne] => self.machine.mul3().unwrap(),
                // add9
                [Trit::NOne, Trit::NOne, Trit::Zero] => self.machine.add9().unwrap(),
                // sub9
                [Trit::NOne, Trit::NOne, Trit::POne] => self.machine.sub9().unwrap(),
                // mul9
                [Trit::NOne, Trit::NOne, Trit::NOne] => self.machine.mul9().unwrap(),
                // cmp
                [Trit::POne, Trit::Zero, Trit::Zero] => self.machine.cmp().unwrap(),
                // br
                [Trit::POne, Trit::Zero, Trit::POne] => self.machine.br().unwrap(),
                // bne
                [Trit::POne, Trit::Zero, Trit::NOne] => self.machine.bne().unwrap(),
                // bgt
                [Trit::POne, Trit::POne, Trit::Zero] => self.machine.bgt().unwrap(),
                // blt
                [Trit::POne, Trit::POne, Trit::POne] => self.machine.blt().unwrap(),
                // beq
                [Trit::POne, Trit::POne, Trit::NOne] => self.machine.beq().unwrap(),
                // pt
                [Trit::NOne, Trit::Zero, Trit::Zero] => {
                    let slice = &self.machine.stack.callstack[(self.machine.program_counter + 1)..=(self.machine.program_counter + 1)];
                    self.machine.pt(slice[0]).unwrap();
                },
                // pth
                [Trit::NOne, Trit::Zero, Trit::POne] => {
                    let slice = &self.machine.stack.callstack[(self.machine.program_counter + 1)..=(self.machine.program_counter + 3)];
                    self.machine.pth([slice[0], slice[1], slice[2]]).unwrap();
                },
                // pw
                [Trit::NOne, Trit::Zero, Trit::NOne] => {
                    let slice = &self.machine.stack.callstack[(self.machine.program_counter + 1)..=(self.machine.program_counter + 9)];
                    self.machine.pw(
                        [slice[0], slice[1], slice[2],
                         slice[3], slice[4], slice[5],
                         slice[6], slice[7], slice[8]])
                        .unwrap();
                },
                // pct
                [Trit::NOne, Trit::POne, Trit::Zero] => {
                    let slice = &self.machine.stack.callstack[(self.machine.program_counter + 1)..=(self.machine.program_counter + 1)];
                    self.machine.pct(slice[0]).unwrap();
                },
                // pcth
                [Trit::NOne, Trit::POne, Trit::POne] => {
                    let slice = &self.machine.stack.callstack[(self.machine.program_counter + 1)..=(self.machine.program_counter + 3)];
                    self.machine.pcth([slice[0], slice[1], slice[2]]).unwrap();
                },
                // pcw
                [Trit::NOne, Trit::POne, Trit::NOne] => {
                    let slice = &self.machine.stack.callstack[(self.machine.program_counter + 1)..=(self.machine.program_counter + 9)];
                    self.machine.pcw(
                        [slice[0], slice[1], slice[2],
                         slice[3], slice[4], slice[5],
                         slice[6], slice[7], slice[8]]
                    ).unwrap();
                },
            }
        }
    }
}
