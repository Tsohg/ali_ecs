mod components;
pub use self::components::Component; //re-export to use in mod.rs

//Declare new components here for src/component_manager/.
pub mod vector2;

#[derive(Debug)]
pub enum ErrCm {
    FailedToAdd(String),
    EntityHasComponent(String),
    EntityNoComponents(String),
    EntityComponentNotFound(String),
}

pub struct ComponentManager { //Maybe have EidManager here too?
    //Eid->Vec<Component>
    components: Vec<Vec<Component>>
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            components: vec![vec![]]
        }
    }

    //Compares 2 enum variants to see if they are equal. The value associated with it does not matter; Source: https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
    pub fn component_variant_eq(comp1: &Component, comp2: &Component) -> bool {
        std::mem::discriminant(comp1) == std::mem::discriminant(comp2)
    }

    pub fn get_component(&mut self, eid: &usize, component: &Component) -> Result<&mut Component, ErrCm> {
        match self.components.get_mut(eid.clone()) {
            Some(entity_components) => {
                match entity_components.iter_mut().find(|c| ComponentManager::component_variant_eq(component, c)) {
                    Some(found) => Ok(found),
                    None => Err(ErrCm::EntityComponentNotFound(format!("eid: {}, component: {:?}", &eid, component)))
                }
            },
            None => Err(ErrCm::EntityNoComponents(format!("eid: {}, component: {:?}", &eid, component)))
        }
    }

    //Add a component to an entity's components if it doesn't already have it.
    pub fn add_component(&mut self, eid: &usize, component: Component) -> Result<(), ErrCm> {
        match self.get_component(&eid, &component) {
            Ok(cmp) => Err(ErrCm::EntityHasComponent(format!("eid: {}", &eid))),
            Err(e) => match e {
                ErrCm::EntityComponentNotFound(_) => {
                    match self.components.get_mut(eid.clone()) {
                        Some(vec) => {
                            vec.push(component);
                            Ok(())
                        },
                        None => Err(ErrCm::FailedToAdd(format!("{:?}", component)))
                    }
                }
                _ => Err(ErrCm::FailedToAdd(format!("{:?}", component)))
            }
        }
    }
}

/*
match self.components.get_mut(eid.clone()) {
    Some(vec) => {
        vec.push(component);
        Ok(())
    },
    None => Err(ErrCm::FailedToAdd(format!("{:?}", component)))
}*/
