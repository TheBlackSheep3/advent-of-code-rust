pub mod error;

mod disk_map;
mod file;

use error::Error;

pub fn get_compacted_filesystem_checksum(input: &str) -> Result<usize, Error> {
    let mut map: disk_map::DiskMap = input.parse()?;
    map.rearrange();
    map.get_check_sum()
}

#[cfg(test)]
mod tests {
    use super::get_compacted_filesystem_checksum;

    pub const SMALL_SAMPLE1: &str = "12345";
    pub const SMALL_SAMPLE2: &str = "90909";
    pub const LARGER_SAMPLE: &str = "2333133121414131402";

    #[test]
    fn compacted_checksum() {
        let input_checksum_pairs: Vec<(&str, usize)> = vec![
            (
                SMALL_SAMPLE1,
                0 * 0 + 1 * 2 + 2 * 2 + 3 * 1 + 4 * 1 + 5 * 1 + 6 * 2 + 7 * 2 + 8 * 2,
            ),
            (
                SMALL_SAMPLE2,
                0 * 0
                    + 1 * 0
                    + 2 * 0
                    + 3 * 0
                    + 4 * 0
                    + 5 * 0
                    + 6 * 0
                    + 7 * 0
                    + 8 * 0
                    + 9 * 1
                    + 10 * 1
                    + 11 * 1
                    + 12 * 1
                    + 13 * 1
                    + 14 * 1
                    + 15 * 1
                    + 16 * 1
                    + 17 * 1
                    + 18 * 2
                    + 19 * 2
                    + 20 * 2
                    + 21 * 2
                    + 22 * 2
                    + 23 * 2
                    + 24 * 2
                    + 25 * 2
                    + 26 * 2,
            ),
            (LARGER_SAMPLE, 1928),
        ];
        for (input, checksum) in input_checksum_pairs {
            assert_eq!(get_compacted_filesystem_checksum(input), Ok(checksum));
        }
    }
}
