extern crate queues;
use queues::*;

pub struct EidManager {
    eid_max: usize,
    eid_q: Queue<usize>
}

impl EidManager {
    pub fn new() -> EidManager {
        EidManager {
            eid_max: 0,
            eid_q: queue![0]
        }
    }

    //Returns the eid for an entity and a hash representing the user.
    pub fn create(&mut self) -> usize {
        match self.eid_q.remove() {
            Ok(eid) => eid,
            Err(_) => {
                self.eid_max += 1;
                self.eid_max
            }
        }
    }

    //Make an EID available and give up ownership of it.
    pub fn free(&mut self, eid: usize) {
        self.eid_q.add(eid);
    }
}
