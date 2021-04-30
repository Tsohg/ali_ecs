mod ecs;
use crate::ecs::*;

/*
* TODO:
*    Implement the first system to move Positions.
*    Each system should have it's own thread.
*    New systems should be easy to add.
*/

fn main() -> Result<(), ErrEcs> {
    let mut ecs = ECS::new();
    let user = "Alias";
    let mut entity = ecs.create_for(user);

    //Adding components
    ecs.add_component(&mut entity, &user, Find::Pos2, Component::Pos2(Vector2{x: 0, y: 1}))?;

    //Retrieving components
    let pos = ecs.get_component(&entity, &user, Find::Pos2)?;
    println!("{:#?}", pos);

    //Error propagated to main which is fine. You could also handle the possible error in a match.
    ecs.get_component(&mut entity, "Bacon", Find::Pos2)?;

    Ok(())
}
