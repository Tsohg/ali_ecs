mod ecs;
use crate::ecs::*;

use std::{thread, time};

fn main() -> Result<(), ErrEcs> {
    let mut ecs = ECS::new();
    let user = "Alias";
    let mut entity = ecs.create_for(user);

    //Adding components
    ecs.add_component(&mut entity, Find::Pos2, Component::Pos2(Vector2{x: 0, y: 1}))?;

    //Reading cloned components.
    let pos = ecs.read_component(&entity, Find::Pos2)?;
    println!("{:#?}", pos);

    //Sending a message to the position system to set the vector to something else.
    ecs.system_send(&entity, System::Position,
         SystemMessage::Pos2Set(entity.clone(), Vector2{x: 300, y: 300}) //cloning entities is fine because it does not clone the data.
     )?;

     //Since system_send works on a separate thread, this tends to finish before system_send edits the field. So we sleep here a bit.
     thread::sleep(time::Duration::from_secs(1));

     //Testing system_send's position set.
     let pos = ecs.read_component(&entity, Find::Pos2)?;
     println!("{:#?}", pos);

     //Removing components
     ecs.remove_component(&mut entity, Find::Pos2)?;

     Ok(())
}
