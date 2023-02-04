use std::fmt;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct ParseRockPaperScissorsError<'a> {
    line: &'a str,
}

impl fmt::Display for ParseRockPaperScissorsError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse line '{}'", self.line)
    }
}

#[derive(Debug, PartialEq)]
pub struct PlayerScore {
    pub left: i32,
    pub right: i32,
}

impl AddAssign for PlayerScore {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            left: self.left + other.left,
            right: self.right + other.right,
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RockPaperScissorsChoice {
    Rock,
    Paper,
    Scissors,
}

pub fn get_rock_paper_scissors_score1(
    input: &str,
) -> Result<PlayerScore, ParseRockPaperScissorsError> {
    let mut result: PlayerScore = PlayerScore { left: 0, right: 0 };
    for line in input.lines() {
        let (left_choice, right_choice) = parse_choices1(line)?;
        result += get_score(&left_choice, &right_choice);
    }
    Ok(result)
}

pub fn get_rock_paper_scissors_score2(
    input: &str,
) -> Result<PlayerScore, ParseRockPaperScissorsError> {
    let mut result: PlayerScore = PlayerScore { left: 0, right: 0 };
    for line in input.lines() {
        let (left_choice, right_choice) = parse_choices2(line)?;
        result += get_score(&left_choice, &right_choice);
    }
    Ok(result)
}

fn parse_choices1(
    line: &str,
) -> Result<(RockPaperScissorsChoice, RockPaperScissorsChoice), ParseRockPaperScissorsError> {
    let mut iterator = line.split_whitespace();
    let left: RockPaperScissorsChoice = match iterator.next() {
        None => return Err(ParseRockPaperScissorsError { line: line }),
        Some(string) => {
            if string.len() != 1 {
                return Err(ParseRockPaperScissorsError { line: line });
            }
            match string.chars().nth(0) {
                Some('A') => RockPaperScissorsChoice::Rock,
                Some('B') => RockPaperScissorsChoice::Paper,
                Some('C') => RockPaperScissorsChoice::Scissors,
                _ => return Err(ParseRockPaperScissorsError { line: line }),
            }
        }
    };
    let right: RockPaperScissorsChoice = match iterator.next() {
        None => return Err(ParseRockPaperScissorsError { line: line }),
        Some(string) => {
            if string.len() != 1 {
                return Err(ParseRockPaperScissorsError { line: line });
            }
            match string.chars().nth(0) {
                Some('X') => RockPaperScissorsChoice::Rock,
                Some('Y') => RockPaperScissorsChoice::Paper,
                Some('Z') => RockPaperScissorsChoice::Scissors,
                _ => return Err(ParseRockPaperScissorsError { line: line }),
            }
        }
    };
    Ok((left, right))
}

fn parse_choices2(
    line: &str,
) -> Result<(RockPaperScissorsChoice, RockPaperScissorsChoice), ParseRockPaperScissorsError> {
    let mut iterator = line.split_whitespace();
    let left: RockPaperScissorsChoice = match iterator.next() {
        None => return Err(ParseRockPaperScissorsError { line: line }),
        Some(string) => {
            if string.len() != 1 {
                return Err(ParseRockPaperScissorsError { line: line });
            }
            match string.chars().nth(0) {
                Some('A') => RockPaperScissorsChoice::Rock,
                Some('B') => RockPaperScissorsChoice::Paper,
                Some('C') => RockPaperScissorsChoice::Scissors,
                _ => return Err(ParseRockPaperScissorsError { line: line }),
            }
        }
    };
    let right: RockPaperScissorsChoice = match iterator.next() {
        None => return Err(ParseRockPaperScissorsError { line: line }),
        Some(string) => {
            if string.len() != 1 {
                return Err(ParseRockPaperScissorsError { line: line });
            }
            match string.chars().nth(0) {
                Some('X') => match &left {
                    RockPaperScissorsChoice::Rock => RockPaperScissorsChoice::Scissors,
                    RockPaperScissorsChoice::Paper => RockPaperScissorsChoice::Rock,
                    RockPaperScissorsChoice::Scissors => RockPaperScissorsChoice::Paper,
                },
                Some('Y') => left.clone(),
                Some('Z') => match &left {
                    RockPaperScissorsChoice::Rock => RockPaperScissorsChoice::Paper,
                    RockPaperScissorsChoice::Paper => RockPaperScissorsChoice::Scissors,
                    RockPaperScissorsChoice::Scissors => RockPaperScissorsChoice::Rock,
                },
                _ => return Err(ParseRockPaperScissorsError { line: line }),
            }
        }
    };
    Ok((left, right))
}

fn get_score(left: &RockPaperScissorsChoice, right: &RockPaperScissorsChoice) -> PlayerScore {
    let mut left_score: i32 = get_shape_score(left);
    let mut right_score: i32 = get_shape_score(right);
    match *left {
        RockPaperScissorsChoice::Rock => match *right {
            RockPaperScissorsChoice::Rock => {
                left_score += 3;
                right_score += 3;
            }
            RockPaperScissorsChoice::Paper => right_score += 6,
            RockPaperScissorsChoice::Scissors => left_score += 6,
        },
        RockPaperScissorsChoice::Paper => match *right {
            RockPaperScissorsChoice::Rock => left_score += 6,
            RockPaperScissorsChoice::Paper => {
                left_score += 3;
                right_score += 3;
            }
            RockPaperScissorsChoice::Scissors => right_score += 6,
        },
        RockPaperScissorsChoice::Scissors => match *right {
            RockPaperScissorsChoice::Rock => right_score += 6,
            RockPaperScissorsChoice::Paper => left_score += 6,
            RockPaperScissorsChoice::Scissors => {
                left_score += 3;
                right_score += 3;
            }
        },
    }
    PlayerScore {
        left: left_score,
        right: right_score,
    }
}

fn get_shape_score(shape: &RockPaperScissorsChoice) -> i32 {
    match *shape {
        RockPaperScissorsChoice::Rock => 1,
        RockPaperScissorsChoice::Paper => 2,
        RockPaperScissorsChoice::Scissors => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "A Y
        B X
        C Z";

    const FAULTY_STR1: &str = "AY
        B X
        C Z";

    #[test]
    fn correct_sum1() {
        assert_eq!(get_rock_paper_scissors_score1(TEST_STR).unwrap().right, 15);
    }

    #[test]
    fn correct_sum2() {
        assert_eq!(get_rock_paper_scissors_score2(TEST_STR).unwrap().right, 12);
    }

    #[test]
    fn parse_error1() {
        let error = match get_rock_paper_scissors_score1(FAULTY_STR1) {
            Ok(_) => {
                assert!(false, "this method should return a parsing error");
                return;
            }
            Err(e) => e,
        };
        assert_eq!(error.line, "AY");
    }

    #[test]
    fn parse_error2() {
        let error = match get_rock_paper_scissors_score2(FAULTY_STR1) {
            Ok(_) => {
                assert!(false, "this method should return a parsing error");
                return;
            }
            Err(e) => e,
        };
        assert_eq!(error.line, "AY");
    }

    #[test]
    fn correct_parse1() {
        assert_eq!(
            parse_choices1("A X").unwrap(),
            (
                RockPaperScissorsChoice::Rock,
                RockPaperScissorsChoice::Rock
            )
        );
        assert_eq!(
            parse_choices1("A Y").unwrap(),
            (RockPaperScissorsChoice::Rock, RockPaperScissorsChoice::Paper)
        );
        assert_eq!(
            parse_choices1("A Z").unwrap(),
            (
                RockPaperScissorsChoice::Rock,
                RockPaperScissorsChoice::Scissors
            )
        );
        assert_eq!(
            parse_choices1("B X").unwrap(),
            (
                RockPaperScissorsChoice::Paper,
                RockPaperScissorsChoice::Rock
            )
        );
        assert_eq!(
            parse_choices1("B Y").unwrap(),
            (
                RockPaperScissorsChoice::Paper,
                RockPaperScissorsChoice::Paper
            )
        );
        assert_eq!(
            parse_choices1("B Z").unwrap(),
            (
                RockPaperScissorsChoice::Paper,
                RockPaperScissorsChoice::Scissors
            )
        );
        assert_eq!(
            parse_choices1("C X").unwrap(),
            (
                RockPaperScissorsChoice::Scissors,
                RockPaperScissorsChoice::Rock
            )
        );
        assert_eq!(
            parse_choices1("C Y").unwrap(),
            (
                RockPaperScissorsChoice::Scissors,
                RockPaperScissorsChoice::Paper
            )
        );
        assert_eq!(
            parse_choices1("C Z").unwrap(),
            (
                RockPaperScissorsChoice::Scissors,
                RockPaperScissorsChoice::Scissors
            )
        );
    }

    #[test]
    fn correct_parse2() {
        assert_eq!(
            parse_choices2("A X").unwrap(),
            (
                RockPaperScissorsChoice::Rock,
                RockPaperScissorsChoice::Scissors
            )
        );
        assert_eq!(
            parse_choices2("A Y").unwrap(),
            (RockPaperScissorsChoice::Rock, RockPaperScissorsChoice::Rock)
        );
        assert_eq!(
            parse_choices2("A Z").unwrap(),
            (
                RockPaperScissorsChoice::Rock,
                RockPaperScissorsChoice::Paper
            )
        );
        assert_eq!(
            parse_choices2("B X").unwrap(),
            (
                RockPaperScissorsChoice::Paper,
                RockPaperScissorsChoice::Rock
            )
        );
        assert_eq!(
            parse_choices2("B Y").unwrap(),
            (
                RockPaperScissorsChoice::Paper,
                RockPaperScissorsChoice::Paper
            )
        );
        assert_eq!(
            parse_choices2("B Z").unwrap(),
            (
                RockPaperScissorsChoice::Paper,
                RockPaperScissorsChoice::Scissors
            )
        );
        assert_eq!(
            parse_choices2("C X").unwrap(),
            (
                RockPaperScissorsChoice::Scissors,
                RockPaperScissorsChoice::Paper
            )
        );
        assert_eq!(
            parse_choices2("C Y").unwrap(),
            (
                RockPaperScissorsChoice::Scissors,
                RockPaperScissorsChoice::Scissors
            )
        );
        assert_eq!(
            parse_choices2("C Z").unwrap(),
            (
                RockPaperScissorsChoice::Scissors,
                RockPaperScissorsChoice::Rock
            )
        );
    }

    #[test]
    fn check_outcomes() {
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Rock,
                &RockPaperScissorsChoice::Rock
            ),
            PlayerScore { left: 4, right: 4 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Rock,
                &RockPaperScissorsChoice::Paper
            ),
            PlayerScore { left: 1, right: 8 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Rock,
                &RockPaperScissorsChoice::Scissors
            ),
            PlayerScore { left: 7, right: 3 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Paper,
                &RockPaperScissorsChoice::Rock
            ),
            PlayerScore { left: 8, right: 1 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Paper,
                &RockPaperScissorsChoice::Paper
            ),
            PlayerScore { left: 5, right: 5 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Paper,
                &RockPaperScissorsChoice::Scissors
            ),
            PlayerScore { left: 2, right: 9 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Scissors,
                &RockPaperScissorsChoice::Rock
            ),
            PlayerScore { left: 3, right: 7 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Scissors,
                &RockPaperScissorsChoice::Paper
            ),
            PlayerScore { left: 9, right: 2 }
        );
        assert_eq!(
            get_score(
                &RockPaperScissorsChoice::Scissors,
                &RockPaperScissorsChoice::Scissors
            ),
            PlayerScore { left: 6, right: 6 }
        );
    }
}
