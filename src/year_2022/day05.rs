use regex::Regex;
use std::collections::HashMap;

pub fn get_top_crates(input: &str) -> Option<String> {
    let problem = parse_input(input);
    Some("".to_string())
}

struct Instruction {
    amount: i32,
    source: i32,
    destination: i32,
}

struct Problem {
    stacks: HashMap<i32, Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Problem {
    fn get_top_string(&self) -> Option<String> {
        let mut char_vector: Vec<char> = Vec::new();
        for i in 0..self.stacks.len() {
            char_vector.push(*self.stacks.get(&(i as i32))?.last()?);
        }
        Some(char_vector.iter().fold(String::new(), |mut string, &c| {
            string.push(c);
            string
        }))
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
                amount: amount,
                source: source,
                destination: destination,
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
    fn top_crates() {
        assert_eq!(get_top_crates(TEST_INPUT), Some("CMZ".to_string()));
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
