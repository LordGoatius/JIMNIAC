#![allow(dead_code)]
#![feature(test)]

use trits::Trit;
pub mod trits;
pub mod tryte;
pub mod word;
pub mod cpu;
pub mod stack;
pub mod septivigntimal;

pub trait GetStatus {
    fn get_parity(&self) -> Trit;
    fn get_sign(&self) -> Trit;
}
