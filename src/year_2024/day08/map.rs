use super::antenna::Antenna;
use super::antinode::Antinode;
use super::error::Error;
use super::position::Position;
use super::position_diff::PositionDifference;
use super::size::Size;
use std::collections::hash_set::HashSet;

#[derive(Debug, PartialEq)]
pub struct Map {
    size: Size,
    antennas: Vec<Antenna>,
}

impl std::str::FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let size = Size {
            width: lines
                .clone()
                .next()
                .map(|l| l.len())
                .ok_or(Error::ParsingFailed)?,
            height: lines.clone().count(),
        };
        if !lines.clone().all(|l| l.len() == size.width) {
            Err(Error::ParsingFailed)
        } else {
            let mut antennas: Vec<Antenna> = vec![];
            for (y, line) in lines.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c.is_alphanumeric() {
                        antennas.push(Antenna {
                            position: Position { x, y },
                            frequency: c,
                        });
                    }
                }
            }
            Ok(Map { size, antennas })
        }
    }
}

impl Map {
    /// The first iterator continously adds the difference to the second antenna position, while
    /// the second iterator continously substracts the difference from the frist antenna position.
    /// Both starting with the antenna position respectively.
    ///
    /// If the frequencies of the antennas do not match, both iterators do not return any values.
    ///
    /// * `first_antenna`: Reference to the first antenna.
    /// * `second_antenna`: Refernce to the second antenna.
    fn get_antinode_iterators(
        &self,
        first_antenna: &Antenna,
        second_antenna: &Antenna,
    ) -> (AntinodeIterator, AntinodeIterator) {
        let addition = |pos: &Position, diff: &PositionDifference| pos.add_diff(diff);
        let subtraction = |pos: &Position, diff: &PositionDifference| pos.sub_diff(diff);
        match first_antenna
            .position
            .get_diff(&second_antenna.position)
            .filter(|_| first_antenna.frequency == second_antenna.frequency)
        {
            Some(difference) => {
                let addition_iter = AntinodeIterator {
                    current_position: Some(second_antenna.position.clone()),
                    difference: difference.clone(),
                    frequency: first_antenna.frequency,
                    map_size: self.size.clone(),
                    iteration_function: addition,
                };
                let subtraction_iter = AntinodeIterator {
                    current_position: Some(first_antenna.position.clone()),
                    difference,
                    frequency: first_antenna.frequency,
                    map_size: self.size.clone(),
                    iteration_function: subtraction,
                };
                (addition_iter, subtraction_iter)
            }
            None => {
                let it = AntinodeIterator {
                    current_position: None,
                    difference: PositionDifference {
                        x_diff: 0,
                        y_diff: 0,
                    },
                    frequency: 'a',
                    map_size: Size {
                        width: 0,
                        height: 0,
                    },
                    iteration_function: addition,
                };
                (it.clone(), it)
            }
        }
    }

    fn get_antenna_pairs(&self) -> HashSet<(&Antenna, &Antenna)> {
        let frequencies = self
            .antennas
            .iter()
            .map(|a| a.frequency)
            .collect::<HashSet<_>>();
        let mut antenna_pairs: HashSet<(&Antenna, &Antenna)> = HashSet::new();
        for frequency in frequencies {
            let antennas_with_frequency_iter =
                self.antennas.iter().filter(|a| a.frequency == frequency);
            for first_antenna in antennas_with_frequency_iter.clone() {
                for second_antenna in antennas_with_frequency_iter.clone() {
                    if first_antenna != second_antenna
                        && !antenna_pairs.contains(&(first_antenna, second_antenna))
                        && !antenna_pairs.contains(&(&second_antenna, &first_antenna))
                    {
                        antenna_pairs.insert((first_antenna, second_antenna));
                    }
                }
            }
        }
        antenna_pairs
    }

    pub fn get_antinodes_double_distance(&self) -> Vec<Antinode> {
        let mut antinodes: Vec<Antinode> = Vec::new();
        let antenna_pairs = self.get_antenna_pairs();
        for (first_antenna, second_antenna) in antenna_pairs.iter() {
            let (mut add_iter, mut sub_iter) =
                self.get_antinode_iterators(first_antenna, second_antenna);
            if let Some(antinode) = add_iter.nth(1) {
                antinodes.push(antinode);
            }
            if let Some(antinode) = sub_iter.nth(1) {
                antinodes.push(antinode);
            }
        }
        antinodes
    }

    pub fn get_antinodes_resonant_harmonics(&self) -> Vec<Antinode> {
        let mut antinodes: Vec<Antinode> = Vec::new();
        for (first_antenna, second_antenna) in self.get_antenna_pairs() {
            let (add_iter, sub_iter) = self.get_antinode_iterators(first_antenna, second_antenna);
            for antinode in add_iter {
                antinodes.push(antinode)
            }
            for antinode in sub_iter {
                antinodes.push(antinode)
            }
        }
        antinodes
    }
}

#[derive(Clone)]
struct AntinodeIterator {
    current_position: Option<Position>,
    difference: PositionDifference,
    frequency: char,
    map_size: Size,
    iteration_function: fn(&Position, &PositionDifference) -> Option<Position>,
}

impl Iterator for AntinodeIterator {
    type Item = Antinode;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_position {
            Some(position) => {
                self.current_position = (self.iteration_function)(&position, &self.difference)
                    .filter(|p| p.is_within_size(&self.map_size));
                Some(Antinode {
                    position,
                    frequency: self.frequency,
                })
            }
            None => None,
        }
        // match (self.iteration_function)(&self.current_position, &self.difference)
        //     .filter(|p| p.is_within_size(&self.map_size))
        // {
        //     Some(position) => {
        //         self.current_position = position.clone();
        //         Some(Antinode {
        //             position,
        //             frequency: self.frequency,
        //         })
        //     }
        //     None => None,
        // }
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day08::tests::TEST_STR;

    use super::*;

    #[test]
    fn map_parse() {
        assert_eq!(
            super::super::tests::TEST_STR.parse(),
            Ok(Map {
                size: Size {
                    width: 12,
                    height: 12
                },
                antennas: vec![
                    Antenna {
                        position: Position { x: 8, y: 1 },
                        frequency: '0'
                    },
                    Antenna {
                        position: Position { x: 5, y: 2 },
                        frequency: '0'
                    },
                    Antenna {
                        position: Position { x: 7, y: 3 },
                        frequency: '0'
                    },
                    Antenna {
                        position: Position { x: 4, y: 4 },
                        frequency: '0'
                    },
                    Antenna {
                        position: Position { x: 6, y: 5 },
                        frequency: 'A'
                    },
                    Antenna {
                        position: Position { x: 8, y: 8 },
                        frequency: 'A'
                    },
                    Antenna {
                        position: Position { x: 9, y: 9 },
                        frequency: 'A'
                    },
                ]
            })
        );
    }

    #[test]
    fn antinodes() {
        let mut expected_antinodes = vec![
            Antinode {
                position: Position { x: 6, y: 0 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 11, y: 0 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 3, y: 1 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 3, y: 1 },
                frequency: 'A',
            },
            Antinode {
                position: Position { x: 4, y: 2 },
                frequency: 'A',
            },
            Antinode {
                position: Position { x: 10, y: 2 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 2, y: 3 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 9, y: 4 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 1, y: 5 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 6, y: 5 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 3, y: 6 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 0, y: 7 },
                frequency: '0',
            },
            Antinode {
                position: Position { x: 7, y: 7 },
                frequency: 'A',
            },
            Antinode {
                position: Position { x: 10, y: 10 },
                frequency: 'A',
            },
            Antinode {
                position: Position { x: 10, y: 11 },
                frequency: 'A',
            },
        ];
        let mut actual_antinodes = super::super::tests::TEST_STR
            .parse::<Map>()
            .unwrap()
            .get_antinodes_double_distance();
        expected_antinodes.sort();
        actual_antinodes.sort();
        assert_eq!(actual_antinodes, expected_antinodes)
    }

    #[test]
    fn antinode_resonant_harmonics() {
        const TEST_INPUT: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

        let x = vec![
            (
                TEST_INPUT.parse::<Map>().expect("parsing failed"),
                vec![
                    Antinode {
                        position: Position { x: 0, y: 0 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 3, y: 1 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 1, y: 2 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 5, y: 0 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 6, y: 2 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 9, y: 3 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 2, y: 4 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 3, y: 6 },
                        frequency: 'T',
                    },
                    Antinode {
                        position: Position { x: 4, y: 8 },
                        frequency: 'T',
                    },
                ],
            ),
            (
                TEST_STR.parse::<Map>().expect("parsing failed"),
                vec![
                    Antinode {
                        position: Position { x: 8, y: 1 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 5, y: 2 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 7, y: 3 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 4, y: 4 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 6, y: 5 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 8, y: 8 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 9, y: 9 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 0, y: 0 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 0, y: 7 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 1, y: 0 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 1, y: 1 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 1, y: 5 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 1, y: 10 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 2, y: 2 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 2, y: 3 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 2, y: 8 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 3, y: 1 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 3, y: 1 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 3, y: 3 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 3, y: 6 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 3, y: 11 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 4, y: 2 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 4, y: 4 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 4, y: 9 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 5, y: 5 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 5, y: 7 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 6, y: 0 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 6, y: 5 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 6, y: 6 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 7, y: 7 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 9, y: 4 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 10, y: 2 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 10, y: 10 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 10, y: 11 },
                        frequency: 'A',
                    },
                    Antinode {
                        position: Position { x: 11, y: 0 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 11, y: 5 },
                        frequency: '0',
                    },
                    Antinode {
                        position: Position { x: 11, y: 11 },
                        frequency: 'A',
                    },
                ],
            ),
        ];
        for (map, expected_antinodes) in x {
            let actual_antinodes = map.get_antinodes_resonant_harmonics();
            let expected_antinodes = expected_antinodes.into_iter().collect::<HashSet<_>>();
            let actual_antinodes = actual_antinodes.into_iter().collect::<HashSet<_>>();
            assert_eq!(expected_antinodes.len(), actual_antinodes.len());
            assert_eq!(expected_antinodes, actual_antinodes);
        }
    }
}
