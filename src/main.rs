/*mod eid_manager;
mod component_manager;

use eid_manager::EidManager;
use component_manager::ComponentManager;*/

mod ecs;
use crate::ecs::ECS;
use crate::ecs::component_manager::Component;
use crate::ecs::component_manager::vector2::Vector2;

fn main() {
    let mut ecs = ECS::new();
    let eid = ecs.create_eid();
    ecs.add_component(&eid, Component::Position(Some(Vector2{x: 0, y: 1})));

    let pos = ecs.get_component(&eid, Component::Position(None));

    //TODO: Unwrap this using the ECS and just return the value.
    match pos {
        Ok(comp) => match comp {
            Component::Position(op) => match op {
                Some(pos) => println!("x: {}, y: {}", pos.x, pos.y),
                None => ()
            }
        },
        Err(e) => println!("{:?}", e)
    }
}
