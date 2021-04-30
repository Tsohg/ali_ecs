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
    ecs.start_systems();

    let user = "Alias";
    let mut entity = ecs.create_for(user);

    //Adding components
    ecs.add_component(&mut entity, &user, Find::Pos2, Component::Pos2(Vector2{x: 0, y: 1}))?;

    //Sending a message to the position system to set the vector to something else.
    ecs.system_send(&entity, &user, System::Position,
         SystemMessage::Pos2Set(entity.clone(), Vector2{x: 300, y: 300}) //cloning entities is fine because it does not clone the data.
     )?;

    //Retrieving components
    let pos = ecs.get_component(&entity, &user, Find::Pos2)?;
    println!("{:#?}", pos);

    //Error propagated to main which is fine. You could also handle the possible error in a match.
    //ecs.get_component(&mut entity, "Bacon", Find::Pos2)?;

    loop { }
}
