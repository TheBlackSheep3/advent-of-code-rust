pub fn parse_and_execute_multiplication(instructions: &str) -> Option<u32> {
    let re = regex::Regex::new(r"mul\((?P<X>\d{1,3}),(?P<Y>\d{1,3})\)").ok()?;
    re.captures_iter(instructions)
        .map(|c| match (c.name("X"), c.name("Y")) {
            (Some(x), Some(y)) => x
                .as_str()
                .parse::<u32>()
                .ok()?
                .checked_mul(y.as_str().parse::<u32>().ok()?),
            _ => None,
        })
        .fold(None, |acc, op| match (acc, op) {
            (Some(v), Some(u)) => v.checked_add(u),
            (None, Some(v)) => Some(v),
            (Some(v), None) => Some(v),
            (None, None) => None,
        })
}

fn add_instruction_slice_to_instruction_vector<'a>(
    whole_instruction_string: &'a str,
    instruction_vector: &mut Vec<&'a str>,
    enabled: bool,
    begin: usize,
    end: usize,
) {
    if enabled {
        println!("pushing: '{}'", &whole_instruction_string[begin..end]);
        instruction_vector.push(&whole_instruction_string[begin..end]);
    }
}

pub fn parse_and_execute_multiplication_with_conditionals(instructions: &str) -> Option<u32> {
    const DO_STR: &str = "do()";
    const DONT_STR: &str = "don't()";
    let mut enabled: bool = true;
    let mut start_index: usize = 0usize;
    let mut end_index: usize = instructions.len();
    let mut covered_index: usize = 0usize;
    let mut result: Option<u32> = None;
    while covered_index < instructions.len() {
        if enabled {
            match &instructions[start_index..].find(DONT_STR) {
                Some(x) => end_index = *x + DONT_STR.len() + start_index,
                None => {}
            }
            result = match (
                result,
                parse_and_execute_multiplication(&instructions[start_index..end_index]),
            ) {
                (Some(x), Some(y)) => x.checked_add(y),
                (None, Some(y)) => Some(y),
                (Some(x), None) => Some(x),
                (None, None) => None,
            };
            covered_index = end_index;
        } else {
            match &instructions[end_index..].find(DO_STR) {
                Some(x) => start_index = *x + DO_STR.len() + end_index,
                None => start_index = instructions.len(),
            }
            end_index = instructions.len();
            covered_index = start_index;
        }
        enabled = !enabled;
    }
    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_STR1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_STR2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    const VALID_STR1: &str = "mul(44,46)";
    const VALID_STR2: &str = "mul(123,4)";

    const INVALID_STR1: &str = "mul(4*";
    const INVALID_STR2: &str = "mul(6,9!";
    const INVALID_STR3: &str = "?(12,34)";
    const INVALID_STR4: &str = "mul ( 2 , 4 )";

    #[rstest]
    #[case(VALID_STR1, Some(44 * 46))]
    #[case(VALID_STR2, Some(123 * 4))]
    #[case(INVALID_STR1, None)]
    #[case(INVALID_STR2, None)]
    #[case(INVALID_STR3, None)]
    #[case(INVALID_STR4, None)]
    fn parse_and_multiply_simple_examples(#[case] input: &str, #[case] expected: Option<u32>) {
        assert_eq!(expected, parse_and_execute_multiplication(input));
    }

    #[test]
    fn parse_and_multiply_longer_example() {
        assert_eq!(parse_and_execute_multiplication(TEST_STR1), Some(161));
    }

    #[test]
    fn parse_and_multiply_with_conditionals() {
        assert_eq!(
            parse_and_execute_multiplication_with_conditionals(TEST_STR2),
            Some(48)
        );
    }
}
