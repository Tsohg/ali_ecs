mod ecs;
use crate::ecs::*;

/*
* TODO:
*    Implement an input system that fires a function when a certain input is given.
*       Should be mappable via a function: fn(input, fn)
*/

fn main() {
    let mut ecs = ECS::new();
    let mut entity = ecs.create_for("Alias");

    match ecs.add_component(&mut entity, "Alias", Find::Position, Component::Position(Vector2{x: 0, y: 0})) {
        Ok(_) => println!("Added"),
        Err(e) => println!("{:#?}", e)
    }

    match ecs.add_component(&mut entity, "Alias", Find::Position, Component::Position(Vector2{x: 0, y: 1})) {
        Ok(_) => println!("Added"),
        Err(e) => println!("{:#?}", e)
    }

    match ecs.update_component(&mut entity, "Alias", Find::Position, Component::Position(Vector2{x: 100, y: 100})) {
        Ok(_) => println!("Updated."),
        Err(e) => println!("{:#?}", e)
    }

    match ecs.remove_component(&mut entity, "Alias", Find::Position) {
        Ok(_) => println!("Removed."),
        Err(e) => println!("{:#?}", e)
    }

    match ecs.get_component_mut(&entity, "Alias", Find::Position) {
        Ok(comp) => println!("{:#?}", comp),
        Err(e) => println!("{:#?}", e)
    }
}
