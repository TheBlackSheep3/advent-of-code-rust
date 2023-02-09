use regex::Regex;
use std::collections::HashMap;

pub fn get_top_crates_one_at_a_time(input: &str) -> Result<String, CrateProblemError> {
    let mut problem = match parse_input(input) {
        Some(p) => p,
        None => return Err(CrateProblemError::Parse),
    };
    problem.execute_instructions_v9000()?;
    match problem.get_top_string() {
        Some(s) => Ok(s),
        None => Err(CrateProblemError::Evaluate),
    }
}

pub fn get_top_crates_multiple_at_a_time(input: &str) -> Result<String, CrateProblemError> {
    let mut problem = match parse_input(input) {
        Some(p) => p,
        None => return Err(CrateProblemError::Parse),
    };
    problem.execute_instructions_v9001()?;
    match problem.get_top_string() {
        Some(s) => Ok(s),
        None => Err(CrateProblemError::Evaluate),
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Instruction {
    amount: i32,
    source: i32,
    destination: i32,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.amount, self.source, self.destination
        )
    }
}

struct Problem {
    stacks: HashMap<i32, Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Problem {
    fn get_top_string(&self) -> Option<String> {
        let mut char_vector: Vec<char> = Vec::new();
        // assert that stacks are labeled 1..n
        for i in 1..self.stacks.len() + 1 {
            char_vector.push(*self.stacks.get(&(i as i32))?.last()?);
        }
        Some(char_vector.iter().fold(String::new(), |mut string, &c| {
            string.push(c);
            string
        }))
    }

    fn execute_instructions_v9000(&mut self) -> Result<(), CrateProblemError> {
        for instruction in self.instructions.clone().iter() {
            for _i in 0..instruction.amount {
                let source_stack = match self.stacks.get_mut(&instruction.source) {
                    Some(s) => s,
                    None => return Err(CrateProblemError::InstructionInvalidSource(*instruction)),
                };
                let cr = match source_stack.pop() {
                    Some(c) => c,
                    None => return Err(CrateProblemError::InstructionInvalidAmount(*instruction)),
                };
                let destination_stack = match self.stacks.get_mut(&instruction.destination) {
                    Some(s) => s,
                    None => {
                        return Err(CrateProblemError::InstructionInvalidDestination(
                            *instruction,
                        ))
                    }
                };
                destination_stack.push(cr);
            }
        }
        self.instructions.clear();
        Ok(())
    }
    fn execute_instructions_v9001(&mut self) -> Result<(), CrateProblemError> {
        for instruction in self.instructions.clone().iter() {
            let mut crate_holder = Vec::new();
            let source_stack = match self.stacks.get_mut(&instruction.source) {
                Some(s) => s,
                None => return Err(CrateProblemError::InstructionInvalidSource(*instruction)),
            };
            for _i in 0..instruction.amount {
                let cr = match source_stack.pop() {
                    Some(c) => c,
                    None => return Err(CrateProblemError::InstructionInvalidAmount(*instruction)),
                };
                crate_holder.push(cr);
            }
            let destination_stack = match self.stacks.get_mut(&instruction.destination) {
                Some(s) => s,
                None => {
                    return Err(CrateProblemError::InstructionInvalidDestination(
                        *instruction,
                    ))
                }
            };
            for cr in crate_holder.iter().rev() {
                destination_stack.push(*cr);
            }
        }
        self.instructions.clear();
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum CrateProblemError {
    Parse,
    Evaluate,
    InstructionInvalidAmount(Instruction),
    InstructionInvalidSource(Instruction),
    InstructionInvalidDestination(Instruction),
}

impl std::fmt::Display for CrateProblemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Debug for CrateProblemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (string, instruction) = match self {
            CrateProblemError::Evaluate => return write!(f, "failed to evaluate top string"),
            CrateProblemError::Parse => return write!(f, "failed to parse problem"),
            CrateProblemError::InstructionInvalidAmount(a) => ("invalid amount", a),
            CrateProblemError::InstructionInvalidSource(a) => ("invalid source", a),
            CrateProblemError::InstructionInvalidDestination(a) => ("invalid destination", a),
        };
        write!(f, "{}: {:?}", string, instruction)
    }
}

fn parse_input(input: &str) -> Option<Problem> {
    let mut lines: Vec<&str> = Vec::new();
    let mut split_index = 0;
    for (index, line) in input.lines().enumerate() {
        lines.push(line);
        if line.is_empty() {
            split_index = index
        }
    }
    let stacks = parse_stacks(&lines[..split_index]);
    let instructions = parse_instructions(&lines[split_index + 1..]);
    Some(Problem {
        stacks: stacks?,
        instructions: instructions?,
    })
}

fn parse_instructions(instructions: &[&str]) -> Option<Vec<Instruction>> {
    let mut instruction_vector: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").ok()?;
    for line in instructions {
        if re.is_match(line) {
            let captures = re.captures_iter(line).next()?;
            let mut groups_iterator = captures.iter();
            groups_iterator.next();
            let amount = groups_iterator.next()??.as_str().parse::<i32>().ok()?;
            let source = groups_iterator.next()??.as_str().parse::<i32>().ok()?;
            let destination = groups_iterator.next()??.as_str().parse::<i32>().ok()?;
            instruction_vector.push(Instruction {
                amount,
                source,
                destination,
            });
        }
    }
    Some(instruction_vector)
}

fn parse_stacks(lines: &[&str]) -> Option<HashMap<i32, Vec<char>>> {
    let mut map: HashMap<i32, Vec<char>> = HashMap::new();
    let mut iterator = lines.iter().rev();
    let indices = iterator.next()?;
    for idx in indices.split_whitespace() {
        map.insert(idx.parse().ok()?, Vec::<char>::new());
    }
    for x in iterator {
        let mut chars = x.chars();
        chars.next();
        let chars = chars;
        for (i, c) in chars.step_by(4).enumerate() {
            if c.is_alphabetic() {
                map.get_mut(&(i as i32 + 1))?.push(c);
            }
        }
    }
    Some(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn top_crates_one() {
        assert_eq!(get_top_crates_one_at_a_time(TEST_INPUT), Ok("CMZ".to_string()));
    }

    #[test]
    fn top_crates_multiple() {
        assert_eq!(get_top_crates_multiple_at_a_time(TEST_INPUT), Ok("MCD".to_string()));
    }

    #[test]
    fn parse_instructions_test() {
        let e = parse_instructions(
            &TEST_INPUT.lines().fold(Vec::<&str>::new(), |mut vec, i| {
                vec.push(i);
                vec
            })[5..],
        )
        .unwrap();
        assert_eq!(e.len(), 4);
    }

    #[test]
    fn top_string() {
        let mut p = Problem {
            stacks: HashMap::<i32, Vec<char>>::new(),
            instructions: Vec::<Instruction>::new(),
        };
        p.stacks.insert(1, vec!['A', 'B', 'C']);
        p.stacks.insert(2, vec!['D']);
        p.stacks.insert(3, vec!['E', 'F']);
        p.stacks.insert(4, vec!['G', 'H', 'I', 'J']);
        assert_eq!(p.get_top_string(), Some("CDFJ".to_string()));
    }

    #[test]
    fn execute_9001() {
        let mut p = Problem {
            stacks: HashMap::<i32, Vec<char>>::new(),
            instructions: Vec::<Instruction>::new(),
        };
        p.stacks.insert(1, vec!['A', 'B', 'C']);
        p.stacks.insert(2, vec!['D']);
        p.stacks.insert(3, vec!['E', 'F']);
        p.stacks.insert(4, vec!['G', 'H', 'I', 'J']);
        let mut expected1 = Problem {
            stacks: HashMap::<i32, Vec<char>>::new(),
            instructions: Vec::<Instruction>::new(),
        };
        expected1.stacks.insert(1, vec!['A', 'B', 'C']);
        expected1.stacks.insert(2, vec!['D']);
        expected1.stacks.insert(3, vec!['E', 'F']);
        expected1.stacks.insert(4, vec!['G', 'H', 'I', 'J']);
        let mut expected2 = Problem {
            stacks: HashMap::<i32, Vec<char>>::new(),
            instructions: Vec::<Instruction>::new(),
        };
        expected2.stacks.insert(1, vec!['A', 'I', 'J']);
        expected2.stacks.insert(2, vec!['D', 'B', 'C']);
        expected2.stacks.insert(3, vec!['E', 'F']);
        expected2.stacks.insert(4, vec!['G', 'H']);
        p.execute_instructions_v9001().unwrap();
        assert_eq!(p.stacks, expected1.stacks);
        p.instructions.push(Instruction {
            amount: 2,
            source: 1,
            destination: 2,
        });
        p.instructions.push(Instruction {
            amount: 2,
            source: 4,
            destination: 1,
        });
        p.execute_instructions_v9001().unwrap();
        assert_eq!(p.stacks, expected2.stacks);
        p.execute_instructions_v9001().unwrap();
        assert_eq!(p.stacks, expected2.stacks);
    }

    #[test]
    fn execute_9000() {
        let mut p = Problem {
            stacks: HashMap::<i32, Vec<char>>::new(),
            instructions: Vec::<Instruction>::new(),
        };
        p.stacks.insert(1, vec!['A', 'B', 'C']);
        p.stacks.insert(2, vec!['D']);
        p.stacks.insert(3, vec!['E', 'F']);
        p.stacks.insert(4, vec!['G', 'H', 'I', 'J']);
        let mut expected1 = Problem {
            stacks: HashMap::<i32, Vec<char>>::new(),
            instructions: Vec::<Instruction>::new(),
        };
        expected1.stacks.insert(1, vec!['A', 'B', 'C']);
        expected1.stacks.insert(2, vec!['D']);
        expected1.stacks.insert(3, vec!['E', 'F']);
        expected1.stacks.insert(4, vec!['G', 'H', 'I', 'J']);
        let mut expected2 = Problem {
            stacks: HashMap::<i32, Vec<char>>::new(),
            instructions: Vec::<Instruction>::new(),
        };
        expected2.stacks.insert(1, vec!['A', 'J', 'I']);
        expected2.stacks.insert(2, vec!['D', 'C', 'B']);
        expected2.stacks.insert(3, vec!['E', 'F']);
        expected2.stacks.insert(4, vec!['G', 'H']);
        p.execute_instructions_v9000().unwrap();
        assert_eq!(p.stacks, expected1.stacks);
        p.instructions.push(Instruction {
            amount: 2,
            source: 1,
            destination: 2,
        });
        p.instructions.push(Instruction {
            amount: 2,
            source: 4,
            destination: 1,
        });
        p.execute_instructions_v9000().unwrap();
        assert_eq!(p.stacks, expected2.stacks);
        p.execute_instructions_v9000().unwrap();
        assert_eq!(p.stacks, expected2.stacks);
    }

    #[test]
    fn parse_stacks_test() {
        let e = parse_stacks(
            &TEST_INPUT.lines().fold(Vec::<&str>::new(), |mut vec, i| {
                vec.push(i);
                vec
            })[..4],
        )
        .unwrap();
        assert_eq!(e.len(), 3);
        println!("{e:?}");
        assert_eq!(e.get(&1).unwrap().len(), 2);
        assert_eq!(e.get(&2).unwrap().len(), 3);
        assert_eq!(e.get(&3).unwrap().len(), 1);
    }
}
