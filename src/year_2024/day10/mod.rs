mod error;
mod map;

use error::Error;
use map::Map;

pub fn sum_trailhead_scores(input: &str) -> Result<u32, Error> {
    let map = input.parse::<Map>()?;
    let mut score_sum = 0u32;
    for head in map.get_trail_heads() {
        score_sum = score_sum
            .checked_add(
                map.get_trail_ends(head)
                    .len()
                    .try_into()
                    .map_err(|_| Error::IntegerConversionFailed)?,
            )
            .ok_or(Error::IntegerOverflow)?
    }
    Ok(score_sum)
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
