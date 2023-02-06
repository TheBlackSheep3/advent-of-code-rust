use std::collections::HashMap;

pub fn get_top_crates(input: &str) -> String {
    parse_input(input);
    "".to_string()
}

fn parse_input(input: &str) {
    let mut lines: Vec<&str> = Vec::new();
    let mut split_index = 0;
    for (index, line) in input.lines().enumerate() {
        lines.push(line);
        if line.is_empty() {
            split_index = index
        }
    }
    parse_stacks(&lines[..split_index]);
    parse_instructions(&lines[split_index + 1..]);
}

fn parse_instructions(split_index: &[&str]) -> () {
    todo!()
}

fn parse_stacks(lines: &[&str]) -> Option<HashMap<i32, Vec<char>>> {
    let mut map: HashMap<i32, Vec<char>> = HashMap::new();
    let mut iterator = lines.iter().rev();
    let indices = iterator.next()?;
    for idx in indices.split_whitespace() {
        map.insert(idx.parse().ok()?, Vec::<char>::new());
    }
    let item_count = map.len();
    for x in iterator {
        let mut chars = x.chars();
        chars.next();
        let chars = chars;
        for (i,c) in chars.step_by(4).enumerate() {
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
        assert_eq!(get_top_crates(TEST_INPUT), "CMZ");
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
