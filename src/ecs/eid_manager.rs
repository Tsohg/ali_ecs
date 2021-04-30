extern crate queues;
use queues::*;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: usize,
    pub component_bitmask: u32,
    pub owner: String,
}

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
    pub fn create(&mut self, user: &str) -> Entity {
        match self.eid_q.remove() {
            Ok(eid) => {
                Entity {
                    id: eid,
                    component_bitmask: 0,
                    owner: String::from(user),
                }
            },
            Err(_) => {
                self.eid_max += 1;
                Entity {
                    id: self.eid_max,
                    component_bitmask: 0,
                    owner: String::from(user),
                }
            }
        }
    }

    //Make an entity available and give up ownership of it.
    pub fn free(&mut self, entity: &Entity) {
        self.eid_q.add(entity.id).expect("Attempt to free a missing entity.");
    }
}
