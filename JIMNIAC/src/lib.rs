#![allow(dead_code)]
#![feature(mpmc_channel)]
#![feature(test)]

pub mod tryte;
pub mod word;
pub mod cpu;
pub mod stack;
pub use septivigntimal;

use ternary::trits::Trit;

pub trait GetStatus {
    fn get_parity(&self) -> Trit;
    fn get_sign(&self) -> Trit;
}
