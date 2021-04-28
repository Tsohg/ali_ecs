mod ecs;
use crate::ecs::*;

/*
* TODO:
*    Implement an input system that fires a function when a certain input is given.
*       Should be mappable via a function: fn(input, fn)
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
