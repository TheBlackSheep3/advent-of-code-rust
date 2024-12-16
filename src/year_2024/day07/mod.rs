pub mod error;

mod checked_op;
mod equation;
mod operation_selection;

use equation::Equation;
use error::Error;

pub fn get_sum_of_calibration_values(input: &str) -> Result<u64, Error> {
    input
        .lines()
        .into_iter()
        .map(|l| l.parse::<Equation<u64>>())
        .collect::<Result<Vec<_>, Error>>()?
        .iter()
        .filter(|e| e.is_solvable_add_mul())
        .fold(Some(0u64), |acc, i| {
            acc.and_then(|a| a.checked_add(i.expected))
        })
        .ok_or(Error::AccumulationFailed)
}

pub fn get_sum_of_calibration_values_with_concat(input: &str) -> Result<u128, Error> {
    input
        .lines()
        .into_iter()
        .map(|l| l.parse::<Equation<u128>>())
        .collect::<Result<Vec<_>, Error>>()?
        .iter()
        .filter(|e| e.is_solvable_add_mul_concat())
        .fold(Some(0u128), |acc, i| {
            acc.and_then(|a| a.checked_add(i.expected))
        })
        .ok_or(Error::AccumulationFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn sum_results() {
        assert_eq!(get_sum_of_calibration_values(TEST_STR), Ok(3749));
    }

    #[test]
    fn sum_results_with_concat() {
        assert_eq!(
            get_sum_of_calibration_values_with_concat(TEST_STR),
            Ok(11387)
        );
    }
}
