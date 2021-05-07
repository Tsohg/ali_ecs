use std::sync::RwLock;

use super::eid_manager::*;
use super::*;

pub struct ComponentManager {
    components: Vec<Vec<RwLock<Component>>>,
    packed_components: Vec<RwLock<Vec<usize>>>, //Component -> eids using it.
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
        for _i in 0..Find::VARIANT_COUNT {
            cm.components.push(vec![]);
            cm.packed_components.push(RwLock::new(vec![]));
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
        (entity.component_bitmask & bit) == bit
    }

    //Reserves memory for an entity's components.
    pub fn reserve(&mut self) {
        for vec in self.components.iter_mut() {
            vec.push(RwLock::new(Component::None));
        }
    }

    //Adds an entry in the packed array for the given entity.
    pub fn pack(&mut self, entity: &Entity, which: &Find) -> Result<(), ErrEcs> {
        match self.packed_components.get_mut(ComponentManager::get_component_id(which)) {
            Some(vec) => {
                vec.write().unwrap().push(entity.id);
                Ok(())
            },
            None => Err(ErrEcs::ComponentCategoryNotFound(format!("category: {:#?}", which)))
        }
    }

    //Removes an entry in the packed array for the given entity.
    pub fn unpack(&mut self, entity: &Entity, which: &Find) -> Result<(), ErrEcs>  {
        match self.packed_components.get_mut(ComponentManager::get_component_id(which)) {
            Some(vec) => {
                vec.write().unwrap().remove(entity.id);
                Ok(())
            },
            None => Err(ErrEcs::ComponentCategoryNotFound(format!("category: {:#?}", which)))
        }
    }

    //Frees the component slots at eid.
    pub fn free(&mut self, entity: &Entity) {
        for vec in self.components.iter_mut() {
            match vec.get_mut(entity.id) {
                Some(c) => *c = RwLock::new(Component::None),
                None => ()
            }
        }
    }

    //Returns a clone of the component data for reading.
    pub fn read_component(&self, entity: &Entity, which: Find) -> Result<Component, ErrEcs> {
        match self.components.get(ComponentManager::get_component_id(&which)) {
            Some(vec) => match vec.get(entity.id) {
                Some(cmp) => Ok(*cmp.read().unwrap()),
                None => Err(ErrEcs::EidOutOfBounds(format!("eid: {}", entity.id)))
            }
            None => Err(ErrEcs::UnallocatedComponent(format!("component: {:#?}", which)))
        }
    }

    //Adds the given component to the entity.
    pub fn add_component(&mut self, entity: &mut Entity, which: Find, component: Component) -> Result<(), ErrEcs> {
        if ComponentManager::entity_has_component(entity, &which) {
            return Err(ErrEcs::ComponentAlreadyExists(format!("eid: {}, component: {:#?}", entity.id, which)))
        }
        self.pack(entity, &which)?;
        self.set_component(entity, &which, component)?;
        entity.component_bitmask |= ComponentManager::get_component_bit(&which);
        Ok(())
    }

    //Removes the given component from the entity.
    pub fn remove_component(&mut self, entity: &mut Entity, which: Find) -> Result<(), ErrEcs> {
        if !ComponentManager::entity_has_component(entity, &which) {
            return Err(ErrEcs::EntityComponentNotFound(format!("eid: {}, component: {:#?}", entity.id, which)))
        }
        self.unpack(entity, &which)?;
        self.set_component(entity, &which, Component::None)?;
        entity.component_bitmask -= ComponentManager::get_component_bit(&which);
        Ok(())
    }

    //Sets an entity's component.
    pub fn set_component(&mut self, entity: &Entity, which: &Find, component: Component) -> Result<(), ErrEcs> {
        match self.components.get_mut(ComponentManager::get_component_id(&which)) {
            Some(vec) => match vec.get_mut(entity.id) {
                Some(data) => {
                    *data = RwLock::new(component);
                    Ok(())
                },
                None => Err(ErrEcs::EidOutOfBounds(format!("eid: {}", entity.id)))
            },
            None => Err(ErrEcs::UnallocatedComponent(format!("component: {:#?}", which)))
        }
    }
}
