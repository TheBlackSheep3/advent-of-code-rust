use std::num::ParseIntError;
use regex::Regex;

pub fn get_calibration_values(calibration_input: &str) -> Result<Vec<i32>, ParseIntError> {
    let lines = calibration_input.lines();
    let re = Regex::new(r"^\D*(?P<first>\d)?.*(?P<second>\d)").unwrap();
    Ok(lines.map(|l| {
        let caps = re.captures(l).expect("invalid calibration line");
        match caps.name("first") {
            Some(first) => {
                first.as_str().parse::<i32>().expect("failed to parse first digit") * 10 + caps.name("second").expect("failed to find second digit").as_str().parse::<i32>().expect("failed to parse second digit")
            }
            None => {
                let digit = caps.name("second").expect("failed to find single digit").as_str().parse::<i32>().expect("failed to parse single digit");
                digit * 11
            }
        }
    }).collect())
}

pub fn get_calibration_values_sum(calibration_input: &str) -> Result<i32, ParseIntError> {
    get_calibration_values(calibration_input).map(|v| v.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn get_sum() {
        assert_eq!(get_calibration_values_sum(TEST_STR).unwrap(), 142);
    }

    #[test]
    fn get_values() {
        assert_eq!(get_calibration_values(TEST_STR).unwrap(), vec![12, 38, 15, 77]);
    }
}
