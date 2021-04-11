use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod eid_manager;
pub mod component_manager;

use self::eid_manager::EidManager;
use self::component_manager::*; //brings in all data types as well.

//Wrapper for EidManager and ComponentManager.
pub struct ECS {
    eid_manager: EidManager,
    component_manager: ComponentManager,
    //user hash -> eid.
    entity_owner: HashMap<u64, Vec<usize>>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            eid_manager: EidManager::new(),
            component_manager: ComponentManager::new(),
            entity_owner: HashMap::new()
        }
    }

    fn hash(user: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(user.as_bytes());
        hasher.finish()
    }

    //Creates and returns an eid mapped to a user.
    pub fn create(&mut self, user: &str) -> usize {
        let auth = ECS::hash(user);
        let eid = self.eid_manager.create();

        match self.entity_owner.get_mut(&auth) {
            Some(owned) => owned.push(eid),
            None => {
                self.entity_owner.insert(auth, vec![eid]);
                ()
            }
        }
        eid
    }

    //Frees an eid from a user.
    pub fn free(&mut self, eid: usize, user: &str) {
        let auth = ECS::hash(user);

        //remove entry from HashMap
        match self.entity_owner.get_mut(&auth) {
            Some(owned) => {
                owned.clear();
                self.entity_owner.remove(&auth);
            },
            None => ()
        }

        //TODO Next Time:
        //Clear all vector components. Probably need a better way of handling this.
        //Maybe register a number of function pointers for get() and iterate through?
        //  ^ Maybe return it from the component manager itself?...Geterator...
        match self.component_manager.get_position(&eid) {
            Some(v) => self.component_manager.set_position(&eid, None),
            None => ()
        }

        self.eid_manager.free(eid);
    }

    pub fn get_position(&self, eid: &usize) {
        ()
    }

    pub fn set_position(&mut self, eid: &usize, position: Option<Vector2>) {
        ()
    }
}
