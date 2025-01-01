use super::position_diff::PositionDifference;
use super::signed_diff::SignedDiff;
use crate::util::position::Position;

impl Position {
    pub fn get_diff(&self, other: &Self) -> Option<PositionDifference> {
        let x_diff = other.x.signed_diff(self.x)?;
        let y_diff = other.y.signed_diff(self.y)?;
        Some(PositionDifference { x_diff, y_diff })
    }

    pub fn add_diff(&self, diff: &PositionDifference) -> Option<Self> {
        let x = self.x.checked_add_signed(diff.x_diff)?;
        let y = self.y.checked_add_signed(diff.y_diff)?;
        Some(Position { x, y })
    }

    pub fn sub_diff(&self, diff: &PositionDifference) -> Option<Self> {
        let x = self.x.checked_add_signed(diff.x_diff.checked_neg()?)?;
        let y = self.y.checked_add_signed(diff.y_diff.checked_neg()?)?;
        Some(Position { x, y })
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => self.y.cmp(&other.y),
            o => o,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Position { x: 1, y: 2 }, Position { x: 0, y: 1 }, Some(PositionDifference { x_diff: -1, y_diff: -1 }))]
    #[case(Position { x: 0, y: 1 }, Position { x: 1, y: 2 }, Some(PositionDifference { x_diff: 1, y_diff: 1 }))]
    #[case(Position { x: 1, y: 2 }, Position { x: 1, y: 2 }, Some(PositionDifference { x_diff: 0, y_diff: 0 }))]
    #[case(Position { x: usize::MAX, y: 2 }, Position { x: 1, y: 2 }, None)]
    #[case(Position { x: 1, y: 2 }, Position { x: usize::MAX, y: 2 }, None)]
    #[case(Position { x: usize::MAX, y: 1 }, Position { x: usize::MAX, y: 2 }, Some(PositionDifference { x_diff: 0, y_diff: 1 }))]
    fn position_diff(
        #[case] position1: Position,
        #[case] position2: Position,
        #[case] expected: Option<PositionDifference>,
    ) {
        assert_eq!(expected, position1.get_diff(&position2))
    }

    #[rstest]
    #[case(Position { x: 0, y: 0 }, PositionDifference { x_diff: 9, y_diff: 2 }, Some(Position { x: 9, y: 2}))]
    #[case(Position { x: 9, y: 2 }, PositionDifference { x_diff: 0, y_diff: 0 }, Some(Position { x: 9, y: 2}))]
    #[case(Position { x: 9, y: 2 }, PositionDifference { x_diff: -10, y_diff: 0 }, None)]
    #[case(Position { x: 9, y: usize::MAX }, PositionDifference { x_diff: 0, y_diff: 9 }, None)]
    #[case(Position { x: 12, y: 3 }, Position { x: 12, y: 3 }.get_diff(&Position { x: 38, y: 99 }).unwrap(), Some(Position { x: 38, y: 99 }))]
    fn add_position_diff(
        #[case] position: Position,
        #[case] position_diff: PositionDifference,
        #[case] expected: Option<Position>,
    ) {
        assert_eq!(expected, position.add_diff(&position_diff))
    }

    #[rstest]
    #[case(Position { x: 0, y: 0 }, PositionDifference { x_diff: 9, y_diff: 2 }, None)]
    #[case(Position { x: 0, y: 0 }, PositionDifference { x_diff: -9, y_diff: -2 }, Some(Position { x: 9, y: 2}))]
    #[case(Position { x: 10, y: 3 }, PositionDifference { x_diff: 2, y_diff: -2 }, Some(Position { x: 8, y: 5}))]
    #[case(Position { x: 10, y: 3 }, PositionDifference { x_diff: -2, y_diff: 2 }, Some(Position { x: 12, y: 1}))]
    #[case(Position { x: 10, y: usize::MAX }, PositionDifference { x_diff: 2, y_diff: -2 }, None)]
    #[case(Position { x: 890, y: 11 }, Position { x: 88, y: 10 }.get_diff(&Position { x: 890, y: 11 }).unwrap(), Some(Position { x: 88, y: 10 }))]
    fn sub_position_diff(
        #[case] position: Position,
        #[case] position_diff: PositionDifference,
        #[case] expected: Option<Position>,
    ) {
        assert_eq!(expected, position.sub_diff(&position_diff))
    }
}
