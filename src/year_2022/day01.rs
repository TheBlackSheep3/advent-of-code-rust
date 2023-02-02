use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

pub fn get_highest_calories_from_input_file(path: &Path) -> Result<i32, Box<dyn Error>> {
    let calories_list = fs::read_to_string(path)?;
    match get_highest_calories_from_text(&calories_list) {
        Ok(x) => Ok(x),
        Err(e) => Err(Box::new(e)),
    }
}

fn get_highest_calories_from_text(calorie_str: &str) -> Result<i32, ParseIntError> {
    let mut highest: i32 = 0;
    let mut current: i32 = 0;
    for line in calorie_str.lines() {
        if line.is_empty() {
            if highest < current {
                highest = current;
            }
            current = 0;
            continue;
        }
        current += line.parse::<i32>()?;
    }
    Ok(highest)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_list() {
        let test_str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(get_highest_calories_from_text(&test_str).unwrap(), 24000);
    }
}
