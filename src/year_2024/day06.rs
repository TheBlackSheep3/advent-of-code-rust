use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Error {
    MapParsingFailed,
    Threading,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MapParsingFailed => write!(f, "failed to parse map"),
            Error::Threading => write!(f, "error joining a thread"),
        }
    }
}

impl std::convert::From<std::boxed::Box<dyn std::any::Any + std::marker::Send>> for Error {
    fn from(_value: std::boxed::Box<dyn std::any::Any + std::marker::Send>) -> Self {
        Error::Threading
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
struct Map {
    guard: Option<Guard>,
    size: (usize, usize),
    obstacles: HashSet<(usize, usize)>,
    iterations: usize,
}

impl std::str::FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
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

impl Map {
    fn loops(&mut self) -> bool {
        let mut visited: HashSet<(usize, usize, GuardOrientation)> = HashSet::<_>::new();
        for x in self.into_iter() {
            if !visited.insert(x) {
                return true;
            }
        }
        false
    }
}

pub fn count_positions(input: &str) -> Result<usize, Error> {
    let x: Map = input.parse()?;
    Ok(x.into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<Vec<(usize, usize)>>()
        .into_iter()
        .collect::<HashSet<(usize, usize)>>()
        .len())
}

fn loop_check_thread_proc(map: &Map, possible_positions: &[(usize, usize)]) -> usize {
    let mut count = 0usize;
    for p in possible_positions {
        let mut map = map.clone();
        if !map.obstacles.insert(*p) {
            panic!("obstacle already present");
        }
        if map.loops() {
            count += 1;
        }
    }
    count
}

pub fn count_loop_positions(input: &str) -> Result<usize, Error> {
    let map: Map = input.parse()?;
    let possible_positions: Vec<(usize, usize)> = map
        .clone()
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    Ok(
        match std::thread::available_parallelism()
            .ok()
            .filter(|x| x.get() > 1usize)
        {
            None => loop_check_thread_proc(&map, &possible_positions),
            Some(thread_count) => {
                let thread_count = thread_count.get();
                let map: std::sync::Arc<Map> = std::sync::Arc::new(map);
                let possible_positions: std::sync::Arc<Vec<(usize, usize)>> =
                    std::sync::Arc::new(possible_positions);
                let mut position_count: usize;
                let step_size: usize = possible_positions.len() / thread_count;
                let mut threads: Vec<std::thread::JoinHandle<usize>> = vec![];
                for i in 0..thread_count - 1 {
                    let map = map.clone();
                    let possible_positions = possible_positions.clone();
                    threads.push(std::thread::spawn(move || {
                        loop_check_thread_proc(
                            &map,
                            &possible_positions[(i * step_size)..((i + 1) * step_size)],
                        )
                    }));
                }
                position_count = loop_check_thread_proc(
                    &map,
                    &possible_positions[((thread_count - 1) * step_size)..],
                );
                for thread in threads {
                    position_count += thread.join()?;
                }
                position_count
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

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
        let map: Map = TEST_STR.parse().unwrap();
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

    #[rstest]
    #[case((3usize, 6usize))]
    #[case((6usize, 7usize))]
    #[case((7usize, 7usize))]
    #[case((1usize, 8usize))]
    #[case((3usize, 8usize))]
    #[case((7usize, 9usize))]
    fn loop_test(#[case] obstacle_position: (usize, usize)) {
        let mut map: Map = TEST_STR.parse().unwrap();
        assert!(!map.clone().loops());
        if !map.obstacles.insert(obstacle_position) {
            panic!("obstacle already present");
        }
        assert!(map.loops());
    }

    #[test]
    fn count_possible_loops() {
        assert_eq!(count_loop_positions(TEST_STR), Ok(6));
    }

    #[test]
    fn parse_map() {
        assert_eq!(
            TEST_STR.parse(),
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
