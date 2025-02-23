use std::sync::{mpmc::{self, Receiver, Sender, TryRecvError}, Arc, Mutex};

use itertools::Itertools;
use ternary::tryte::Tryte;

// NOTE: use std::sync::mpmc::Receiver;

#[derive(Debug)]
pub struct Ports(Vec<Port>);

#[derive(Debug)]
pub struct Port {
    port_in: Receiver<Tryte>,
    port_out: Sender<Tryte>,
}

impl Default for Ports {
    fn default() -> Self {
        Ports(vec![mpmc::channel(); 5].into_iter().map(|(port_out, port_in)| Port { port_in, port_out }).collect_vec())
    }
}

impl Ports {
    pub fn try_out(&self, port: Tryte, data: Tryte) -> Option<Tryte> {
        let port: isize = (port.into()) ;
        let port = port + 9841;

        self.0.get(port as usize)?.port_out.send(data);
        Some(data)
    }

    pub fn try_in(&self, port: Tryte) -> Option<Tryte> {
        let port: isize = (port.into()) ;
        let port = port + 9841;
        self.0.get(port as usize)?.port_in.try_recv().ok()
    }
}
