use std::collections::HashSet;

use phf::phf_map;

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
    use super::*;

    static SAMPE_INPUT: phf::Map<&'static str, i32> = phf_map! {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7,
        "bvwbjplbgvbhsrlpgdmjqwftvncz" => 5,
        "nppdvjthqldpwncqszvftbrmjlhg" => 6,
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 10,
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 11,
    };

    #[test]
    fn start_marker() {
        for (input, &marker) in &SAMPE_INPUT {
            assert_eq!(get_start_marker(input), Some(marker));
        }
    }
}
