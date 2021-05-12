use std::sync::{Arc, RwLock};

pub mod messages;
pub use messages::SystemMessage;
pub mod position_system;
pub use position_system::PositionSystem;
use crate::ecs::component_manager::ComponentManager;
use crate::ecs::ComponentData;

pub trait HandleSystemMessage {
    fn handle_message(component_data: Arc<RwLock<ComponentData>>, msg: SystemMessage); //returns the transmitter to the system thread.
}
