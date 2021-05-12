use std::sync::RwLock;

use super::eid_manager::*;
use super::*;

pub struct ComponentManager {}

//Define getter/setter for each component.
impl ComponentManager {
    //Get the vector index of a particular set of components.
    pub fn get_component_id(which: &Find) -> usize {
        match which {
            Find::Pos2 => 0,
        }
    }

    //Get the bit in which a component is represented in the bitmask.
    pub fn get_component_bit(which: &Find) -> u32 {
        match which {
            Find::Pos2 => 1,
        }
    }

    //Returns true if the entity has the specified component.
    pub fn entity_has_component(entity: &Entity, which: &Find) -> bool {
        let bit = ComponentManager::get_component_bit(which);
        (entity.component_bitmask & bit) == bit
    }

    //Reserves memory for an entity's components.
    pub fn reserve(component_data: Arc<RwLock<ComponentData>>) {
        for vec in component_data.write().unwrap().components.iter_mut() {
            vec.push(RwLock::new(Component::None));
        }
    }

    //Adds an entry in the packed array for the given entity.
    pub fn pack(component_data: Arc<RwLock<ComponentData>>, entity: &Entity, which: &Find) -> Result<(), ErrEcs> {
        match component_data.read().unwrap().packed_components.get(ComponentManager::get_component_id(which)) {
            Some(vec) => {
                vec.write().unwrap().push(entity.id);
                Ok(())
            },
            None => Err(ErrEcs::ComponentCategoryNotFound(format!("category: {:#?}", which)))
        }
    }

    //Removes an entry in the packed array for the given entity.
    pub fn unpack(component_data: Arc<RwLock<ComponentData>>, entity: &Entity, which: &Find) -> Result<(), ErrEcs>  {
        match component_data.read().unwrap().packed_components.get(ComponentManager::get_component_id(which)) {
            Some(vec) => {
                vec.write().unwrap().remove(entity.id);
                Ok(())
            },
            None => Err(ErrEcs::ComponentCategoryNotFound(format!("category: {:#?}", which)))
        }
    }

    //Frees the component slots at eid.
    pub fn free(component_data: Arc<RwLock<ComponentData>>, entity: &Entity) {
        for vec in component_data.read().unwrap().components.iter() {
            match vec.get(entity.id) {
                Some(c) => *c.write().unwrap() = Component::None,
                None => ()
            }
        }

        for vec in component_data.read().unwrap().packed_components.iter() {
            if vec.read().unwrap().iter().any(|&eid| eid == entity.id) {
                vec.write().unwrap().remove(entity.id);
            }
        }
    }

    //Returns a clone of the component data for reading.
    pub fn read_component(component_data: Arc<RwLock<ComponentData>>, entity: &Entity, which: Find) -> Result<Component, ErrEcs> {
        match component_data.read().unwrap().components.get(ComponentManager::get_component_id(&which)) {
            Some(vec) => match vec.get(entity.id) {
                Some(cmp) => Ok(*cmp.read().unwrap()),
                None => Err(ErrEcs::EidOutOfBounds(format!("eid: {}", entity.id)))
            }
            None => Err(ErrEcs::UnallocatedComponent(format!("component: {:#?}", which)))
        }
    }

    //Adds the given component to the entity.
    pub fn add_component(component_data: Arc<RwLock<ComponentData>>, entity: &mut Entity, which: Find, component: Component) -> Result<(), ErrEcs> {
        if ComponentManager::entity_has_component(entity, &which) {
            return Err(ErrEcs::ComponentAlreadyExists(format!("eid: {}, component: {:#?}", entity.id, which)))
        }
        ComponentManager::pack(component_data.clone(), entity, &which)?;
        ComponentManager::set_component(component_data.clone(), entity, &which, component)?;
        entity.component_bitmask |= ComponentManager::get_component_bit(&which);
        Ok(())
    }

    //Removes the given component from the entity.
    pub fn remove_component(component_data: Arc<RwLock<ComponentData>>, entity: &mut Entity, which: Find) -> Result<(), ErrEcs> {
        if !ComponentManager::entity_has_component(entity, &which) {
            return Err(ErrEcs::EntityComponentNotFound(format!("eid: {}, component: {:#?}", entity.id, which)))
        }
        ComponentManager::unpack(component_data.clone(), entity, &which)?;
        ComponentManager::set_component(component_data.clone(), entity, &which, Component::None)?;
        entity.component_bitmask -= ComponentManager::get_component_bit(&which);
        Ok(())
    }

    //Sets an entity's component.
    pub fn set_component(component_data: Arc<RwLock<ComponentData>>, entity: &Entity, which: &Find, component: Component) -> Result<(), ErrEcs> {
        match component_data.read().unwrap().components.get(ComponentManager::get_component_id(&which)) {
            Some(vec) => match vec.get(entity.id) {
                Some(data) => {
                    *data.write().unwrap() = component;
                    Ok(())
                },
                None => Err(ErrEcs::EidOutOfBounds(format!("eid: {}", entity.id)))
            },
            None => Err(ErrEcs::UnallocatedComponent(format!("component: {:#?}", which)))
        }
    }
}
