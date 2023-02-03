use std::num::ParseIntError;

pub fn get_top_n_calorie_sum(calorie_str: &str, n: usize) -> Result<i32, ParseIntError> {
    let mut highest: Vec<i32> = vec![0; n];
    let mut current: i32 = 0;
    for line in calorie_str.lines() {
        if line.is_empty() {
            swap_highest(current, &mut highest);
            current = 0;
            continue;
        }
        current += line.parse::<i32>()?;
    }
    swap_highest(current, &mut highest);
    Ok(highest.iter().fold(0, |acc, value| acc + value))
}

fn swap_highest(curr: i32, highest: &mut Vec<i32>) {
    let mut current = curr;
    for val in highest {
        if *val < current {
            let tmp = *val;
            *val = current;
            current = tmp;
        }
    }
}

pub fn get_highest_calories(calorie_str: &str) -> Result<i32, ParseIntError> {
    get_top_n_calorie_sum(calorie_str, 1)
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

    const FAULTY_STR: &str = "399
30
890

80

x
800
100

90";

    #[test]
    fn get_highest() {
        assert_eq!(get_highest_calories(TEST_STR).unwrap(), 24000);
    }

    #[test]
    fn get_top_3_sum() {
        assert_eq!(get_top_n_calorie_sum(TEST_STR, 3).unwrap(), 45000);
    }

    #[test]
    fn parse_error() {
        match get_highest_calories(FAULTY_STR) {
            Ok(_) => assert!(false, "this method should return a parsing error"),
            Err(e) => assert_eq!(get_typename_of(&e), std::any::type_name::<ParseIntError>()),
        }
        match get_top_n_calorie_sum(FAULTY_STR, 3) {
            Ok(_) => assert!(false, "this method should return a parsing error"),
            Err(e) => assert_eq!(get_typename_of(&e), std::any::type_name::<ParseIntError>()),
        }
    }

    fn get_typename_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }
}
