use core::panic;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Error {
    MapParsingFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MapParsingFailed => write!(f, "failed to parse map"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GuardOrientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Guard {
    orientation: GuardOrientation,
    position: (usize, usize),
}

impl Guard {
    fn rotate(&mut self) {
        match self.orientation {
            GuardOrientation::Up => self.orientation = GuardOrientation::Right,
            GuardOrientation::Right => self.orientation = GuardOrientation::Down,
            GuardOrientation::Down => self.orientation = GuardOrientation::Left,
            GuardOrientation::Left => self.orientation = GuardOrientation::Up,
        }
    }

    fn step(&mut self) {
        match self.peek() {
            Some(x) => self.position = x,
            None => panic!("guard moved out of the map"),
        }
    }

    fn peek(&self) -> Option<(usize, usize)> {
        match self.orientation {
            GuardOrientation::Up => self.position.1.checked_sub(1).map(|y| (self.position.0, y)),
            GuardOrientation::Right => self.position.0.checked_add(1).map(|x| (x, self.position.1)),
            GuardOrientation::Down => self.position.1.checked_add(1).map(|y| (self.position.0, y)),
            GuardOrientation::Left => self.position.0.checked_sub(1).map(|x| (x, self.position.1)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    guard: Option<Guard>,
    size: (usize, usize),
    obstacles: HashSet<(usize, usize)>,
    iterations: usize,
}

impl TryFrom<&str> for Map {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value.lines();
        let height = lines.clone().count();
        let width = lines
            .clone()
            .next()
            .map(|l| l.len())
            .ok_or(Error::MapParsingFailed)?;
        if !lines.clone().all(|l| l.len() == width) {
            Err(Error::MapParsingFailed)
        } else {
            Ok(())
        }?;
        let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
        let mut opt_guard: Option<Guard> = None;
        for (y, line) in lines.enumerate() {
            match (&opt_guard, line.find('^')) {
                (None, Some(x)) => {
                    opt_guard = Some(Guard {
                        orientation: GuardOrientation::Up,
                        position: (x, y),
                    })
                }
                (Some(_), Some(_)) => return Err(Error::MapParsingFailed),
                _ => (),
            }
            match (&opt_guard, line.find('>')) {
                (None, Some(x)) => {
                    opt_guard = Some(Guard {
                        orientation: GuardOrientation::Right,
                        position: (x, y),
                    })
                }
                (Some(_), Some(_)) => return Err(Error::MapParsingFailed),
                _ => (),
            }
            match (&opt_guard, line.find('v')) {
                (None, Some(x)) => {
                    opt_guard = Some(Guard {
                        orientation: GuardOrientation::Down,
                        position: (x, y),
                    })
                }
                (Some(_), Some(_)) => return Err(Error::MapParsingFailed),
                _ => (),
            }
            match (&opt_guard, line.find('<')) {
                (None, Some(x)) => {
                    opt_guard = Some(Guard {
                        orientation: GuardOrientation::Left,
                        position: (x, y),
                    })
                }
                (Some(_), Some(_)) => return Err(Error::MapParsingFailed),
                _ => (),
            }
            let found_obstacles = line
                .match_indices('#')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>();
            for found in found_obstacles {
                obstacles.insert(found);
            }
        }
        match &opt_guard {
            Some(_) => Ok(Map {
                guard: opt_guard,
                size: (width, height),
                obstacles,
                iterations: 0usize,
            }),
            None => Err(Error::MapParsingFailed),
        }
    }
}

impl Iterator for Map {
    type Item = (usize, usize, GuardOrientation);
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.guard?.position;
        let orientation = self.guard?.orientation;
        self.guard = match self
            .guard?
            .peek()
            .filter(|(x, y)| *x < self.size.0 && *y < self.size.1)
        {
            Some(next_position) => {
                let mut guard = self.guard?;
                if self.obstacles.contains(&next_position) {
                    guard.rotate()
                } else {
                    guard.step()
                }
                Some(guard)
            }
            _ => None,
        };
        Some((x, y, orientation))
    }
}

pub fn count_positions(input: &str) -> Result<usize, Error> {
    let x: Map = input.try_into()?;
    Ok(x.into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<Vec<(usize, usize)>>()
        .into_iter()
        .collect::<HashSet<(usize, usize)>>()
        .len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn count_distinct_positions() {
        assert_eq!(count_positions(TEST_STR), Ok(41))
    }

    #[test]
    fn map_iteration() {
        let map: Map = TEST_STR.try_into().unwrap();
        assert_eq!(
            map.into_iter()
                .collect::<Vec<(usize, usize, GuardOrientation)>>(),
            vec![
                (4, 6, GuardOrientation::Up),
                (4, 5, GuardOrientation::Up),
                (4, 4, GuardOrientation::Up),
                (4, 3, GuardOrientation::Up),
                (4, 2, GuardOrientation::Up),
                (4, 1, GuardOrientation::Up),
                (4, 1, GuardOrientation::Right),
                (5, 1, GuardOrientation::Right),
                (6, 1, GuardOrientation::Right),
                (7, 1, GuardOrientation::Right),
                (8, 1, GuardOrientation::Right),
                (8, 1, GuardOrientation::Down),
                (8, 2, GuardOrientation::Down),
                (8, 3, GuardOrientation::Down),
                (8, 4, GuardOrientation::Down),
                (8, 5, GuardOrientation::Down),
                (8, 6, GuardOrientation::Down),
                (8, 6, GuardOrientation::Left),
                (7, 6, GuardOrientation::Left),
                (6, 6, GuardOrientation::Left),
                (5, 6, GuardOrientation::Left),
                (4, 6, GuardOrientation::Left),
                (3, 6, GuardOrientation::Left),
                (2, 6, GuardOrientation::Left),
                (2, 6, GuardOrientation::Up),
                (2, 5, GuardOrientation::Up),
                (2, 4, GuardOrientation::Up),
                (2, 4, GuardOrientation::Right),
                (3, 4, GuardOrientation::Right),
                (4, 4, GuardOrientation::Right),
                (5, 4, GuardOrientation::Right),
                (6, 4, GuardOrientation::Right),
                (6, 4, GuardOrientation::Down),
                (6, 5, GuardOrientation::Down),
                (6, 6, GuardOrientation::Down),
                (6, 7, GuardOrientation::Down),
                (6, 8, GuardOrientation::Down),
                (6, 8, GuardOrientation::Left),
                (5, 8, GuardOrientation::Left),
                (4, 8, GuardOrientation::Left),
                (3, 8, GuardOrientation::Left),
                (2, 8, GuardOrientation::Left),
                (1, 8, GuardOrientation::Left),
                (1, 8, GuardOrientation::Up),
                (1, 7, GuardOrientation::Up),
                (1, 7, GuardOrientation::Right),
                (2, 7, GuardOrientation::Right),
                (3, 7, GuardOrientation::Right),
                (4, 7, GuardOrientation::Right),
                (5, 7, GuardOrientation::Right),
                (6, 7, GuardOrientation::Right),
                (7, 7, GuardOrientation::Right),
                (7, 7, GuardOrientation::Down),
                (7, 8, GuardOrientation::Down),
                (7, 9, GuardOrientation::Down),
            ]
        );
    }

    #[test]
    fn parse_map() {
        assert_eq!(
            TEST_STR.try_into(),
            Ok(Map {
                guard: Some(Guard {
                    orientation: GuardOrientation::Up,
                    position: (4, 6)
                }),
                size: (10, 10),
                obstacles: HashSet::from([
                    (4, 0),
                    (9, 1),
                    (2, 3),
                    (7, 4),
                    (1, 6),
                    (8, 7),
                    (0, 8),
                    (6, 9)
                ]),
                iterations: 0,
            })
        )
    }
}
