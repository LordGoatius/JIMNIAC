use std::{array, sync::{Arc, atomic::{AtomicBool, AtomicU32, AtomicU64}}};

use crossbeam_utils::CachePadded;
use ternary::{tryte::Tryte, word::Word};

/// Ports also acts as the interrupt controller for the I/O going through it
pub struct Ports {
    // a `u64` us used instead of a `Word` because Rust's atomic
    // capabilities for custom types isn't quite there yet
    // NOTE: Don't care about cache contention here. I don't expect more than
    // one port to be used for this version.
    ports: [AtomicU64; Tryte::TRYTE_SIZE],
    interrupts: Arc<CachePadded<AtomicBool>>,
    interrupt_num: Arc<CachePadded<AtomicU32>>,
}

impl Ports {
    pub fn init<'a>(interrupts: Arc<CachePadded<AtomicBool>>, interrupt_num: Arc<CachePadded<AtomicU32>>) -> Ports {
        Ports {
            ports: array::from_fn(|_| AtomicU64::new(Word::ZERO.num())),
            interrupts,
            interrupt_num,
        }
    }
}

