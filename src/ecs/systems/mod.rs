use std::sync::mpsc;

pub trait System {
    fn start(&self) -> Send; //returns the transmitter to the system thread.
    fn stop(&self, tx: Send); //stops the system thread and takes ownership of the transmitter returned from start() so that it goes out of scope.
}

pub enum Systems {
    Position,
}
