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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::super::tests::TEST_STR;
    use super::super::Error;
    use super::Map;
    use super::Matrix;
    use super::Position;

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
}
