use super::antenna::Antenna;
use super::antinode::Antinode;
use super::error::Error;
use super::position::Position;
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
    pub fn get_antinodes(&self) -> Vec<Antinode> {
        let mut antinodes: Vec<Antinode> = Vec::new();
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
        for (first_antenna, second_antenna) in antenna_pairs.iter() {
            let diff = first_antenna.position.get_diff(&second_antenna.position);
            if first_antenna.frequency == second_antenna.frequency {
                if let Some(diff) = diff {
                    let possible_antinode_position = first_antenna
                        .position
                        .sub_diff(&diff)
                        .filter(|p| p.is_within_size(&self.size));
                    if let Some(possible_antinode_position) = possible_antinode_position {
                        antinodes.push(Antinode {
                            position: possible_antinode_position,
                            frequency: first_antenna.frequency,
                        });
                    }
                    let possible_antinode_position = second_antenna
                        .position
                        .add_diff(&diff)
                        .filter(|p| p.is_within_size(&self.size));
                    if let Some(possible_antinode_position) = possible_antinode_position {
                        antinodes.push(Antinode {
                            position: possible_antinode_position,
                            frequency: first_antenna.frequency,
                        });
                    }
                }
            }
        }
        antinodes
    }
}

#[cfg(test)]
mod tests {
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
            .get_antinodes();
        expected_antinodes.sort();
        actual_antinodes.sort();
        assert_eq!(actual_antinodes, expected_antinodes)
    }
}
