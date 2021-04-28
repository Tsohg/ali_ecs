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
    pub fn free(&mut self, entity: Entity, user: &str) { //Entity should go out of scope here and be dropped.
        //remove an eid from the owned eids of a user.
        if let Some(owned) = self.entity_ownership.get_mut(&ECS::hash(user)) {
            owned.remove(entity.id);
        }

        //free component slots and make eid available.
        self.component_manager.free(&entity);
        self.eid_manager.free(&entity);
    }

    //Checks to see if a user owns a particular eid.
    pub fn authenticate(&self, entity: &Entity, user: &str) -> Result<(), ErrEcs> {
        if let Some(owned) = self.entity_ownership.get(&ECS::hash(user)) {
            if owned.iter().any(|&owned_eid| owned_eid == entity.id) {
                Ok(())
            } else {
                Err(ErrEcs::UserDoesNotOwn(format!("user: {}, eid: {}", user, &entity.id)))
            }
        } else {
            Err(ErrEcs::UserNotFound(format!("user: {}", user)))
        }
    }

    pub fn get_component(&self, entity: &Entity, user: &str, which: Find) -> Result<&Component, ErrEcs> {
        self.authenticate(entity, user)?;
        self.component_manager.get_component(entity, &which)
    }

    pub fn get_component_mut(&mut self, entity: &Entity, user: &str, which: Find) -> Result<&mut Component, ErrEcs> {
        self.authenticate(entity, user)?;
        self.component_manager.get_component_mut(entity, &which)
    }

    pub fn add_component(&mut self, entity: &mut Entity, user: &str, which: Find, component: Component) -> Result<(), ErrEcs> {
        self.authenticate(entity, user)?;
        self.component_manager.add_component(entity, which, component)
    }

    pub fn update_component(&mut self, entity: &Entity, user: &str, which: Find, component: Component) -> Result<(), ErrEcs> {
        self.authenticate(entity, user)?;
        self.component_manager.update_component(entity, which, component)
    }

    pub fn remove_component(&mut self, entity: &mut Entity, user: &str, which: Find) -> Result<(), ErrEcs> {
        self.authenticate(entity, user)?;
        self.component_manager.remove_component(entity, which)
    }
}

#[derive(Debug)]
pub enum ErrEcs {
    //Ecs
    UserNotFound(String),
    UserDoesNotOwn(String),

    //Component Manager
    UnallocatedComponent(String),
    EidOutOfBounds(String),
    ComponentAlreadyExists(String),
    EntityComponentNotFound(String),
    ComponentCategoryNotFound(String),
}
