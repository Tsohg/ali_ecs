use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use super::*;

//WIP
pub struct PositionSystem {}
impl System<Pos2Msg> for PositionSystem {
    fn start() -> Box<Sender<Pos2Msg>> {
        let (tx, rx): (Sender<Pos2Msg>, Receiver<Pos2Msg>) = mpsc::channel();
        thread::spawn(move|| {
            loop {
                match rx.recv_timeout(Duration::from_millis(1)) {
                    Ok(msg) => {
                        match msg {
                            Pos2Msg::Stop() => break,
                            _ => println!("{:#?}", msg),
                        }
                    },
                    Err(_) => () //ignore timeout errors.
                }
            }
        });
        Box::new(tx)
    }
}
