pub fn get_priority_sum(input: &str) -> Option<i32> {
    let mut sum: i32 = 0;
    for line in input.lines() {
        match get_duplicate_from_halves(line.trim()) {
            Some(c) => sum += get_priority(c),
            None => return None,
        }
    }
    Some(sum)
}

pub fn get_badge_sum(input: &str) -> Option<i32> {
    let mut sum: i32 = 0;
    let mut lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        lines.push(line);
    }
    if lines.len() % 3 != 0 {
        return None;
    }
    for group in lines.chunks_exact(3) {
        match get_batch_type(group) {
            Some(c) => sum += get_priority(c),
            None => return None,
        }
    }

    Some(sum)
}

fn get_batch_type(group: &[&str]) -> Option<char> {
    let one = group.first().unwrap();
    let two = group.get(1).unwrap();
    let three = group.get(2).unwrap();
    for c in one.chars() {
        match two.find(c) {
            None => continue,
            Some(_) => match three.find(c) {
                None => continue,
                Some(_) => return Some(c),
            },
        }
    }
    None
}

fn get_duplicate_from_halves(line: &str) -> Option<char> {
    let len: usize = line.len();
    if len == 0 || len % 2 != 0 {
        return None;
    }
    let first: &str = &line[..len / 2];
    let second: &str = &line[len / 2..];
    for c in first.chars() {
        match second.find(c) {
            None => continue,
            Some(_) => return Some(c),
        }
    }
    None
}

fn get_priority(c: char) -> i32 {
    const LOWERCASE_OFFSET: i32 = 96;
    const UPPERCASE_OFFSET: i32 = 38;
    match c {
        'a'..='z' => c as i32 - LOWERCASE_OFFSET,
        'A'..='Z' => c as i32 - UPPERCASE_OFFSET,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority() {
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
        assert_eq!(get_priority('P'), 42);
        assert_eq!(get_priority('v'), 22);
        assert_eq!(get_priority('t'), 20);
        assert_eq!(get_priority('s'), 19);
    }

    #[test]
    fn badge_sum() {
        assert_eq!(get_badge_sum(TEST_INPUT).unwrap(), 70);
    }

    #[test]
    fn get_sum() {
        assert_eq!(get_priority_sum(TEST_INPUT).unwrap(), 157);
    }
}
