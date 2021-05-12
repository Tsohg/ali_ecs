use std::thread;
use std::sync::{Arc, RwLock};

use super::*;
use crate::ecs::{Component, Find};

pub struct PositionSystem {}

impl HandleSystemMessage for PositionSystem {
    fn handle_message(component_data: Arc<RwLock<ComponentData>>, msg: SystemMessage) {
        let cd = component_data.clone();
        thread::spawn(move||{
            match msg {
                SystemMessage::Pos2Set(en, v2) => {
                    if let Err(e) = ComponentManager::set_component(cd, &en, &Find::Pos2, Component::Pos2(v2)) {
                        println!("Error in position system: {:#?}", e);
                    }    
                },
                _ => ()
            }
        });
    }
}
