use super::position::Position;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Antinode {
    pub position: Position,
    pub frequency: char,
}

impl Ord for Antinode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.position.cmp(&other.position) {
            std::cmp::Ordering::Equal => self.frequency.cmp(&other.frequency),
            o => o,
        }
    }
}

impl PartialOrd for Antinode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
