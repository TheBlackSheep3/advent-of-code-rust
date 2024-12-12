#[derive(Debug, PartialEq)]
pub enum Error {
    ParsingFailed,
    BitFieldGeneration,
    AccumulationFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParsingFailed => write!(f, "failed to parse input"),
            Error::BitFieldGeneration => write!(
                f,
                "failed to generate proper bit field from unsinged integer"
            ),
            Error::AccumulationFailed => write!(f, "result accumulation failed"),
        }
    }
}

trait CheckedOp {
    fn checked_add(self, rhs: Self) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn checked_mul(self, rhs: Self) -> Option<Self>
    where
        Self: std::marker::Sized;
}

macro_rules! checkedop_impl {
    () => {
        fn checked_add(self, rhs: Self) -> Option<Self> {
            self.checked_add(rhs)
        }
        fn checked_mul(self, rhs: Self) -> Option<Self> {
            self.checked_mul(rhs)
        }
    };
}

impl CheckedOp for u8 {
    checkedop_impl!();
}
impl CheckedOp for u16 {
    checkedop_impl!();
}
impl CheckedOp for u32 {
    checkedop_impl!();
}
impl CheckedOp for u64 {
    checkedop_impl!();
}
impl CheckedOp for usize {
    checkedop_impl!();
}
impl CheckedOp for i8 {
    checkedop_impl!();
}
impl CheckedOp for i16 {
    checkedop_impl!();
}
impl CheckedOp for i32 {
    checkedop_impl!();
}
impl CheckedOp for i64 {
    checkedop_impl!();
}

#[derive(Debug, PartialEq)]
struct Equation<
    T: CheckedOp + std::str::FromStr + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
> {
    expected: T,
    test_values: Vec<T>,
}

impl<
        T: CheckedOp
            + std::str::FromStr
            + std::cmp::PartialEq
            + std::cmp::PartialOrd
            + std::clone::Clone,
    > std::str::FromStr for Equation<T>
{
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
                    .map_err(|_| Error::ParsingFailed)?;
                let test_values = c
                    .name("test_values")
                    .ok_or(Error::ParsingFailed)?
                    .as_str()
                    .trim()
                    .split(' ')
                    .into_iter()
                    .map(|str| str.parse::<T>().map_err(|_| Error::ParsingFailed))
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

fn get_bit_vector(unsigned_integer: usize, len: usize) -> Result<Vec<bool>, Error> {
    if unsigned_integer > (1 << len) - 1 {
        Err(Error::BitFieldGeneration)
    } else {
        let mut unsigned_integer = unsigned_integer;
        let mut bit_vector = Vec::new();
        while bit_vector.len() < len {
            bit_vector.push(unsigned_integer % 2 == 1);
            unsigned_integer >>= 1; // divide by 2
        }
        Ok(bit_vector)
    }
}

impl<
        T: CheckedOp
            + std::str::FromStr
            + std::cmp::PartialEq
            + std::cmp::PartialOrd
            + std::clone::Clone,
    > Equation<T>
{
    fn is_solvable(&self) -> bool {
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
}

pub fn get_sum_of_calibration_values(input: &str) -> Result<u64, Error> {
    input
        .lines()
        .into_iter()
        .map(|l| l.parse::<Equation<u64>>())
        .collect::<Result<Vec<_>, Error>>()?
        .iter()
        .filter(|e| e.is_solvable())
        .fold(Some(0u64), |acc, i| {
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
        assert_eq!(get_sum_of_calibration_values(TEST_STR), Ok(3749))
    }

    #[test]
    fn check_solvable() {
        assert!("190: 10 19"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
        assert!("3267: 81 40 27"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
        assert!(!"83: 17 5".parse::<Equation<usize>>().unwrap().is_solvable());
        assert!(!"156: 15 6"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
        assert!(!"7290: 6 8 6 15"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
        assert!(!"161011: 16 10 13"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
        assert!(!"192: 17 8 14"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
        assert!(!"21037: 9 7 18 13"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
        assert!("292: 11 6 16 20"
            .parse::<Equation<usize>>()
            .unwrap()
            .is_solvable());
    }

    #[test]
    fn parse() {
        assert_eq!(
            "190: 10 19".parse::<Equation<u8>>(),
            Ok(Equation::<u8> {
                expected: 190u8,
                test_values: vec![10u8, 19u8]
            })
        );
        assert_eq!(
            "3267: 81 40 27".parse::<Equation<u8>>(),
            Err(Error::ParsingFailed)
        );
        assert_eq!(
            "3267: 81 40 27".parse::<Equation<u16>>(),
            Ok(Equation::<u16> {
                expected: 3267u16,
                test_values: vec![81u16, 40u16, 27u16]
            })
        );
        assert_eq!(
            "161011: 16 10 13".parse::<Equation<u8>>(),
            Err(Error::ParsingFailed)
        );
        assert_eq!(
            "161011: 16 10 13".parse::<Equation<u16>>(),
            Err(Error::ParsingFailed)
        );
        assert_eq!(
            "161011: 16 10 13".parse::<Equation<u32>>(),
            Ok(Equation::<u32> {
                expected: 161011u32,
                test_values: vec![16u32, 10u32, 13u32]
            })
        );
    }

    #[test]
    fn generate_bit_vector() {
        assert_eq!(get_bit_vector(1, 4), Ok(vec![true, false, false, false]));
        assert_eq!(get_bit_vector(5, 4), Ok(vec![true, false, true, false]));
        assert_eq!(get_bit_vector(5, 2), Err(Error::BitFieldGeneration));
    }
}
