use std::thread;
use std::sync::{Arc, RwLock};

use super::*;
use crate::ecs::c_data::*;
use crate::ecs::eid_manager::Entity;
use crate::ecs::{Component, Find};

pub struct PositionSystem {}

/*impl PositionSystem {
    fn set_pos2(cm: &mut ComponentManager, entity: Entity, v2: Vector2) {

    }
}*/

impl HandleSystemMessage for PositionSystem {
    fn handle_message(cm: Arc<RwLock<ComponentManager>>, msg: SystemMessage) {
        thread::spawn(move||{
            match msg {
                SystemMessage::Pos2Set(en, v2) => {
                    cm.write().unwrap().set_component(&en, &Find::Pos2, Component::Pos2(v2));
                    ()
                },
                _ => ()
            }
        });
    }
}
