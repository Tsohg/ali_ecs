use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

mod eid_manager;
mod component_manager;
mod c_data;

use self::eid_manager::EidManager;
pub use self::component_manager::*;
pub use self::c_data::*;

//Wrapper for EidManager and ComponentManager that utilizies their functions correctly.
pub struct ECS {
    eid_manager: EidManager,
    component_manager: ComponentManager,
    //user hash -> eid.
    entity_ownership: HashMap<u64, Vec<usize>>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            eid_manager: EidManager::new(),
            component_manager: ComponentManager::new(),
            entity_ownership: HashMap::new()
        }
    }

    fn hash(user: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(user.as_bytes());
        hasher.finish()
    }

    //Creates and returns an eid mapped to a user.
    pub fn create_for(&mut self, user: &str) -> usize {
        let eid = self.eid_manager.create();
        self.component_manager.reserve();

        if let Some(owned) = self.entity_ownership.get_mut(&ECS::hash(user)) {
            owned.push(eid);
        } else {
            self.entity_ownership.insert(ECS::hash(user), vec![eid]);
        }

        eid
    }

    //Frees an eid from a user.
    pub fn free(&mut self, eid: usize, user: &str) {
        //remove an eid from the owned eids of a user.
        if let Some(owned) = self.entity_ownership.get_mut(&ECS::hash(user)) {
            owned.remove(eid);
        }

        //free component slots and make eid available.
        self.component_manager.free(&eid);
        self.eid_manager.free(eid);
    }

    //Checks to see if a user owns a particular eid.
    pub fn authenticate(&self, eid: &usize, user: &str) -> Result<(), ErrCm> {
        if let Some(owned) = self.entity_ownership.get(&ECS::hash(user)) {
            if owned.iter().any(|&owned_eid| owned_eid == *eid) {
                Ok(())
            } else {
                Err(ErrCm::UserDoesNotOwn(format!("user: {}, eid: {}", user, eid)))
            }
        } else {
            Err(ErrCm::UserNotFound(format!("user: {}", user)))
        }
    }

    pub fn get_component(&self, eid: &usize, user: &str, which: Find) -> Result<&Component, ErrCm> {
        match self.authenticate(eid, user) {
            Ok(_) => self.component_manager.get_component(eid, which),
            Err(msg) => Err(msg)
        }
    }

    pub fn get_component_mut(&mut self, eid: &usize, user: &str, which: Find) -> Result<&mut Component, ErrCm> {
        match self.authenticate(eid, user) {
            Ok(_) => self.component_manager.get_component_mut(eid, which),
            Err(msg) => Err(msg)
        }
    }

    pub fn set_component(&mut self, eid: &usize, user: &str, which: Find, component: Component) -> Result<(), ErrCm> {
        match self.authenticate(eid, user) {
            Ok(_) => self.component_manager.set_component(eid, which, component),
            Err(msg) => Err(msg)
        }
    }
}
