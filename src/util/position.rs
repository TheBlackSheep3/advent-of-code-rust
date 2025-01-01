use super::size::Size;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn is_within_size(&self, size: &Size) -> bool {
        self.x < size.width && self.y < size.height
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Position;
    use super::Size;

    #[rstest]
    #[case(Position { x: 3, y: 8 }, Size { width: 10, height: 10 }, true)]
    #[case(Position { x: 12, y: 8 }, Size { width: 10, height: 10 }, false)]
    #[case(Position { x: 3, y: 20 }, Size { width: 10, height: 10 }, false)]
    #[case(Position { x: 10, y: 10 }, Size { width: 10, height: 10 }, false)]
    #[case(Position { x: 0, y: 0 }, Size { width: 0, height: 0 }, false)]
    fn bounds_check(#[case] position: Position, #[case] size: Size, #[case] expected: bool) {
        assert_eq!(expected, position.is_within_size(&size))
    }
}
