mod ecs;
use crate::ecs::*;

/*
* TODO:
*    Implement the first system to move positions.
*    Each system should have it's own thread.
*    New systems should be easy to add.
*/

fn main() -> Result<(), ErrEcs>{
    let mut ecs = ECS::new();
    let user = "Alias";
    let mut entity = ecs.create_for(user);

    //Adding components
    ecs.add_component(&mut entity, &user, Find::Position, Component::Position(Vector2{x: 0, y: 1}))?;

    //Retrieving components
    let pos = ecs.get_component(&entity, &user, Find::Position)?;
    println!("{:#?}", pos);

    //Error propagated
    ecs.get_component(&mut entity, "Bacon", Find::Position)?;

    Ok(())
}
