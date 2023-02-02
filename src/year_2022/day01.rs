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

pub fn get_top_n_calorie_sum_from_input_file(path: &Path, n: usize) -> Result<i32, Box<dyn Error>> {
    let calories_list = fs::read_to_string(path)?;
    match get_top_n_calorie_sum_from_text(&calories_list, n) {
        Ok(x) => Ok(x),
        Err(e) => Err(Box::new(e)),
    }
}

fn get_top_n_calorie_sum_from_text(calorie_str: &str, n: usize) -> Result<i32, ParseIntError> {
    let mut highest: Vec<i32> = vec![0; n];
    let mut current: i32 = 0;
    for line in calorie_str.lines() {
        if line.is_empty() {
            foo(current, &mut highest);
            current = 0;
            continue;
        }
        current += line.parse::<i32>()?;
    }
    foo(current, &mut highest);
    Ok(highest.iter().fold(0, |acc, value| acc + value))
}

fn foo(curr: i32, highest: &mut Vec<i32>) {
    let mut current = curr;
    for val in highest {
        if *val < current {
            let tmp = *val;
            *val = current;
            current = tmp;
        }
    }
}

fn get_highest_calories_from_text(calorie_str: &str) -> Result<i32, ParseIntError> {
    get_top_n_calorie_sum_from_text(calorie_str, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STR: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    #[test]
    fn get_highest() {
        assert_eq!(get_highest_calories_from_text(&TEST_STR).unwrap(), 24000);
    }

    #[test]
    fn get_top_3_sum() {
        assert_eq!(
            get_top_n_calorie_sum_from_text(&TEST_STR, 3).unwrap(),
            45000
        );
    }
}
