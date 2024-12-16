pub mod error;

mod antenna;
mod antinode;
mod map;
mod position;
mod position_diff;
mod signed_diff;
mod size;

pub fn count_distinct_antinode_positions(input: &str) -> Result<usize, error::Error> {
    let map = input.parse::<map::Map>()?;
    Ok(map
        .get_antinodes()
        .iter()
        .map(|a| a.position)
        .collect::<std::collections::HashSet<_>>()
        .len())
}

#[cfg(test)]
mod tests {
    use super::count_distinct_antinode_positions;

    pub const TEST_STR: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn get_antinode_position_count() {
        assert_eq!(count_distinct_antinode_positions(TEST_STR), Ok(14));
    }
}
