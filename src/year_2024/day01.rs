use std::num::ParseIntError;

fn parse_lists(list_input: &str) -> Result<(Vec<u32>, Vec<u32>), ParseIntError> {
    let lines = list_input.lines();
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];
    for line in lines {
        let mut split = line.split("   ");
        left_list.push(
            split
                .next()
                .expect("expected 'val1   val2'")
                .parse::<u32>()?,
        );
        right_list.push(
            split
                .next()
                .expect("expected 'val1   val2'")
                .parse::<u32>()?,
        );
    }
    Ok((left_list, right_list))
}

pub fn get_list_difference(list_input: &str) -> Result<u32, ParseIntError> {
    let (mut left_list, mut right_list) = parse_lists(list_input)?;
    left_list.sort();
    right_list.sort();

    Ok(left_list.iter().zip(right_list.iter()).fold(0u32, |acc, (l, r)| acc + l.abs_diff(*r)))
}

pub fn get_list_similarity_score(list_input: &str) -> Result<u32, ParseIntError> {
    let (left_list, right_list) = parse_lists(list_input)?;
    
    Ok(left_list.iter().fold(0u32, |acc, i| acc + (*i * u32::try_from(right_list.iter().filter(|r| *r == i).count()).unwrap())))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn get_difference() {
        assert_eq!(get_list_difference(TEST_STR), Ok(11u32));
    }

    #[test]
    fn get_similarity_score() {
        assert_eq!(get_list_similarity_score(TEST_STR), Ok(31u32));
    }
}
