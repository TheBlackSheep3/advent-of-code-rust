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
    use super::*;
    use phf::phf_map;

    static SAMPE_INPUT: phf::Map<&'static str, (i32, i32)> = phf_map! {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => (7, 19),
        "bvwbjplbgvbhsrlpgdmjqwftvncz" => (5, 23),
        "nppdvjthqldpwncqszvftbrmjlhg" => (6, 23),
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => (10, 29),
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => (11, 26),
    };

    #[test]
    fn start_package() {
        for (input, &marker) in &SAMPE_INPUT {
            assert_eq!(get_start_package(input), Some(marker.0));
        }
        assert_eq!(get_start_package(""), None);
        assert_eq!(get_start_package("abcabc"), None);
    }

    #[test]
    fn start_message() {
        for (input, &marker) in &SAMPE_INPUT {
            assert_eq!(get_start_message(input), Some(marker.1));
        }
        assert_eq!(get_start_message(""), None);
        assert_eq!(get_start_message("abcdefghijklmabcdefghijklm"), None)
    }

    #[test]
    fn unique() {
        assert!(!are_unique_chars("bbbbbb"));
        assert!(are_unique_chars("abcde"));
        assert!(!are_unique_chars("56as96d"));
    }
}
