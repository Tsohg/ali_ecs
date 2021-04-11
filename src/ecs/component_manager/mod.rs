//New component data structures should be defined and then pub use in this directory.

//Component data structures. Add to this list for new components.
pub mod vector2;
pub use super::vector2::Vector2;

#[derive(Debug)]
pub enum ErrCm {
    FailedToAdd(String),
    EntityNoComponents(String),
    EntityComponentNotFound(String),
}

//Define components as an Option of their data structure.
struct Components {
    position: Vec<Option<Vector2>>,
}

pub struct ComponentManager {
    component_size: usize,
    components : Components,
}

//Define getter/setter for each component.
impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_size: 0,
            components: Components {
                position: vec![],
            }
        }
    }

    pub fn set_position(&mut self, entity_id: &usize, position: Option<Vector2>) {
        if entity_id >= &self.component_size {
            self.components.position.push(position);
            self.component_size += 1;
        } else {
            self.components.position[*entity_id] = position;
        }
    }

    pub fn get_position(&self, entity_id: &usize) -> &Option<Vector2> {
        &self.components.position[*entity_id]
    }
}
