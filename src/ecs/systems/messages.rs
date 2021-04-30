use crate::ecs::c_data::*;

//System messages for the Pos2 component. First parameter is the entity id as usize.
#[derive(Debug)]
pub enum Pos2Msg {
    //Stops the system. All system messages should contain this variant.
    Stop(),

    //Replaces the current Vector2 with the given Vector2
    Set(usize, Vector2),

    //Vector mathematical operations.
    Add(usize, Vector2),
    Sub(usize, Vector2),
    Mul(usize, Vector2),
    Div(usize, Vector2),

    //Vector increments/decrements
    IncX(usize),
    IncY(usize),
    DecX(usize),
    DecY(usize),
}
