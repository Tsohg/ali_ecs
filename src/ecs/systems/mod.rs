use std::sync::{Arc, RwLock};

pub mod messages;
pub use messages::SystemMessage;
pub mod position_system;
pub use position_system::PositionSystem;
use crate::ecs::component_manager::ComponentManager;
use crate::ecs::Component;

pub trait HandleSystemMessage {
    fn handle_message(cm: Arc<RwLock<ComponentManager>>, msg: SystemMessage); //returns the transmitter to the system thread.
}
