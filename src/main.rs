mod ecs;
use crate::ecs::*;

fn main() {
    let mut ecs = ECS::new();
    let eid = ecs.create_for("Alias");

    let pos = Component::Position(Vector2{x: 0, y: 0});
    match ecs.set_component(&eid, "hurp", Find::Position, pos) {
        Ok(_) => (),
        Err(msg) => println!("{:#?}", msg)
    }

    let pos = Component::Position(Vector2{x: 0, y: 0});
    match ecs.set_component(&eid, "Alias", Find::Position, pos) {
        Ok(_) => (),
        Err(msg) => println!("{:#?}", msg)
    }

    match ecs.get_component_mut(&eid, "Alias", Find::Position) {
        Ok(comp) => println!("{:#?}", comp),
        Err(msg) => println!("{:#?}", msg)
    }
}
