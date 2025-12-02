use std::sync::atomic::AtomicU64;

use ternary::tryte::Tryte;

pub struct Ports {
    // a `u64` us used instead of a `Word` because Rust's atomic
    // capabilities for custom types isn't quite there yet
    ports: [AtomicU64; Tryte::TRYTE_SIZE],
}
