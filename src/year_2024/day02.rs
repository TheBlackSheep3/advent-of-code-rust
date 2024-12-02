use std::{num::ParseIntError, u32};

fn parse_report(report: &str) -> Result<Vec<u32>, ParseIntError> {
    Ok(report
        .split(" ")
        .into_iter()
        .map(|d| d.parse::<u32>().unwrap())
        .collect())
}

fn report_is_safe(items: &Vec<u32>) -> bool {
    let zipped = items.iter().zip(items.iter().skip(1));
    zipped.len() > 0 &&(zipped
        .clone()
        .fold(true, |acc, (one, two)| acc && (one > two))
        || zipped
            .clone()
            .fold(true, |acc, (one, two)| acc && (one < two)))
        && zipped.clone().fold(true, |acc, (one, two)| {
            let diff: u32 = one.abs_diff(*two);
            acc && (diff >= 1 && diff <= 3)
        })
}

fn is_safe(report: &str) -> Result<bool, ParseIntError> {
    let digits: Vec<u32> = parse_report(report)?;
    Ok(report_is_safe(&digits))
}

fn is_safe_dampened(report: &str) -> Result<bool, ParseIntError> {
    let digits: Vec<u32> = parse_report(report)?;
    if report_is_safe(&digits) {
        Ok(true)
    } else {
        for x in 0..digits.len() {
            let mut modified: Vec<u32> = digits.clone();
            modified.remove(x);
            if report_is_safe(&modified) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub fn count_safe_reports(input: &str) -> Result<usize, ParseIntError> {
    Ok(input
        .lines()
        .into_iter()
        .filter(|l| is_safe(l).unwrap())
        .count()
        .try_into()
        .unwrap())
}

pub fn count_safe_reports_dampened(input: &str) -> Result<usize, ParseIntError> {
    Ok(input
        .lines()
        .into_iter()
        .filter(|l| is_safe_dampened(l).unwrap())
        .count()
        .try_into()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn check_is_safe() {
        assert_eq!(is_safe("7 6 4 2 1"), Ok(true));
        assert_eq!(is_safe("1 2 7 8 9"), Ok(false));
        assert_eq!(is_safe("9 7 6 2 1"), Ok(false));
        assert_eq!(is_safe("1 3 2 4 5"), Ok(false));
        assert_eq!(is_safe("8 6 4 4 1"), Ok(false));
        assert_eq!(is_safe("1 3 6 7 9"), Ok(true));
    }

    #[test]
    fn count_safe() {
        assert_eq!(count_safe_reports(TEST_STR), Ok(2));
    }

    #[test]
    fn check_is_safe_dampened() {
        assert_eq!(is_safe_dampened("7 6 4 2 1"), Ok(true));
        assert_eq!(is_safe_dampened("1 2 7 8 9"), Ok(false));
        assert_eq!(is_safe_dampened("9 7 6 2 1"), Ok(false));
        assert_eq!(is_safe_dampened("1 3 2 4 5"), Ok(true));
        assert_eq!(is_safe_dampened("8 6 4 4 1"), Ok(true));
        assert_eq!(is_safe_dampened("1 3 6 7 9"), Ok(true));
    }

    #[test]
    fn count_safe_dampened() {
        assert_eq!(count_safe_reports_dampened(TEST_STR), Ok(4));
    }
}
