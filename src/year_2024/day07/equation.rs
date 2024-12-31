use super::checked_op::CheckedOp;
use super::error::Error;
use super::operation_selection::*;

#[derive(Debug, PartialEq)]
pub struct Equation<T> {
    pub expected: T,
    pub test_values: Vec<T>,
}

impl<T: std::str::FromStr> std::str::FromStr for Equation<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"(?P<expected>\d+):(?P<test_values>(?: \d+)+)").unwrap();
        let expected: T;
        match re.captures(s) {
            None => Err(Error::ParsingFailed),
            Some(c) => {
                expected = c
                    .name("expected")
                    .ok_or(Error::ParsingFailed)?
                    .as_str()
                    .parse::<T>()
                    .map_err(|_| Error::IntegerTypeTooSmall)?;
                let test_values = c
                    .name("test_values")
                    .ok_or(Error::ParsingFailed)?
                    .as_str()
                    .trim()
                    .split(' ')
                    .into_iter()
                    .map(|str| str.parse::<T>().map_err(|_| Error::IntegerTypeTooSmall))
                    .into_iter()
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(Equation {
                    expected,
                    test_values,
                })
            }
        }
    }
}

impl<T: CheckedOp + PartialEq + Clone> Equation<T> {
    pub fn is_solvable_add_mul(&self) -> bool {
        let len = self.test_values.len();
        if len == 1usize {
            self.expected == *self.test_values.first().unwrap()
        } else {
            for i in 0..1 << (self.test_values.len() - 1) {
                let should_add_bit_vector = get_bit_vector(i, len - 1).unwrap();
                let mut iter = self.test_values.iter();
                let mut val: Option<T> = iter.next().map(|x| x.clone());
                for (value, perform_addition) in iter.zip(should_add_bit_vector.iter()) {
                    match val {
                        None => break,
                        Some(v) => {
                            if *perform_addition {
                                val = v.checked_add(value.clone());
                            } else {
                                val = v.checked_mul(value.clone());
                            }
                        }
                    }
                }
                if Some(self.expected.clone()) == val {
                    return true;
                }
            }
            false
        }
    }

    pub fn is_solvable_add_mul_concat(&self) -> bool {
        let len = self.test_values.len();
        if len == 1usize {
            self.expected == *self.test_values.first().unwrap()
        } else {
            for i in 0u32..3u32.pow(
                <usize as std::convert::TryInto<u32>>::try_into(self.test_values.len()).unwrap()
                    - 1u32,
            ) {
                let operation_vector = get_enumeration_vector(i, len - 1).unwrap();
                let mut iter = self.test_values.iter();
                let mut val: Option<T> = iter.next().map(|x| x.clone());
                for (value, operation) in iter.zip(operation_vector.iter()) {
                    match val {
                        None => break,
                        Some(v) => match operation {
                            Operation::Addition => {
                                val = v.checked_add(value.clone());
                            }
                            Operation::Multiplication => {
                                val = v.checked_mul(value.clone());
                            }
                            Operation::Concatination => {
                                val = v.concat(value.clone());
                            }
                        },
                    }
                }
                if Some(self.expected.clone()) == val {
                    return true;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Equation;
    use super::Error;

    #[rstest]
    #[case("190: 10 19", Ok(Equation::<u8> { expected: 190u8, test_values: vec![10u8, 19u8] }))]
    #[case("3267: 81 40 27", Err(Error::IntegerTypeTooSmall))]
    #[case("161011: 16 10 13", Err(Error::IntegerTypeTooSmall))]
    #[case("123", Err(Error::ParsingFailed))]
    fn parse_u8(#[case] input: &str, #[case] expected: Result<Equation<u8>, Error>) {
        assert_eq!(expected, input.parse())
    }

    #[rstest]
    #[case("190: 10 19", Ok(Equation::<u16> { expected: 190u16, test_values: vec![10u16, 19u16] }))]
    #[case("3267: 81 40 27", Ok(Equation::<u16> { expected: 3267u16, test_values: vec![81u16, 40u16, 27u16] }))]
    #[case("161011: 16 10 13", Err(Error::IntegerTypeTooSmall))]
    #[case("123", Err(Error::ParsingFailed))]
    fn parse_u16(#[case] input: &str, #[case] expected: Result<Equation<u16>, Error>) {
        assert_eq!(expected, input.parse())
    }

    #[rstest]
    #[case("190: 10 19", Ok(Equation::<u32> { expected: 190u32, test_values: vec![10u32, 19u32] }))]
    #[case("3267: 81 40 27", Ok(Equation::<u32> { expected: 3267u32, test_values: vec![81u32, 40u32, 27u32] }))]
    #[case("161011: 16 10 13", Ok(Equation::<u32> { expected: 161011u32, test_values: vec![16u32, 10u32, 13u32] }))]
    #[case("123", Err(Error::ParsingFailed))]
    fn parse_u32(#[case] input: &str, #[case] expected: Result<Equation<u32>, Error>) {
        assert_eq!(expected, input.parse())
    }

    #[rstest]
    #[case("190: 10 19", true)]
    #[case("3267: 81 40 27", true)]
    #[case("83: 17 5", false)]
    #[case("156: 15 6", false)]
    #[case("7290: 6 8 6 15", false)]
    #[case("161011: 16 10 13", false)]
    #[case("192: 17 8 14", false)]
    #[case("21037: 9 7 18 13", false)]
    #[case("292: 11 6 16 20", true)]
    fn check_solvable(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(
            expected,
            input
                .parse::<Equation<usize>>()
                .unwrap()
                .is_solvable_add_mul()
        )
    }

    #[rstest]
    #[case("190: 10 19", true)]
    #[case("3267: 81 40 27", true)]
    #[case("83: 17 5", false)]
    #[case("156: 15 6", true)]
    #[case("7290: 6 8 6 15", true)]
    #[case("161011: 16 10 13", false)]
    #[case("192: 17 8 14", true)]
    #[case("21037: 9 7 18 13", false)]
    #[case("292: 11 6 16 20", true)]
    fn check_solvable_with_concat(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(
            expected,
            input
                .parse::<Equation<usize>>()
                .unwrap()
                .is_solvable_add_mul_concat()
        )
    }
}
