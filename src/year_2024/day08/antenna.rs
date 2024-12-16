use super::position::Position;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Antenna {
    pub position: Position,
    pub frequency: char,
}
