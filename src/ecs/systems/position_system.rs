use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;

use super::*;

pub struct PositionSystem {}

impl SystemStart for PositionSystem {
    fn start() -> Box<Sender<SystemMessage>> {
        let (tx, rx): (Sender<SystemMessage>, Receiver<SystemMessage>) = mpsc::channel();
        thread::spawn(move|| {
            loop {
                match rx.recv() {
                    Ok(msg) => {
                        match msg {
                            SystemMessage::Stop() => break,
                            SystemMessage::Pos2Set(en, v2) => { //TODO: Replace with function calls.
                                //cm.update_component(&en, Find::Pos2, Component::Pos2(v2)); //PROBLEM
                            },
                            _ => println!("Catch all: {:#?}", msg),
                        }
                    },
                    Err(e) => println!("{}", e)
                }
            }
        });
        Box::new(tx)
    }
}
