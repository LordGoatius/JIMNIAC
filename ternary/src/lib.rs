#![allow(dead_code)]
#![allow(clippy::assign_op_pattern)]
#![feature(ascii_char, stmt_expr_attributes, test)]

#[macro_use]
extern crate static_assertions;

#[macro_use]
extern crate approx;

pub mod trits;
pub mod tryte;
pub mod word;
pub mod errors;
pub mod prelude;
// pub mod floating;
#[cfg(feature = "const_size")]
pub mod varsize;

//== Consts ==//
const TRYTE_BIT_LEN: usize = 18;
const WORD_BIT_LEN: usize = 54;

pub const TRIT_BIT_MASK: u8 = 0b11;
pub const TRYTE_BIT_MASK: u32 = 0b111111111111111111;
pub const WORD_BIT_MASK: u64 = 0b111111111111111111111111111111111111111111111111111111;
pub const WORD_ZERO_TOP: u64 = 0b101010101010101010101010101010101010000000000000000000;
pub const TRYTE_LEN: usize = 9;
pub const WORD_LEN: usize = 27;
