use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub mod messages;
pub use messages::*;

pub mod position_system;
pub use position_system::PositionSystem;

pub trait System<T> {
    fn start() -> Box<Sender<T>>; //returns the transmitter to the system thread.
}
