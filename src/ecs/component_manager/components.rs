//Use positions here.
use super::vector2::Vector2;

#[derive(Debug)]
pub enum Component {
    Position(Option<Vector2>),
}
