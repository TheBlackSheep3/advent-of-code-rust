use std::str::FromStr;

use crate::util::matrix::Matrix;
use crate::util::position::Position;

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
