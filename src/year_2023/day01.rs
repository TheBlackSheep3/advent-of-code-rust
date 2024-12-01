use regex::Regex;
use std::num::ParseIntError;

fn get_calibration_values_only_digits(calibration_input: &str) -> Result<Vec<i32>, ParseIntError> {
    let lines = calibration_input.lines();
    let re = Regex::new(r"^\D*(?P<first>\d)?.*(?P<second>\d)").unwrap();
    Ok(lines
        .map(|l| {
            let caps = re.captures(l).expect("invalid calibration line");
            match caps.name("first") {
                Some(first) => {
                    first
                        .as_str()
                        .parse::<i32>()
                        .expect("failed to parse first digit")
                        * 10
                        + caps
                            .name("second")
                            .expect("failed to find second digit")
                            .as_str()
                            .parse::<i32>()
                            .expect("failed to parse second digit")
                }
                None => {
                    let digit = caps
                        .name("second")
                        .expect("failed to find single digit")
                        .as_str()
                        .parse::<i32>()
                        .expect("failed to parse single digit");
                    digit * 11
                }
            }
        })
        .collect())
}

pub fn get_calibration_values_sum_only_digits(
    calibration_input: &str,
) -> Result<i32, ParseIntError> {
    get_calibration_values_only_digits(calibration_input).map(|v| v.iter().sum())
}

pub fn get_calibration_values_sum_with_words(
    calibration_input: &str,
) -> Result<i32, ParseIntError> {
    get_calibration_values_with_words(calibration_input).map(|v| v.iter().sum())
}

fn get_calibration_values_with_words(calibration_input: &str) -> Result<Vec<i32>, ParseIntError> {
    let lines = calibration_input.lines();
    let re = Regex::new(r"^\D*(?P<first>\d)?.*(?P<second>\d)").unwrap();
    Ok(lines
        .map(|l| {

            let caps = re.captures(l).expect("invalid calibration line");
            match caps.name("first") {
                Some(first) => {
                    first
                        .as_str()
                        .parse::<i32>()
                        .expect("failed to parse first digit")
                        * 10
                        + caps
                            .name("second")
                            .expect("failed to find second digit")
                            .as_str()
                            .parse::<i32>()
                            .expect("failed to parse second digit")
                }
                None => {
                    let digit = caps
                        .name("second")
                        .expect("failed to find single digit")
                        .as_str()
                        .parse::<i32>()
                        .expect("failed to parse single digit");
                    digit * 11
                }
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_STR_WORDS: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn get_sum_only_digits() {
        assert_eq!(
            get_calibration_values_sum_only_digits(TEST_STR).unwrap(),
            142
        );
    }

    #[test]
    fn get_sum_with_word() {
        assert_eq!(
            get_calibration_values_sum_with_words(TEST_STR_WORDS).unwrap(),
            281
        );
    }

    #[test]
    fn get_values_only_digits() {
        assert_eq!(
            get_calibration_values_only_digits(TEST_STR).unwrap(),
            vec![12, 38, 15, 77]
        );
    }

    #[test]
    fn get_values_with_words() {
        assert_eq!(
            get_calibration_values_with_words(TEST_STR_WORDS).unwrap(),
            vec![29, 83, 13, 24, 42, 14, 76]
        );
    }
}
