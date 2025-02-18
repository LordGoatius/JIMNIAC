#![feature(mpmc_channel)]
use std::{ops::{Deref, DerefMut}, sync::mpmc::{self, Receiver, Sender}, thread::sleep, time::Duration};
// NOTE: use std::sync::mpmc::Receiver;

use septivigntimal::*;
use ternary::{tryte::Tryte, word::Word};

pub trait Ternary {}

impl Ternary for Word {}
impl Ternary for Tryte {}

#[derive(Debug, Default)]
pub struct Ports(Vec<Port>);

impl Deref for Ports {
    type Target = Vec<Port>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl Default for Ports {
//     fn default() -> Self {
//         Ports(vec![])
//     }
// }

#[derive(Debug)]
pub struct Port {
    send_in: Sender<Tryte>,
    send_out: Receiver<Tryte>,
}


pub struct Cpu {
    interrupts: (Sender<usize>, Receiver<usize>),
    ports: Ports,
}

fn new_ports() -> Ports {
    let (s0, r0) = mpmc::channel::<Tryte>();
    let p0 = Port { send_in: s0, send_out: r0 };
    let (s1, r1) = mpmc::channel::<Tryte>();
    let p1 = Port { send_in: s1, send_out: r1 };
    let (s2, r2) = mpmc::channel::<Tryte>();
    let p2 = Port { send_in: s2, send_out: r2 };
    Ports(vec![p0, p1, p2])
}

impl Cpu {
    fn interrupts(self) {
        use std::io::{stdin, stdout, Write};
        use std::thread;
        use termion::event::Key;
        use termion::input::TermRead;
        use termion::raw::IntoRawMode;

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(
            stdout,
            // "{}{}q to exit. Type stuff, use alt, and so on.{}",
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        thread::scope(|s| {
            s.spawn(|| {
                for k in stdin.keys() {
                    match k.as_ref().unwrap() {
                        Key::Char('a') => {
                            self.interrupts.0.send(0).unwrap();
                            self.ports[0].send_in.send([O, N, E].into()).unwrap();
                        },
                        Key::Char('b') => {
                            self.interrupts.0.send(1).unwrap();
                            self.ports[1].send_in.send([T, W, O].into()).unwrap();
                        },
                        Key::Ctrl('c') => {
                            self.interrupts.0.send(3).unwrap();
                            break;
                        }
                        _ => {
                            self.interrupts.0.send(2).unwrap();
                            self.ports[2].send_in.send(Tryte::default()).unwrap();
                        },
                    }
                }
            });
            s.spawn(|| loop {
                match self.interrupts.1.try_recv() {
                    Err(_) => continue,
                    Ok(val @ 0..3) => {
                        // The time from the interrupt to port being sent needs to be synced
                        // Does not take much time lol, but it is necessary
                        sleep(Duration::from_nanos(1));
                        if let Ok(msg) = self.ports[val].send_out.try_recv() {
                            print!("{:#}", msg);
                            stdout.flush().unwrap();
                        }
                    },
                    Ok(3) => break,
                    _ => {
                        print!("recv non match");
                        stdout.flush().unwrap();
                        continue;
                    },
                }
            });
        });

        write!(stdout, "{}", termion::cursor::Show).unwrap();
        print!("Exit\n");
        stdout.suspend_raw_mode().unwrap();
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::mpmc;

    use crate::{new_ports, Cpu};

    #[test]
    fn test_interrupt_signals() {
        let cpu = Cpu {
            interrupts: mpmc::channel(),
            ports: new_ports(),
        };

        cpu.interrupts();
    }
}
