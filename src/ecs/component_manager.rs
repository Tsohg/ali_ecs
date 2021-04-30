use std::collections::HashMap;

use super::c_data::*;
use super::eid_manager::*;
use super::*;

pub struct ComponentManager {
    components: Vec<Vec<Component>>,
    packed_components: Vec<Vec<usize>>, //Component -> eids using it.
}

//Define getter/setter for each component.
impl ComponentManager {
    pub fn new() -> ComponentManager {
        let mut cm = ComponentManager {
            components: vec![],
            packed_components: vec![],
        };

        assert!(Find::VARIANT_COUNT == Component::VARIANT_COUNT - 1);

        //Initialize vectors to the number of total components based on the length of Find enum.
        for i in 0..Find::VARIANT_COUNT {
            cm.components.push(vec![]);
            cm.packed_components.push(vec![]);
        }
        cm
    }

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
        ((entity.component_bitmask & bit) == bit)
    }

    //Reserves memory for an entity's components.
    pub fn reserve(&mut self) {
        for vec in self.components.iter_mut() {
            vec.push(Component::None);
        }
    }

    //Adds an entry in the packed array for the given entity.
    pub fn pack(&mut self, entity: &Entity, which: &Find) -> Result<(), ErrEcs> {
        match self.packed_components.get_mut(ComponentManager::get_component_id(which)) {
            Some(vec) => {
                vec.push(entity.id);
                Ok(())
            },
            None => Err(ErrEcs::ComponentCategoryNotFound(format!("category: {:#?}", which)))
        }
    }

    //Removes an entry in the packed array for the given entity.
    pub fn unpack(&mut self, entity: &Entity, which: &Find) -> Result<(), ErrEcs>  {
        match self.packed_components.get_mut(ComponentManager::get_component_id(which)) {
            Some(vec) => {
                vec.remove(entity.id);
                Ok(())
            },
            None => Err(ErrEcs::ComponentCategoryNotFound(format!("category: {:#?}", which)))
        }
    }

    //Returns an immutable packed array of eids for a given component.
    pub fn get_packed_components(&self, which: Find) -> Result<&Vec<usize>, ErrEcs> {
        match self.packed_components.get(ComponentManager::get_component_id(&which)) {
            Some(vec) => Ok(vec),
            None => Err(ErrEcs::ComponentCategoryNotFound(format!("category: {:#?}", which)))
        }
    }

    //Frees the component slots at eid.
    pub fn free(&mut self, entity: &Entity) {
        for vec in self.components.iter_mut() {
            match vec.get_mut(entity.id) {
                Some(c) => *c = Component::None,
                None => ()
            }
        }
        //entity.component_bitmask = 0; //Required if entity does not go out of scope in ecs/mod.rs
    }

    //Returns a mutable component from the specified entity.
    pub fn get_component_mut(&mut self, entity: &Entity, which: &Find) -> Result<&mut Component, ErrEcs> {
        match self.components.get_mut(ComponentManager::get_component_id(which)) {
            Some(vec) => match vec.get_mut(entity.id) {
                Some(data) => {
                    Ok(data)
                },
                None => Err(ErrEcs::EidOutOfBounds(format!("eid: {}", entity.id)))
            },
            None => Err(ErrEcs::UnallocatedComponent(format!("component: {:#?}", which)))
        }
    }

    //Returns an immutable component from the specified entity.
    pub fn get_component(&self, entity: &Entity, which: &Find) -> Result<&Component, ErrEcs> { //Can't seem to find a way to not reproduce the same code as get_component_mut.
        match self.components.get(ComponentManager::get_component_id(which)) {
            Some(vec) => match vec.get(entity.id) {
                Some(data) => {
                    Ok(data)
                },
                None => Err(ErrEcs::EidOutOfBounds(format!("eid: {}", entity.id)))
            },
            None => Err(ErrEcs::UnallocatedComponent(format!("component: {:#?}", which)))
        }
    }

    pub fn add_component(&mut self, entity: &mut Entity, which: Find, component: Component) -> Result<(), ErrEcs> {
        if ComponentManager::entity_has_component(entity, &which) {
            return Err(ErrEcs::ComponentAlreadyExists(format!("eid: {}, component: {:#?}", entity.id, which)))
        }

        self.pack(entity, &which);

        entity.component_bitmask |= ComponentManager::get_component_bit(&which);

        match self.get_component_mut(entity, &which) {
            Ok(comp) => match comp {
                Component::None => {
                    *comp = component;
                    Ok(())
                },
                _ => Err(ErrEcs::ComponentAlreadyExists(format!("eid: {}, component: {:#?}", entity.id, which)))
            }
            Err(e) => Err(e)
        }
    }

    pub fn update_component(&mut self, entity: &Entity, which: Find, with: Component) -> Result<(), ErrEcs> {
        if !ComponentManager::entity_has_component(entity, &which) {
            return Err(ErrEcs::EntityComponentNotFound(format!("eid: {}, component: {:#?}", entity.id, which)))
        }

        match self.get_component_mut(entity, &which) {
            Ok(comp) => {
                *comp = with;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn remove_component(&mut self, entity: &mut Entity, which: Find) -> Result<(), ErrEcs> {
        if !ComponentManager::entity_has_component(entity, &which) {
            return Err(ErrEcs::EntityComponentNotFound(format!("eid: {}, component: {:#?}", entity.id, which)))
        }

        self.unpack(entity, &which);

        let diff = ComponentManager::get_component_bit(&which);
        let result = self.update_component(entity, which, Component::None);
        entity.component_bitmask -= diff;
        result
    }
}
