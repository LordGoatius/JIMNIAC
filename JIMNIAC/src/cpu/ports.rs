use std::sync::{mpsc::{Receiver, Sender, TryRecvError}, Arc, Mutex};

use ternary::tryte::Tryte;
// NOTE: use std::sync::mpmc::Receiver;


#[derive(Debug, Default)]
pub struct Ports(Vec<Port>);

#[derive(Debug)]
pub struct Port {
    port_in: Receiver<Tryte>,
    port_out: Sender<Tryte>,
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
