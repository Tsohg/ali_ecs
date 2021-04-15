use std::collections::HashMap;

use super::c_data::*;
use super::eid_manager::*;

/*
* How to Add a new component:
*   Add the component data structure to c_data.rs (or maybe later, a c_data directory).
*   Add the component name to the enum Component with its data structure as a parameter.
*   Add the component name to the enum Find without parameters.
*   In ComponentManager::new() => Insert the new component into the hashmap using Find::component_name as the key, and an empty data_structure as the argument.
*       {Denoted by comment in Component::new()}
*/

/*
* TODO:
*    Implement a component id system which is an index into components.
*    Implement support for the Find method to map it to a component id.
*    Finish implementing add, remove, update using the bitmask.
*       Entities should have, at most, 1 component per type.
*    Update documentation on how to add a new component.
*    Implement an input system that fires a function when a certain input is given.
*       Should be mappable via a function: fn(input, fn)
*/


//Container for a component's data type.
#[derive(Debug)]
pub enum Component {
    Position(Vector2),
    None
}

//Used for component searching in the hash map.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Find {
    Position,
}

pub struct ComponentManager {
    components: HashMap<Find, Vec<Component>>,
}

//Define getter/setter for each component.
impl ComponentManager {
    pub fn new() -> ComponentManager {
        let mut cm = ComponentManager {
            components: HashMap::new(),
        };

        //Insert all components here.
        cm.components.insert(Find::Position, vec![]);
        cm
    }

    //Adds 1 new element to each component vector initialized to None.
    pub fn reserve(&mut self) {
        for (_k, v) in self.components.iter_mut() {
            v.push(Component::None);
        }
    }

    //Frees the component slots at eid.
    pub fn free(&mut self, entity: &Entity) {
        for(_k, vec) in self.components.iter_mut() {
            match vec.get_mut(entity.id) {
                Some(comp) => *comp = Component::None,
                None => ()
            }
        }
    }

    pub fn get_component_mut(&mut self, entity: &Entity, which: &Find) -> Result<&mut Component, ErrCm> {
        match self.components.get_mut(which) {
            Some(vec) => match vec.get_mut(entity.id) {
                Some(data) => {
                    Ok(data)
                },
                None => Err(ErrCm::EidOutOfBounds(format!("eid: {}", entity.id)))
            },
            None => Err(ErrCm::ComponentNotFound(format!("component: {:#?}", which)))
        }
    }

    pub fn get_component(&self, entity: &Entity, which: &Find) -> Result<&Component, ErrCm> { //Can't seem to find a way to not reproduce the same code as get_component_mut.
        match self.components.get(which) {
            Some(vec) => match vec.get(entity.id) {
                Some(data) => {
                    Ok(data)
                },
                None => Err(ErrCm::EidOutOfBounds(format!("eid: {}", entity.id)))
            },
            None => Err(ErrCm::ComponentNotFound(format!("component: {:#?}", which)))
        }
    }

    //WIP
    pub fn add_component(&mut self, entity: &Entity, which: Find, component: Component) -> Result<(), ErrCm> {
        match self.get_component_mut(entity, &which) {
            Ok(comp) => match comp {
                Component::None => {
                    *comp = component;
                    Ok(())
                },
                _ => Err(ErrCm::ComponentAlreadyExists(format!("eid: {}, component: {:#?}", entity.id, &component)))
            }
            Err(e) => Err(e)
        }
    }

    //WIP
    pub fn update_component(&mut self, entity: &Entity, which: Find, with: Component) -> Result<(), ErrCm> {
        match self.get_component_mut(entity, &which) {
            Ok(comp) => {
                *comp = with;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    //WIP
    pub fn remove_component(&mut self, entity: &Entity, which: Find) -> Result<(), ErrCm> {
        self.update_component(entity, which, Component::None)
    }
}

#[derive(Debug)]
pub enum ErrCm {
    ComponentNotFound(String),
    EidOutOfBounds(String),
    ComponentAlreadyExists(String),

    //Used in ECS.
    UserNotFound(String),
    UserDoesNotOwn(String),
}
