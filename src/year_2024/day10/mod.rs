mod error;
mod map;

use error::Error;

pub fn sum_trailhead_scores(input: &str) -> Result<u32, Error> {
    Err(Error::ParsingFailed)
}


#[cfg(test)]
mod tests {
    use super::sum_trailhead_scores;

    pub(super) const TEST_STR: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn sum_scores() {
        assert_eq!(sum_trailhead_scores(TEST_STR), Ok(36));
    }
}
