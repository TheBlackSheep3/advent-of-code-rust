use std::collections::HashSet;

pub fn get_start_package(input: &str) -> Option<i32> {
    get_position_after_n_unique_chars(input, 4)
}

pub fn get_start_message(input: &str) -> Option<i32> {
    get_position_after_n_unique_chars(input, 14)
}

fn get_position_after_n_unique_chars(input: &str, n: usize) -> Option<i32> {
    if n < 2 || input.len() < n {
        None
    } else {
        let start = n.checked_sub(1)?;
        for i in start..input.len() {
            if are_unique_chars(&input[i - start..=i]) {
                let index = i.checked_add(1)?;
                return Some(index as i32);
            }
        }
        None
    }
}

fn are_unique_chars(input: &str) -> bool {
    let mut storage: HashSet<char> = HashSet::new();
    for c in input.chars() {
        if !storage.insert(c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(7))]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5))]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", Some(6))]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(10))]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(11))]
    #[case("", None)]
    #[case("abcabc", None)]
    fn start_package(#[case] input: &str, #[case] expected: Option<i32>) {
        assert_eq!(expected, get_start_package(input))
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(19))]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(23))]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", Some(23))]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(29))]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(26))]
    #[case("", None)]
    #[case("abcdefghijklmabcdefghijklm", None)]
    fn start_message(#[case] input: &str, #[case] expected: Option<i32>) {
        assert_eq!(expected, get_start_message(input))
    }

    #[rstest]
    #[case("abcde", true)]
    #[case("bbbbbb", false)]
    #[case("56as96d", false)]
    fn unique(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(expected, are_unique_chars(input))
    }
}
