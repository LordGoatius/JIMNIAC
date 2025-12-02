use std::sync::atomic::Atomic;

use ternary::tryte::Tryte;

pub struct Ports {
    // a `u64` us used instead of a `Word` because Rust's atomic
    // capabilities for custom types isn't quite there yet
    ports: [Atomic<u64>; Tryte::TRYTE_SIZE],
}
