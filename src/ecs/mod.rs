/*

An entity component system meant for use in small 2d games.
Copyright (C) 2021 <Discord: Alias#2836>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

For more details see: <http://www.gnu.org/licenses/>.

*/

//stdlib
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

//externals
extern crate variant_count;
use variant_count::VariantCount;

//internals
mod eid_manager;
mod component_manager;
mod systems;
mod c_data;
pub use self::c_data::*; //all data structures for the components.
use self::systems::*;
use self::component_manager::*;
use self::eid_manager::*;


/*
* How to Add a new component:
*   Add the component data structure to c_data.rs (or maybe later, a c_data directory).
*   Add the component name to the enum Component with its data structure as a parameter.
*   Add the component name to the enum Find without parameters.
*
*   Compiler will complain until you:
*       Give the component an index in ComponentManager::get_component_id()
*       Give the component a bit in ComponentManager::get_component_bit()
*/

//Container for a component's data type.
#[derive(Debug, VariantCount)]
pub enum Component {
    Pos2(Vector2),
    None,
}

//Used for component searching in the hash map.
#[derive(Debug, PartialEq, Eq, Hash, VariantCount)]
pub enum Find {
    Pos2,
}

//Wrapper for EidManager and ComponentManager that utilizies their functions correctly.
pub struct ECS {
    eid_manager: EidManager,
    component_manager: ComponentManager,
    //user hash -> eid.
    entity_ownership: HashMap<u64, Vec<usize>>,
    //system_txs: HashMap<Find, Box<Sender<T>>>, //Transmitters to system threads. WIP
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            eid_manager: EidManager::new(),
            component_manager: ComponentManager::new(),
            entity_ownership: HashMap::new(),
            //system_txs: HashMap::new(),
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

    //WIP
    /*pub fn start_systems(&mut self) {

    }

    fn start_system(&mut self, system: &impl System) -> Box<Send> {
        Box::new(system.start())
    }

    pub fn stop_systems(&mut self) {

    }

    fn stop_system(&mut self, system: &impl System) {

    }*/

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

    //Should probably be done only through a system.
    /*pub fn update_component(&mut self, entity: &Entity, user: &str, which: Find, component: Component) -> Result<(), ErrEcs> {
        self.authenticate(entity, user)?;
        self.component_manager.update_component(entity, which, component)
    }*/

    pub fn remove_component(&mut self, entity: &mut Entity, user: &str, which: Find) -> Result<(), ErrEcs> {
        self.authenticate(entity, user)?;
        self.component_manager.remove_component(entity, which)
    }
}

//Potential errors that can occur within the ECS.
#[derive(Debug)]
pub enum ErrEcs {
    //Ecs Errors
    UserNotFound(String),
    UserDoesNotOwn(String),

    //Component Manager Errors
    UnallocatedComponent(String),
    EidOutOfBounds(String),
    ComponentAlreadyExists(String),
    EntityComponentNotFound(String),
    ComponentCategoryNotFound(String),
}
