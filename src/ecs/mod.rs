mod eid_manager;
pub mod component_manager;

use self::eid_manager::EidManager;

use self::component_manager::*; //brings in all data types as well.

//need to find a way to scope components with a component enum in scope here as well for a general get_component i think.
//pub use crate::ecs::component_manager::vector2::Vector2;

//Wrapper for EidManager and ComponentManager.
pub struct ECS {
    eid_manager: EidManager,
    component_manager: ComponentManager
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            eid_manager: EidManager::new(),
            component_manager: ComponentManager::new()
        }
    }

    pub fn create_eid(&mut self) -> usize {
        self.eid_manager.create()
    }

    pub fn get_position(&self, eid: &usize) {
        ()
    }

    pub fn set_position(&mut self, eid: &usize, position: Option<Vector2>) {
        ()
    }
}
