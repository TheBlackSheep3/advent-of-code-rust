use crate::util::position::Position;
use super::position_diff::PositionDifference;
use super::signed_diff::SignedDiff;
use super::size::Size;


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

    pub fn is_within_size(&self, size: &Size) -> bool {
        self.x < size.width && self.y < size.height
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
    use super::*;

    #[test]
    fn position_diff() {
        assert_eq!(
            Position { x: 1, y: 2 }.get_diff(&Position { x: 0, y: 1 }),
            Some(PositionDifference {
                x_diff: -1,
                y_diff: -1
            })
        );
        assert_eq!(
            Position { x: 12, y: 3 }.add_diff(
                &Position { x: 12, y: 3 }
                    .get_diff(&Position { x: 38, y: 99 })
                    .unwrap()
            ),
            Some(Position { x: 38, y: 99 })
        );
        assert_eq!(
            Position { x: 890, y: 11 }.sub_diff(
                &Position { x: 88, y: 10 }
                    .get_diff(&Position { x: 890, y: 11 })
                    .unwrap()
            ),
            Some(Position { x: 88, y: 10 })
        );
    }

    #[test]
    fn bounds_check() {
        let sample_size = Size {
            width: 10,
            height: 10,
        };
        assert_eq!(Position { x: 3, y: 8 }.is_within_size(&sample_size), true);
        assert_eq!(Position { x: 12, y: 8 }.is_within_size(&sample_size), false);
        assert_eq!(Position { x: 3, y: 20 }.is_within_size(&sample_size), false);
        assert_eq!(
            Position { x: 10, y: 10 }.is_within_size(&sample_size),
            false
        );
    }
}
