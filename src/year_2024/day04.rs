#[derive(Debug, PartialEq)]
pub enum Error {
    IntConversion(std::num::TryFromIntError),
    IntOverflow,
    MatrixIteration,
    MalformedMatrix,
    PatternTooLarge,
    Threading,
    Multiple(Vec<Error>),
}

impl std::convert::From<std::num::TryFromIntError> for Error {
    fn from(value: std::num::TryFromIntError) -> Self {
        Error::IntConversion(value)
    }
}

impl std::convert::From<std::boxed::Box<dyn std::any::Any + std::marker::Send>> for Error {
    fn from(value: std::boxed::Box<dyn std::any::Any + std::marker::Send>) -> Self {
        Error::Threading
    }
}

fn validate_pattern_matrix_combination(pattern: &str, matrix: &[&str]) -> Result<(), Error> {
    match matrix.first().map(|x| x.len()) {
        Some(len) => {
            if pattern.len() > len || pattern.len() > matrix.len() {
                Err(Error::PatternTooLarge)
            } else {
                Ok(())
            }
        }
        None => Err(Error::MalformedMatrix),
    }
}

fn count_horizontal_matches(pattern: &str, matrix: &[&str]) -> Result<u32, Error> {
    validate_pattern_matrix_combination(pattern, matrix)?;
    let mut count: u32 = 0u32;
    for row in matrix {
        count += u32::try_from(row.match_indices(pattern).count())?;
    }
    Ok(count)
}

fn count_vertical_matches(pattern: &str, matrix: &[&str]) -> Result<u32, Error> {
    validate_pattern_matrix_combination(pattern, matrix)?;
    let mut count: u32 = 0u32;
    for i in 0..(matrix.len() - pattern.len() + 1) {
        for j in 0..(matrix.first().ok_or_else(|| Error::MatrixIteration)?.len()) {
            let mut not_matched: bool = false;
            for k in 0..pattern.len() {
                if pattern[k..k + 1] != matrix[i + k][j..j + 1] {
                    not_matched = true;
                    break;
                }
            }
            if !not_matched {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn count_diagonal_downward(pattern: &str, matrix: &[&str]) -> Result<u32, Error> {
    validate_pattern_matrix_combination(pattern, matrix)?;
    let mut count: u32 = 0u32;
    for i in 0..(matrix.len() - pattern.len() + 1) {
        for j in
            0..(matrix.first().ok_or_else(|| Error::MatrixIteration)?.len() - pattern.len() + 1)
        {
            let mut not_matched: bool = false;
            for k in 0..pattern.len() {
                if pattern[k..k + 1] != matrix[i + k][j + k..j + k + 1] {
                    not_matched = true;
                    break;
                }
            }
            if !not_matched {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn count_diagonal_upward(pattern: &str, matrix: &[&str]) -> Result<u32, Error> {
    validate_pattern_matrix_combination(pattern, matrix)?;
    let mut count: u32 = 0u32;
    for i in 0..(matrix.len() - pattern.len() + 1) {
        for j in
            0..(matrix.first().ok_or_else(|| Error::MatrixIteration)?.len() - pattern.len() + 1)
        {
            let mut not_matched: bool = false;
            for k in 0..pattern.len() {
                if pattern[k..k + 1] != matrix[i + pattern.len() - (k + 1)][j + k..j + k + 1] {
                    not_matched = true;
                    break;
                }
            }
            if !not_matched {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn convert_string_into_matrix(input: &str) -> Result<Vec<&str>, Error> {
    let v: Vec<&str> = input.lines().collect();
    match v.first().map(|x| x.len()) {
        Some(l) => {
            if v.iter().all(|row| row.len() == l) {
                Ok(v)
            } else {
                Err(Error::MalformedMatrix)
            }
        }
        None => Err(Error::MalformedMatrix),
    }
}

pub fn get_xmas_count(input: &str) -> Result<u32, Error> {
    let input: std::sync::Arc<String> = std::sync::Arc::new(String::from(input));
    let pattern_forward: std::sync::Arc<String> = std::sync::Arc::new(String::from("XMAS"));
    let pattern_forward_clone1 = pattern_forward.clone();
    let pattern_forward_clone2 = pattern_forward.clone();
    let pattern_forward_clone3 = pattern_forward.clone();
    let pattern_backward: std::sync::Arc<String> =
        std::sync::Arc::new(pattern_forward.chars().rev().collect::<String>());
    let pattern_backward_clone1 = pattern_backward.clone();
    let pattern_backward_clone2 = pattern_backward.clone();
    let pattern_backward_clone3 = pattern_backward.clone();
    let input_clone1 = input.clone();
    let input_clone2 = input.clone();
    let input_clone3 = input.clone();
    let input_clone4 = input.clone();
    let input_clone5 = input.clone();
    let input_clone6 = input.clone();
    let input_clone7 = input.clone();
    let threads = vec![
        std::thread::spawn(move || {
            count_horizontal_matches(
                &pattern_forward_clone1,
                &convert_string_into_matrix(&input_clone1)?,
            )
        }),
        std::thread::spawn(move || {
            count_horizontal_matches(
                &pattern_backward_clone1,
                &convert_string_into_matrix(&input_clone2)?,
            )
        }),
        std::thread::spawn(move || {
            count_vertical_matches(
                &pattern_forward_clone2,
                &convert_string_into_matrix(&input_clone3)?,
            )
        }),
        std::thread::spawn(move || {
            count_vertical_matches(
                &pattern_backward_clone2,
                &convert_string_into_matrix(&input_clone4)?,
            )
        }),
        std::thread::spawn(move || {
            count_diagonal_upward(
                &pattern_forward_clone3,
                &convert_string_into_matrix(&input_clone5)?,
            )
        }),
        std::thread::spawn(move || {
            count_diagonal_upward(
                &pattern_backward_clone3,
                &convert_string_into_matrix(&input_clone6)?,
            )
        }),
        std::thread::spawn(move || {
            count_diagonal_downward(
                &pattern_forward,
                &convert_string_into_matrix(&input_clone7)?,
            )
        }),
    ];

    let result = count_diagonal_downward(&pattern_backward, &convert_string_into_matrix(&input)?);

    let mut results = vec![result];

    for handle in threads {
        results.push(handle.join()?);
    }

    results
        .iter()
        .fold(Some(0u32), |acc: Option<u32>, res| match acc { Some(val) => val.checked_add(*res.as_ref().unwrap()), None=> None }).ok_or_else(||Error::IntOverflow)
    // NOTE: can't get this to work rightt now because Error does not implement Copy trait
    // results.iter().fold(Ok(0u32), |acc, res| match (acc, *res) {
    //     (Ok(x), Ok(y)) => x.checked_add(y).ok_or_else(|| Error::IntOverflow),
    //     (Err(x), Ok(_)) => Err(x),
    //     (Ok(_), Err(y)) => Err(y),
    //     (Err(Error::Multiple(v)), Err(Error::Multiple(u))) => {let mut err_vector: Vec<Error> = vec![]; err_vector.extend(v); err_vector.extend(u); Err(Error::Multiple(err_vector))},
    //     (Err(Error::Multiple(v)), Err(y)) => {let mut err_vector: Vec<Error> = vec![]; err_vector.extend(v); err_vector.push(y); Err(Error::Multiple(err_vector))},
    //     (Err(x), Err(Error::Multiple(u))) => {let mut err_vector: Vec<Error> = vec![]; err_vector.push(x); err_vector.extend(u); Err(Error::Multiple(err_vector))},
    // (Err(x), Err(y)) => Err(Error::Multiple(vec![x, y]))
    // })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn search_xmas() {
        assert_eq!(get_xmas_count(TEST_STR), Ok(18));
    }

    #[test]
    fn horizontal_search() {
        let x: Vec<&str> = TEST_STR.split('\n').collect();
        let pattern: &str = "XMAS";
        assert_eq!(count_horizontal_matches(pattern, &x), Ok(3));
        assert_eq!(
            count_horizontal_matches(&pattern.chars().rev().collect::<String>(), &x),
            Ok(2)
        );
    }

    #[test]
    fn vertical_search() {
        let x: Vec<&str> = TEST_STR.split('\n').collect();
        let pattern: &str = "XMAS";
        assert_eq!(count_vertical_matches(pattern, &x), Ok(1));
        assert_eq!(
            count_vertical_matches(&pattern.chars().rev().collect::<String>(), &x),
            Ok(2)
        );
    }

    #[test]
    fn diagonal_downward_search() {
        let x: Vec<&str> = TEST_STR.split('\n').collect();
        let pattern: &str = "XMAS";
        assert_eq!(count_diagonal_downward(pattern, &x), Ok(1));
        assert_eq!(
            count_diagonal_downward(&pattern.chars().rev().collect::<String>(), &x),
            Ok(4)
        );
    }

    #[test]
    fn diagonal_upward_search() {
        let x: Vec<&str> = TEST_STR.split('\n').collect();
        let pattern: &str = "XMAS";
        assert_eq!(count_diagonal_upward(pattern, &x), Ok(4));
        assert_eq!(
            count_diagonal_upward(&pattern.chars().rev().collect::<String>(), &x),
            Ok(1)
        );
    }
}
