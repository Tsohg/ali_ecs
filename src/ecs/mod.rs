mod eid_manager;
pub mod component_manager;

use self::eid_manager::EidManager;

use self::component_manager::*;

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

    pub fn get_component(&mut self, eid: &usize, component: Component) -> Result<&mut Component, ErrCm> {
        self.component_manager.get_component(&eid, &component)
    }

    pub fn add_component(&mut self, eid: &usize, component: Component) -> Result<(), ErrCm> {
        self.component_manager.add_component(&eid, component)
    }
}
