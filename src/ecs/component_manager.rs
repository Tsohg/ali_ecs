use std::collections::HashMap;

use super::c_data::*;

/*
* How to Add a new component:
*   Add the component data structure to c_data.rs (or maybe later, a c_data directory).
*   Add the component name to the enum Component with its data structure as a parameter.
*   Add the component name to the enum Find without parameters.
*   In ComponentManager::new() => Insert the new component into the hashmap using Find::component_name as the key, and an empty data_structure as the argument.
*       {Denoted by comment in Component::new()}
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
    components : HashMap<Find, Vec<Component>>,
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
    pub fn free(&mut self, eid: &usize) {
        for(_k, vec) in self.components.iter_mut() {
            match vec.get_mut(*eid) {
                Some(comp) => *comp = Component::None,
                None => ()
            }
        }
    }

    pub fn get_component_mut(&mut self, eid: &usize, which: Find) -> Result<&mut Component, ErrCm> {
        match self.components.get_mut(&which) {
            Some(vec) => match vec.get_mut(*eid) {
                Some(data) => {
                    Ok(data)
                },
                None => Err(ErrCm::EidOutOfBounds(format!("eid: {}", eid)))
            },
            None => Err(ErrCm::ComponentNotFound(format!("component: {:#?}", which)))
        }
    }

    pub fn get_component(&self, eid: &usize, which: Find) -> Result<&Component, ErrCm> { //Can't seem to find a way to not reproduce the same code as get_component_mut.
        match self.components.get(&which) {
            Some(vec) => match vec.get(*eid) {
                Some(data) => {
                    Ok(data)
                },
                None => Err(ErrCm::EidOutOfBounds(format!("eid: {}", eid)))
            },
            None => Err(ErrCm::ComponentNotFound(format!("component: {:#?}", which)))
        }
    }

    pub fn set_component(&mut self, eid: &usize, which: Find, component: Component) -> Result<(), ErrCm> {
        match self.get_component_mut(eid, which) {
            Ok(comp) => {
                *comp = component;
                Ok(())
            },
            Err(msg) => Err(msg)
        }
    }
}

#[derive(Debug)]
pub enum ErrCm {
    ComponentNotFound(String),
    EidOutOfBounds(String),

    //Used in ECS.
    UserNotFound(String),
    UserDoesNotOwn(String),
}
