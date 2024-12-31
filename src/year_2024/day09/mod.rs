pub mod error;

mod disk_map;
mod file;

use error::Error;

pub fn get_compacted_filesystem_checksum(input: &str) -> Result<usize, Error> {
    let mut map: disk_map::DiskMap = input.parse()?;
    map.rearrange();
    map.get_check_sum()
}

pub fn get_compacted_filesystem_no_fragmentation_checksum(input: &str) -> Result<usize, Error> {
    let mut map: disk_map::DiskMap = input.parse()?;
    map.rearrange_no_fragmentation();
    map.get_check_sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::get_compacted_filesystem_checksum;
    use super::get_compacted_filesystem_no_fragmentation_checksum;

    pub const SMALL_SAMPLE1: &str = "12345";
    pub const SMALL_SAMPLE2: &str = "90909";
    pub const LARGER_SAMPLE: &str = "2333133121414131402";

    #[rstest]
    #[case(SMALL_SAMPLE1, Ok(60))]
    #[case(SMALL_SAMPLE2, Ok(513))]
    #[case(LARGER_SAMPLE, Ok(1928))]
    fn compacted_checksum(#[case] input: &str, #[case] expected: Result<usize, super::Error>) {
        assert_eq!(expected, get_compacted_filesystem_checksum(input));
    }

    #[test]
    fn compacted_no_fragmentation_checksum() {
        assert_eq!(
            get_compacted_filesystem_no_fragmentation_checksum(LARGER_SAMPLE),
            Ok(2858)
        );
    }
}
