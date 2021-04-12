use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

mod eid_manager;
mod component_manager;
mod c_data;

use self::eid_manager::EidManager;
pub use self::component_manager::*;
pub use self::c_data::*;

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
    pub fn create_for(&mut self, user: &str) -> usize {
        let auth = ECS::hash(user);
        let eid = self.eid_manager.create();
        self.component_manager.reserve();

        if let Some(owned) = self.entity_owner.get_mut(&auth) {
            owned.push(eid);
        } else {
            self.entity_owner.insert(auth, vec![eid]);
        }

        eid
    }

    //Frees an eid from a user.
    pub fn free(&mut self, eid: usize, user: &str) {
        let auth = ECS::hash(user);

        //remove an eid from the owned eids of a user.
        if let Some(owned) = self.entity_owner.get_mut(&auth) {
            owned.remove(eid);
        }

        //free component slots and make eid available.
        self.component_manager.free(&eid);
        self.eid_manager.free(eid);
    }

    pub fn get_component_mut(&mut self, eid: &usize, user: &str, which: Find) -> Result<&mut Component, ErrCm> {
        self.component_manager.get_component_mut(eid, which)
    }

    pub fn set_component(&mut self, eid: &usize, user: &str, which: Find, component: Component) -> Result<(), ErrCm> {
        let auth = ECS::hash(user);

        if let Some(owned) = self.entity_owner.get_mut(&auth) {
            if owned.iter().any(|&owned_eid| owned_eid == *eid) {
                self.component_manager.set_component(eid, which, component)
            } else {
                Err(ErrCm::UserDoesNotOwn(format!("user: {}, eid: {}", user, eid)))
            }
        } else {
            Err(ErrCm::UserNotFound(format!("user: {}", user)))
        }
    }
}
