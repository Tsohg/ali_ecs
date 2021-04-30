use std::sync::mpsc::Sender;

pub mod messages;
pub use messages::SystemMessage;
pub mod position_system;
pub use position_system::PositionSystem;

pub trait SystemStart {
    fn start() -> Box<Sender<SystemMessage>>; //returns the transmitter to the system thread.
}
