use std::sync::{mpsc::{Receiver, Sender}, Arc, Mutex};
// NOTE: use std::sync::mpmc::Receiver;

use ternary::{tryte::Tryte, word::Word};

pub trait Ternary {}

impl Ternary for Word {}
impl Ternary for Tryte {}

#[derive(Debug, Default)]
pub struct Ports(Vec<Ports>);

// impl Default for Ports {
//     fn default() -> Self {
//         Ports(vec![])
//     }
// }

#[derive(Default, Debug)]
pub struct Port<T: Ternary> {
    send_in: Option<Sender<T>>,
    send_ou: Option<Receiver<T>>,
}
