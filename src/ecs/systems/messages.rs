use crate::ecs::c_data::*;
use crate::ecs::eid_manager::Entity;

//Message data that is passed into a system.
#[derive(Debug)]
pub enum SystemMessage { //Position Message
    //Stops the system.
    Stop(),

    //Replaces the current Vector2 with the given Vector2
    Pos2Set(Entity, Vector2),
    //Vector mathematical operations.
    Pos2Add(Entity, Vector2),
    Pos2Sub(Entity, Vector2),
    Pos2Mul(Entity, Vector2),
    Pos2Div(Entity, Vector2),
    //Vector increments/decrements
    Pos2IncX(Entity),
    Pos2IncY(Entity),
    Pos2DecX(Entity),
    Pos2DecY(Entity),
    Pos2Inc(Entity),
    Pos2Dec(Entity),
}
