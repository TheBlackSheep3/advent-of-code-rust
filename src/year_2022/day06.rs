use std::collections::HashSet;

pub fn get_start_marker(input: &str) -> Option<i32> {
    if input.len() < 4 {
        return None;
    }
    for i in 3..input.len() {
        if are_unique_chars(&input[i - 3..=i]) {
            return Some((i + 1) as i32);
        }
    }
    None
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
    use phf::phf_map;
    use super::*;

    static SAMPE_INPUT: phf::Map<&'static str, (i32, i32)> = phf_map! {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => (7, 19),
        "bvwbjplbgvbhsrlpgdmjqwftvncz" => (5, 23),
        "nppdvjthqldpwncqszvftbrmjlhg" => (6, 23),
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => (10, 29),
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => (11, 26),
    };

    #[test]
    fn start_marker() {
        for (input, &marker) in &SAMPE_INPUT {
            assert_eq!(get_start_marker(input), Some(marker.0));
        }
        assert_eq!(get_start_marker(""), None);
        assert_eq!(get_start_marker("abcabc"), None);
    }

    #[test]
    fn unique() {
        assert_eq!(are_unique_chars("bbbbbb"), false);
        assert_eq!(are_unique_chars("abcde"), true);
        assert_eq!(are_unique_chars("56as96d"), false);
    }
}
