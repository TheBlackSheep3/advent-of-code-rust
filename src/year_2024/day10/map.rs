use std::collections::HashSet;
use std::ops::Index;
use std::str::FromStr;

use crate::util::matrix::Matrix;
use crate::util::position::Position;

#[derive(Debug, PartialEq)]
pub(super) struct Map {
    matrix: Matrix<u32>,
    trail_heads: Vec<Position>,
}

impl FromStr for Map {
    type Err = super::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let rows = lines.clone().count();
        let columns = lines
            .clone()
            .next()
            .map(|line| line.len())
            .ok_or(super::Error::ParsingFailed)?;
        let matrix = Matrix::from_vec(
            lines
                .into_iter()
                .flat_map(|line| {
                    line.chars().map(|char| {
                        char.to_string()
                            .parse::<u32>()
                            .map_err(|_| super::Error::ParsingFailed)
                    })
                })
                .into_iter()
                .collect::<Result<Vec<u32>, super::Error>>()?,
            rows,
            columns,
        )
        .map_err(|_| super::Error::ParsingFailed)?;
        let mut trail_heads = Vec::new();
        for (y, row) in matrix.rows().enumerate() {
            for (x, height) in row.iter().enumerate() {
                if height == &0u32 {
                    trail_heads.push(Position { x, y });
                }
            }
        }
        Ok(Map {
            matrix,
            trail_heads,
        })
    }
}

impl<T> Index<&Position> for Matrix<T> {
    type Output = T;
    fn index(&self, index: &Position) -> &Self::Output {
        &self[(index.y, index.x)]
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        &self[(index.y, index.x)]
    }
}

impl Map {
    pub fn get_trail_heads<'m>(&'m self) -> Vec<&'m Position> {
        self.trail_heads.iter().collect()
    }

    pub fn get_trail_ends(&self, position: &Position) -> HashSet<Position> {
        let mut iter = TrailIterator::new(self, position);
        match iter.nth(8) {
            Some(x) => x,
            None => HashSet::new(),
        }
    }
}

struct TrailIterator<'m> {
    matrix: &'m Matrix<u32>,
    current_positions: HashSet<Position>,
}

impl<'m> Iterator for TrailIterator<'m> {
    type Item = HashSet<Position>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next_positions: HashSet<Position> = HashSet::new();
        for pos in self.current_positions.iter() {
            let current_value = self.matrix[pos];
            for new_pos in pos.sourounding_positions().into_iter().filter(|x| {
                x.is_within_size(&self.matrix.size()) && current_value + 1 == self.matrix[x]
            }) {
                next_positions.insert(new_pos);
            }
        }
        self.current_positions = next_positions;
        if self.current_positions.len() != 0 {
            Some(self.current_positions.clone())
        } else {
            None
        }
    }
}

impl<'m> TrailIterator<'m> {
    pub fn new(map: &'m Map, trail_head: &'m Position) -> Self {
        if map.matrix[trail_head] != 0 {
            panic!("{trail_head:?} is no trail head")
        }
        Self {
            matrix: &map.matrix,
            current_positions: HashSet::from([trail_head.clone()]),
        }
    }
}

impl Position {
    fn sourounding_positions(&self) -> Vec<Self> {
        let mut positions = Vec::new();
        if let Some(new_x) = self.x.checked_sub(1) {
            positions.push(Self { x: new_x, ..*self });
        }
        if let Some(new_x) = self.x.checked_add(1) {
            positions.push(Self { x: new_x, ..*self });
        }
        if let Some(new_y) = self.y.checked_sub(1) {
            positions.push(Self { y: new_y, ..*self });
        }
        if let Some(new_y) = self.y.checked_add(1) {
            positions.push(Self { y: new_y, ..*self });
        }
        positions
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::collections::HashSet;

    use super::super::tests::TEST_STR;
    use super::super::Error;
    use super::Map;
    use super::Matrix;
    use super::Position;
    use super::TrailIterator;

    #[rstest]
    #[case(TEST_STR, Ok(Map{matrix: Matrix::<u32>::from_vec(vec![8,9,0,1,0,1,2,3,7,8,1,2,1,8,7,4, 8,7,4,3,0,9,6,5, 9,6,5,4,9,8,7,4, 4,5,6,7,8,9,0,3, 3,2,0,1,9,0,1,2, 0,1,3,2,9,8,0,1, 1,0,4,5,6,7,3,2, ], 8, 8).unwrap(), trail_heads: vec![ Position{x:2,y:0}, Position{x:4,y:0}, Position{x:4,y:2}, Position{x:6,y:4}, Position{x:2,y:5}, Position{x:5,y:5}, Position{x:0,y:6}, Position{x:6,y:6}, Position{x:1,y:7}, ]}))]
    #[case("0123456789", Ok(Map{matrix: Matrix::<u32>::from_vec(vec![0,1,2,3,4,5,6,7,8,9], 1, 10).unwrap(), trail_heads: vec![Position{x: 0, y: 0}]}))]
    #[case("01234\n56780", Ok(Map{matrix: Matrix::<u32>::from_vec(vec![0,1,2,3,4,5,6,7,8,0], 2, 5).unwrap(), trail_heads: vec![Position{x: 0, y: 0},Position{x: 4, y: 1}]}))]
    #[case("81234\n56781", Ok(Map{matrix: Matrix::<u32>::from_vec(vec![8,1,2,3,4,5,6,7,8,1], 2, 5).unwrap(), trail_heads: vec![]}))]
    #[case("01234\n5678", Err(Error::ParsingFailed))]
    #[case("01234\n56a89", Err(Error::ParsingFailed))]
    fn parse(#[case] input: &str, #[case] expected: Result<Map, Error>) {
        assert_eq!(expected, input.parse())
    }

    #[rstest]
    #[case(Map{matrix: Matrix::<u32>::from_vec(vec![8,9,0,1,0,1,2,3,7,8,1,2,1,8,7,4, 8,7,4,3,0,9,6,5, 9,6,5,4,9,8,7,4, 4,5,6,7,8,9,0,3, 3,2,0,1,9,0,1,2, 0,1,3,2,9,8,0,1, 1,0,4,5,6,7,3,2, ], 8, 8).unwrap(), trail_heads: vec![ Position{x:2,y:0}, Position{x:4,y:0}, Position{x:4,y:2}, Position{x:6,y:4}, Position{x:2,y:5}, Position{x:5,y:5}, Position{x:0,y:6}, Position{x:6,y:6}, Position{x:1,y:7}, ]},
        vec![ &Position{x:2,y:0}, &Position{x:4,y:0}, &Position{x:4,y:2}, &Position{x:6,y:4}, &Position{x:2,y:5}, &Position{x:5,y:5}, &Position{x:0,y:6}, &Position{x:6,y:6}, &Position{x:1,y:7}, ])]
    #[case(Map{matrix: Matrix::<u32>::from_vec(vec![0,1,2,3,4,5,6,7,8,9], 1, 10).unwrap(), trail_heads: vec![Position{x: 0, y: 0}]}, vec![&Position{x:0, y:0}])]
    #[case(Map{matrix: Matrix::<u32>::from_vec(vec![0,1,2,3,4,5,6,7,8,0], 2, 5).unwrap(), trail_heads: vec![Position{x: 0, y: 0},Position{x: 4, y: 1}]}, vec![&Position{x:0,y:0}, &Position{x:4, y:1}])]
    #[case(Map{matrix: Matrix::<u32>::from_vec(vec![8,1,2,3,4,5,6,7,8,1], 2, 5).unwrap(), trail_heads: vec![]}, vec![])]
    fn get_trail_head(#[case] input: Map, #[case] expected: Vec<&Position>) {
        assert_eq!(expected, input.get_trail_heads())
    }

    #[rstest]
    #[case("01234\n98765".parse::<Map>().unwrap(), Position { x: 0, y: 0 }, HashSet::from([Position{ x: 0, y: 1 }]))]
    #[case(TEST_STR.parse::<Map>().unwrap(), Position { x: 2, y: 0 }, HashSet::from([ Position { x: 0, y: 3 }, Position { x: 1, y: 0 }, Position { x: 4, y: 3 }, Position { x: 5, y: 4 }, Position { x: 4, y: 5 }, ]))]
    fn get_trail_ends(
        #[case] input: Map,
        #[case] head: Position,
        #[case] expected: HashSet<Position>,
    ) {
        assert_eq!(expected, input.get_trail_ends(&head))
    }

    #[rstest]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 0, y: 0}, 0)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 1, y: 0}, 1)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 2, y: 0}, 2)]
    #[should_panic(expected = "out of bounds")]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 3, y: 0}, 2)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 0, y: 1}, 3)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 1, y: 1}, 4)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 2, y: 1}, 5)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 0, y: 2}, 6)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 1, y: 2}, 7)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 2, y: 2}, 8)]
    #[should_panic(expected = "out of bounds")]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), &Position{ x: 0, y: 3}, 2)]
    fn matrix_position_borrow_index(
        #[case] m: Matrix<u32>,
        #[case] index: &Position,
        #[case] expexted: u32,
    ) {
        assert_eq!(expexted, m[index])
    }

    #[rstest]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 0, y: 0}, 0)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 1, y: 0}, 1)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 2, y: 0}, 2)]
    #[should_panic(expected = "out of bounds")]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 3, y: 0}, 2)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 0, y: 1}, 3)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 1, y: 1}, 4)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 2, y: 1}, 5)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 0, y: 2}, 6)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 1, y: 2}, 7)]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 2, y: 2}, 8)]
    #[should_panic(expected = "out of bounds")]
    #[case(Matrix::<u32>::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3).unwrap(), Position{ x: 0, y: 3}, 2)]
    fn matrix_position_index(
        #[case] m: Matrix<u32>,
        #[case] index: Position,
        #[case] expexted: u32,
    ) {
        assert_eq!(expexted, m[index])
    }

    #[rstest]
    #[case(Position{ x: 1, y: 1 }, vec![Position{ x: 0, y: 1 }, Position{ x: 2, y: 1 }, Position{ x: 1, y: 0 }, Position{ x: 1, y: 2 }])]
    #[case(Position{ x: 0, y: 1 }, vec![Position{ x: 1, y: 1 }, Position{ x: 0, y: 0 }, Position{ x: 0, y: 2 }])]
    #[case(Position{ x: 1, y: 0 }, vec![Position{ x: 0, y: 0 }, Position{ x: 2, y: 0 }, Position{ x: 1, y: 1 }])]
    #[case(Position{ x: 0, y: 0 }, vec![Position{ x: 1, y: 0 }, Position{ x: 0, y: 1 }])]
    #[case(Position{ x: usize::MAX, y: usize::MAX }, vec![Position{ x: usize::MAX - 1, y: usize::MAX }, Position{ x: usize::MAX, y: usize::MAX - 1 }])]
    fn sourrounding_positions(#[case] input: Position, #[case] expected: Vec<Position>) {
        assert_eq!(expected, input.sourounding_positions())
    }

    /*
     * Position{x:4,y:2},
     * Position{x:6,y:4},
     * Position{x:2,y:5},
     * Position{x:5,y:5},
     * Position{x:0,y:6},
     * Position{x:6,y:6},
     * Position{x:1,y:7},
     */
    #[rstest]
    #[should_panic(expected = "no trail head")]
    #[case(TEST_STR.parse::<Map>().unwrap(), Position { x: 0, y: 0 }, vec![])]
    #[case(TEST_STR.parse::<Map>().unwrap(), Position { x: 2, y: 0 },
        vec![
            HashSet::from([ Position { x: 3, y: 0 }, Position { x: 2, y: 1 }, ]),
            HashSet::from([ Position { x: 3, y: 1 }, ]),
            HashSet::from([ Position { x: 3, y: 2 }, ]),
            HashSet::from([ Position { x: 2, y: 2 }, Position { x: 3, y: 3 }, ]),
            HashSet::from([ Position { x: 2, y: 3 }, ]),
            HashSet::from([ Position { x: 1, y: 3 }, Position { x: 2, y: 4 }, ]),
            HashSet::from([ Position { x: 1, y: 2 }, Position { x: 3, y: 4 }, ]),
            HashSet::from([ Position { x: 0, y: 2 }, Position { x: 1, y: 1 }, Position { x: 4, y: 4 }, ]),
            HashSet::from([ Position { x: 0, y: 3 }, Position { x: 1, y: 0 }, Position { x: 4, y: 3 }, Position { x: 5, y: 4 }, Position { x: 4, y: 5 }, ]),
        ]
    )]
    #[case(TEST_STR.parse::<Map>().unwrap(), Position{ x: 4, y: 0 },
        vec![
            HashSet::from([ Position { x: 3, y: 0 }, Position { x: 5, y: 0 }, Position { x: 4, y: 1 }, ]),
            HashSet::from([ Position { x: 3, y: 1 }, Position { x: 6, y: 0 }, ]),
            HashSet::from([ Position { x: 3, y: 2 }, Position { x: 7, y: 0 }, ]),
            HashSet::from([ Position { x: 2, y: 2 }, Position { x: 3, y: 3 }, Position { x: 7, y: 1 }, ]),
            HashSet::from([ Position { x: 2, y: 3 }, Position { x: 7, y: 2 }, ]),
            HashSet::from([ Position { x: 1, y: 3 }, Position { x: 2, y: 4 }, Position { x: 6, y: 2 }, ]),
            HashSet::from([ Position { x: 1, y: 2 }, Position { x: 3, y: 4 }, Position { x: 6, y: 1 }, Position { x: 6, y: 3 }, ]),
            HashSet::from([ Position { x: 0, y: 2 }, Position { x: 1, y: 1 }, Position { x: 4, y: 4 }, Position { x: 5, y: 1 }, Position { x: 5, y: 3 }, ]),
            HashSet::from([ Position { x: 0, y: 3 }, Position { x: 1, y: 0 }, Position { x: 4, y: 3 }, Position { x: 5, y: 4 }, Position { x: 4, y: 5 }, Position { x: 5, y: 2 }, ]),
        ]
    )]
    #[case("01234\n98765".parse::<Map>().unwrap(), Position { x: 0, y: 0 },
        vec![
            HashSet::from([ Position { x: 1, y: 0 } ]),
            HashSet::from([ Position { x: 2, y: 0 } ]),
            HashSet::from([ Position { x: 3, y: 0 } ]),
            HashSet::from([ Position { x: 4, y: 0 } ]),
            HashSet::from([ Position { x: 4, y: 1 } ]),
            HashSet::from([ Position { x: 3, y: 1 } ]),
            HashSet::from([ Position { x: 2, y: 1 } ]),
            HashSet::from([ Position { x: 1, y: 1 } ]),
            HashSet::from([ Position { x: 0, y: 1 } ]),
        ]
    )]
    fn trail_iterator(
        #[case] map: Map,
        #[case] start: Position,
        #[case] iterator_values: Vec<HashSet<Position>>,
    ) {
        let iter = TrailIterator::new(&map, &start);
        for (expected, actual) in iterator_values.into_iter().zip(iter) {
            assert_eq!(expected, actual)
        }
    }
}
