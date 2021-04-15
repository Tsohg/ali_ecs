use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

mod eid_manager;
mod component_manager;
mod c_data;

pub use self::eid_manager::*;
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
    pub fn create_for(&mut self, user: &str) -> Entity {
        let entity = self.eid_manager.create();
        self.component_manager.reserve();

        if let Some(owned) = self.entity_ownership.get_mut(&ECS::hash(user)) {
            owned.push(entity.id);
        } else {
            self.entity_ownership.insert(ECS::hash(user), vec![entity.id]);
        }
        entity
    }

    //Frees an eid from a user.
    pub fn free(&mut self, entity: Entity, user: &str) {
        //remove an eid from the owned eids of a user.
        if let Some(owned) = self.entity_ownership.get_mut(&ECS::hash(user)) {
            owned.remove(entity.id);
        }

        //free component slots and make eid available.
        self.component_manager.free(&entity);
        self.eid_manager.free(&entity);
    }

    //Checks to see if a user owns a particular eid.
    pub fn authenticate(&self, entity: &Entity, user: &str) -> Result<(), ErrCm> {
        if let Some(owned) = self.entity_ownership.get(&ECS::hash(user)) {
            if owned.iter().any(|&owned_eid| owned_eid == entity.id) {
                Ok(())
            } else {
                Err(ErrCm::UserDoesNotOwn(format!("user: {}, eid: {}", user, &entity.id)))
            }
        } else {
            Err(ErrCm::UserNotFound(format!("user: {}", user)))
        }
    }

    pub fn get_component(&self, entity: &Entity, user: &str, which: Find) -> Result<&Component, ErrCm> {
        match self.authenticate(entity, user) {
            Ok(_) => self.component_manager.get_component(entity, &which),
            Err(msg) => Err(msg)
        }
    }

    pub fn get_component_mut(&mut self, entity: &Entity, user: &str, which: Find) -> Result<&mut Component, ErrCm> {
        match self.authenticate(entity, user) {
            Ok(_) => self.component_manager.get_component_mut(entity, &which),
            Err(msg) => Err(msg)
        }
    }
}
