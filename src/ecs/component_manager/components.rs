//Use component's data structures here.
use super::vector2::Vector2;

//Add components and their related data structure here. Make sure the datastructure is wrapped in Option<T> as we use the None option for enum searching.
#[derive(Debug)]
pub enum Component {
    Position(Option<Vector2>),
}
